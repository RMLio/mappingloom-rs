//! Normalizes a mapping document with SPARQL Update rules over an Oxigraph
//! store, one rule set per language.
//!
//! [`normalize_kgc`] applies the RML-KGC rule set of [`kgc::RULES`].
//! [`normalize_rml_str`] and [`normalize_rml_path`] apply the RML.io rule
//! set of the paper (Min & Hartig, Appendix A).

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;

use error::{NormalizerError, NormalizerResult};
use oxigraph::io::{RdfFormat, RdfParseError, RdfParser, RdfSerializer};
use oxigraph::model::dataset::{
    CanonicalizationAlgorithm, CanonicalizationHashAlgorithm,
};
use oxigraph::model::{
    BlankNode, Dataset, Graph, GraphName, GraphNameRef, LiteralRef, NamedNode,
    NamedNodeRef, Quad, Term, Triple, TripleRef,
};
use oxigraph::sparql::SparqlEvaluator;
use oxigraph::store::Store;
use queries::QUERY_MSG_PAIRS;

/// The namespace of the links that the KGC rules add to the normal form.
macro_rules! nf {
    () => {
        "https://w3id.org/imec/knows/ns/rml/normal-form#"
    };
}

mod error;
pub mod kgc;
mod queries;

pub const NF: &str = nf!();

/// The namespace of the skolem IRIs of the original document's blank nodes.
pub const GENID: &str =
    "https://github.com/RMLio/algemaploom-rs/.well-known/genid/";

/// A normalization rule.
pub struct Rule {
    pub id: &'static str,
    pub description: &'static str,
    pub action: Action,
}

pub enum Action {
    /// A SPARQL Update request over the mapping graph.
    Update(&'static str),
    /// A step that SPARQL can't express, such as a recursive comparison.
    Native(fn(&Store) -> NormalizerResult<()>),
}

/// A normalized mapping: the normal form, which shares its blank nodes with
/// the original document, and the skolem IRI of each blank node of the
/// original document, which `nf:origin` links name.
pub struct NormalForm {
    pub normalized: Graph,
    pub skolem: HashMap<BlankNode, NamedNode>,
}

/// Normalizes an RML-KGC mapping in Turtle. `base_iri` is the processor's
/// base IRI, which every triples map without `rml:baseIRI` takes.
pub fn normalize_kgc(
    mapping: &str,
    base_iri: &str,
) -> NormalizerResult<NormalForm> {
    apply(mapping, base_iri, kgc::RULES)
}

/// Applies `rules` in order to a mapping in Turtle.
pub fn apply(
    mapping: &str,
    base_iri: &str,
    rules: &[Rule],
) -> NormalizerResult<NormalForm> {
    let original: Graph = RdfParser::from_format(RdfFormat::Turtle)
        .for_slice(mapping)
        .map(|quad| quad.map(Triple::from).map_err(RdfParseError::from))
        .collect::<Result<_, _>>()?;
    let processor = Quad::new(
        NamedNodeRef::new_unchecked(concat!(nf!(), "processor")),
        NamedNodeRef::new_unchecked("http://w3id.org/rml/baseIRI"),
        NamedNode::new(base_iri)?,
        GraphName::DefaultGraph,
    );
    let store = Store::new()?;
    store.extend(
        original
            .iter()
            .map(|t| t.into_owned().in_graph(GraphName::DefaultGraph)),
    )?;
    store.insert(&processor)?;
    for rule in rules {
        log::debug!("Rule {}: {}", rule.id, rule.description);
        match rule.action {
            Action::Update(update) => run_update(&store, update)?,
            Action::Native(step) => step(&store)?,
        }
    }
    store.remove(&processor)?;
    let skolem = skolem_iris(&original);
    let origin = NamedNodeRef::new_unchecked(concat!(nf!(), "origin"));
    let label = NamedNodeRef::new_unchecked(concat!(nf!(), "label"));
    let mut normalized = Graph::new();
    for quad in store.iter() {
        let mut triple = Triple::from(quad?);
        if let Term::BlankNode(node) = &triple.object {
            if let Some(iri) =
                skolem.get(node).filter(|_| triple.predicate == origin)
            {
                normalized.insert(TripleRef::new(
                    iri,
                    label,
                    LiteralRef::new_simple_literal(node.as_str()),
                ));
                triple.object = iri.clone().into();
            }
        }
        normalized.insert(&triple);
    }
    Ok(NormalForm { normalized, skolem })
}

fn run_update(store: &Store, update: &str) -> NormalizerResult<()> {
    Ok(SparqlEvaluator::new()
        .parse_update(update)?
        .on_store(store)
        .execute()?)
}

/// The skolem IRI of each blank node, from its canonical label (RDFC-1.0),
/// so it depends on the graph alone.
fn skolem_iris(graph: &Graph) -> HashMap<BlankNode, NamedNode> {
    let dataset: Dataset = graph
        .iter()
        .map(|triple| triple.in_graph(GraphNameRef::DefaultGraph))
        .collect();
    let algorithm = CanonicalizationAlgorithm::Rdfc10 {
        hash_algorithm: CanonicalizationHashAlgorithm::Sha256,
    };
    dataset
        .canonicalize_blank_nodes(algorithm)
        .into_iter()
        .map(|(node, canonical)| {
            let iri = format!("{GENID}{}", canonical.as_str());
            (node.into_owned(), NamedNode::new_unchecked(iri))
        })
        .collect()
}

pub fn normalize_rml_str(rml_str: &str) -> NormalizerResult<String> {
    let store = Store::new()?;
    let mut loader = store.bulk_loader();
    let parser = RdfParser::from_format(RdfFormat::Turtle);
    let mut reader = parser.for_reader(rml_str.as_bytes());

    loader.load_ok_quads::<RdfParseError, NormalizerError>(&mut reader)?;
    loader.commit()?;

    let prefixes = reader.prefixes().collect::<Vec<_>>();
    log::debug!("Found prefixes of the RML document: {:?}", prefixes);

    for (update, msg) in QUERY_MSG_PAIRS {
        log::info!("{}", msg);
        let update = SparqlEvaluator::new().parse_update(update)?;
        update.on_store(&store).execute()?;
    }

    log::info!("Serializing normalized RDF dataset to Turtle");
    let mut serializer = RdfSerializer::from_format(RdfFormat::Turtle);
    serializer = prefixes.iter().fold(serializer, |acc, pref| {
        acc.with_prefix(pref.0, pref.1).unwrap()
    });

    let mut buf_writer = BufWriter::new(vec![]);
    let mut write_serializer = serializer.for_writer(buf_writer);
    for quad in store.iter().flatten() {
        write_serializer.serialize_triple(&Into::<Triple>::into(quad))?;
    }

    buf_writer = write_serializer.finish()?;

    Ok(String::from_utf8(buf_writer.into_inner().unwrap())?)
}

pub fn normalize_rml_path(rml_path: &Path) -> NormalizerResult<()> {
    log::info!(
        "Normalizing the given RML document at {}",
        rml_path.to_string_lossy()
    );
    let file = File::open(rml_path)?;
    let mut reader = BufReader::new(file);
    let mut buf = vec![];
    reader.read_to_end(&mut buf)?;
    let normalized_rml = normalize_rml_str(String::from_utf8(buf)?.as_str())?;

    let output_file_name = format!(
        "normalized_{}",
        rml_path
            .file_name()
            .expect("Expect a valid file name for the given path")
            .to_string_lossy()
    );
    let output_path = rml_path.with_file_name(output_file_name);
    let output_file = File::create(output_path)?;
    let mut buf_writer = BufWriter::new(output_file);
    buf_writer.write_all(normalized_rml.as_bytes())?;
    Ok(())
}
