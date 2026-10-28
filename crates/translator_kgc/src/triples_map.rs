//! Algorithm 1: the branch of one normalized triples map.

use algemaploom_plan::{
    ExtendOp, ExtendPair, NodeIndex, Operator, Plan, Position, ProjectOp,
    QuadPattern, SerializerOp, SourceOp,
};
use oxigraph::model::NamedOrBlankNodeRef;

use crate::mapping::{as_node, Mapping};
use crate::term_map::{base_iri, expression, Context, Role};
use crate::{vocab, Error, ErrorKind};

/// The operators of one normalized triples map, before they join the plan.
#[derive(Debug)]
pub struct Branch {
    source: SourceOp,
    source_origin: Vec<String>,
    extend: ExtendOp,
    serializer: SerializerOp,
    origin: Vec<String>,
}

impl Branch {
    /// Adds the branch to the plan, and returns its Serializer.
    pub fn add_to(self, plan: &mut Plan) -> NodeIndex {
        let source =
            plan.add(Operator::SourceOp(self.source), self.source_origin);
        // Algorithm 1 projects onto the attributes of the quad pattern.
        let attributes = self
            .extend
            .pairs
            .iter()
            .map(|pair| pair.attribute.clone())
            .collect();
        let extend = plan.add(Operator::ExtendOp(self.extend), vec![]);
        let project =
            plan.add(Operator::ProjectOp(ProjectOp { attributes }), vec![]);
        let serializer =
            plan.add(Operator::SerializerOp(self.serializer), self.origin);
        plan.connect(source, extend);
        plan.connect(extend, project);
        plan.connect(project, serializer);
        serializer
    }
}

/// Translates a normalized triples map: one subject map and one
/// predicate-object map with one predicate map, one object map and at most
/// one graph map. Every error of its term maps and its source is reported.
pub fn translate(
    mapping: &Mapping,
    triples_map: NamedOrBlankNodeRef<'_>,
) -> Result<Branch, Vec<Error>> {
    let tm_origin = mapping.origin(triples_map);
    let mapping_error = |node, code, message: String| {
        vec![Error::new(ErrorKind::Mapping, code, message)
            .at(mapping.origin(node))]
    };
    let subject_maps = mapping.objects(triples_map, vocab::SUBJECT_MAP);
    let subject_map = match subject_maps.as_slice() {
        [subject_map] => as_node(*subject_map).ok_or_else(|| {
            mapping_error(
                triples_map,
                "subject-maps",
                "a subject map is a node".into(),
            )
        })?,
        _ => {
            return Err(mapping_error(
                triples_map,
                "subject-maps",
                format!(
                    "a triples map has one subject map, not {}",
                    subject_maps.len()
                ),
            ))
        }
    };
    // The normalizer splits each predicate-object map with a predicate map
    // and an object map until each of them is single.
    let pom = mapping.node(triples_map, vocab::PREDICATE_OBJECT_MAP);
    let (pom, predicate_map, object_map) = pom
        .and_then(|pom| {
            Some((
                pom,
                mapping.node(pom, vocab::PREDICATE_MAP)?,
                mapping.node(pom, vocab::OBJECT_MAP)?,
            ))
        })
        .ok_or_else(|| {
            mapping_error(
                pom.unwrap_or(triples_map),
                "predicate-object-map",
                "a predicate-object map has a predicate map and an object map"
                    .into(),
            )
        })?;
    // A constant default graph is the absent graph of a quad pattern.
    let graph_map = mapping.node(pom, vocab::GRAPH_MAP).filter(|map| {
        mapping.object(*map, vocab::CONSTANT)
            != Some(vocab::DEFAULT_GRAPH.into())
    });
    let mut maps = vec![
        ("subject", Role::Subject, subject_map),
        ("predicate", Role::Predicate, predicate_map),
        ("object", Role::Object, object_map),
    ];
    if let Some(graph_map) = graph_map {
        maps.push(("graph", Role::Graph, graph_map));
    }

    let mut context = Context {
        base_iri: base_iri(mapping, triples_map),
        queries: Default::default(),
    };
    let pom_origin = mapping.origin(pom);
    let mut pairs = Vec::new();
    let mut errors = Vec::new();
    for (attribute, role, map) in &maps {
        match expression(mapping, *map, *role, &mut context) {
            Ok(expression) => {
                let mut origin = vec![tm_origin.clone()];
                if *role != Role::Subject {
                    origin.push(pom_origin.clone());
                }
                origin.push(mapping.origin(*map));
                // A shortcut's term map has its holder as origin.
                origin.dedup();
                pairs.push(ExtendPair {
                    attribute: attribute.to_string(),
                    expression,
                    origin,
                });
            }
            Err(error) => errors.push(error),
        }
    }
    let (source, source_origin) =
        match crate::source::source(mapping, triples_map, &context.queries) {
            Ok(source) if errors.is_empty() => source,
            Ok(_) => return Err(errors),
            Err(error) => {
                errors.push(error);
                return Err(errors);
            }
        };

    let attribute = |name: &str| Position::Attribute {
        attribute: name.into(),
    };
    let pattern = QuadPattern {
        subject: attribute("subject"),
        predicate: attribute("predicate"),
        object: attribute("object"),
        graph: graph_map.map(|_| attribute("graph")),
        origin: vec![tm_origin.clone(), pom_origin],
    };
    Ok(Branch {
        source,
        source_origin,
        extend: ExtendOp { pairs },
        serializer: SerializerOp {
            patterns: vec![pattern],
        },
        origin: vec![tm_origin],
    })
}
