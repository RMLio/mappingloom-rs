use std::collections::{HashMap, HashSet};

use normalizer::NormalForm;
use oxigraph::model::{
    BlankNode, Graph, NamedNode, NamedNodeRef, NamedOrBlankNodeRef, TermRef,
};

use crate::vocab;

/// The normal form of a mapping, read by subject and predicate.
pub struct Mapping {
    graph: Graph,
    skolem: HashMap<BlankNode, NamedNode>,
}

impl Mapping {
    pub fn new(normal_form: NormalForm) -> Self {
        Self {
            graph: normal_form.normalized,
            skolem: normal_form.skolem,
        }
    }

    /// The normalized triples maps: the nodes with a predicate-object map.
    /// A triples map without one produces no triples.
    pub fn triples_maps(&self) -> Vec<NamedOrBlankNodeRef<'_>> {
        let mut seen = HashSet::new();
        self.graph
            .triples_for_predicate(vocab::PREDICATE_OBJECT_MAP)
            .map(|triple| triple.subject)
            .filter(|triples_map| seen.insert(*triples_map))
            .collect()
    }

    /// The objects of `subject` for `predicate`.
    pub fn objects<'a>(
        &'a self,
        subject: NamedOrBlankNodeRef<'a>,
        predicate: NamedNodeRef<'a>,
    ) -> Vec<TermRef<'a>> {
        self.graph
            .objects_for_subject_predicate(subject, predicate)
            .collect()
    }

    /// The object of `subject` for `predicate`, if it has one.
    pub fn object<'a>(
        &'a self,
        subject: NamedOrBlankNodeRef<'a>,
        predicate: NamedNodeRef<'a>,
    ) -> Option<TermRef<'a>> {
        self.graph.object_for_subject_predicate(subject, predicate)
    }

    /// The object of `subject` for `predicate` as a node.
    pub fn node<'a>(
        &'a self,
        subject: NamedOrBlankNodeRef<'a>,
        predicate: NamedNodeRef<'a>,
    ) -> Option<NamedOrBlankNodeRef<'a>> {
        self.object(subject, predicate).and_then(as_node)
    }

    /// The predicates of `subject`.
    pub fn predicates<'a>(
        &'a self,
        subject: NamedOrBlankNodeRef<'a>,
    ) -> Vec<NamedNodeRef<'a>> {
        self.graph
            .triples_for_subject(subject)
            .map(|triple| triple.predicate)
            .collect()
    }

    /// The node of the original mapping document that `node` stands for: the
    /// node itself, its `nf:origin` if a rule created it, and a skolem IRI
    /// for a blank node.
    pub fn origin(&self, node: NamedOrBlankNodeRef<'_>) -> String {
        match self.node(node, vocab::NF_ORIGIN) {
            Some(origin) => self.origin(origin),
            None => match node {
                NamedOrBlankNodeRef::NamedNode(iri) => iri.as_str().into(),
                NamedOrBlankNodeRef::BlankNode(blank) => {
                    self.skolem.get(&blank.into_owned()).map_or_else(
                        || blank.to_string(),
                        |iri| iri.as_str().into(),
                    )
                }
            },
        }
    }
}

/// A term as a node, if it is an IRI or a blank node.
pub fn as_node(term: TermRef<'_>) -> Option<NamedOrBlankNodeRef<'_>> {
    match term {
        TermRef::NamedNode(iri) => Some(iri.into()),
        TermRef::BlankNode(blank) => Some(blank.into()),
        _ => None,
    }
}

/// The value of a literal.
pub fn string(term: Option<TermRef<'_>>) -> Option<String> {
    match term {
        Some(TermRef::Literal(literal)) => Some(literal.value().into()),
        _ => None,
    }
}
