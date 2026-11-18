//! Algorithm 1: the branch of one normalized triples map.

use algemaploom_plan::{
    ExtendOp, ExtendPair, InputRole, JoinKind, JoinOp, JoinPair, NodeIndex,
    Operator, Plan, Position, ProjectOp, QuadPattern, RenameOp, RenamePair,
    SerializerOp, SourceOp,
};
use oxigraph::model::NamedOrBlankNodeRef;

use crate::mapping::{as_node, Mapping};
use crate::term_map::{base_iri, expression, Context, Role};
use crate::{vocab, Error, ErrorKind};

/// The prefix of the attributes of a join's parent side.
const PARENT: &str = "parent:";

/// The operators of one normalized triples map, before they join the plan.
#[derive(Debug)]
pub struct Branch {
    source: SourceOp,
    source_origin: Vec<String>,
    extend: ExtendOp,
    join: Option<Join>,
    serializer: SerializerOp,
    origin: Vec<String>,
}

/// The parent side of a referencing object map: the parent's Source, with
/// its `#` renamed so that both sides' schemas are disjoint, an Extend with
/// the parent keys, the Join, and the Extend of the object from the parent
/// subject map.
#[derive(Debug)]
struct Join {
    source: SourceOp,
    source_origin: Vec<String>,
    rename: RenameOp,
    keys: ExtendOp,
    join: JoinOp,
    object: ExtendOp,
    origin: Vec<String>,
}

impl Branch {
    /// Adds the branch to the plan, and returns its Serializer.
    pub fn add_to(self, plan: &mut Plan) -> NodeIndex {
        let source =
            plan.add(Operator::Source(self.source), self.source_origin);
        let extend = plan.add(Operator::Extend(self.extend), vec![]);
        plan.connect(source, extend);
        let tuples = match self.join {
            None => extend,
            Some(join) => {
                let parent = plan
                    .add(Operator::Source(join.source), join.source_origin);
                let rename = plan.add(Operator::Rename(join.rename), vec![]);
                plan.connect(parent, rename);
                // A join without conditions has no keys to bind.
                let keys = if join.keys.pairs.is_empty() {
                    rename
                } else {
                    let keys = plan.add(Operator::Extend(join.keys), vec![]);
                    plan.connect(rename, keys);
                    keys
                };
                let joined = plan.add(Operator::Join(join.join), join.origin);
                let object = plan.add(Operator::Extend(join.object), vec![]);
                plan.connect_as(extend, joined, InputRole::Left);
                plan.connect_as(keys, joined, InputRole::Right);
                plan.connect(joined, object);
                object
            }
        };
        // Algorithm 1 projects onto the attributes of the quad pattern.
        let pattern = &self.serializer.patterns[0];
        let attributes = [
            Some(&pattern.subject),
            Some(&pattern.predicate),
            Some(&pattern.object),
            pattern.graph.as_ref(),
        ]
        .into_iter()
        .flatten()
        .filter_map(|position| match position {
            Position::Attribute { attribute } => Some(attribute.clone()),
            Position::Constant { .. } => None,
        })
        .collect();
        let project =
            plan.add(Operator::Project(ProjectOp { attributes }), vec![]);
        let serializer =
            plan.add(Operator::Serializer(self.serializer), self.origin);
        plan.connect(tuples, project);
        plan.connect(project, serializer);
        serializer
    }
}

/// The one subject map of a triples map.
pub fn subject_map<'a>(
    mapping: &'a Mapping,
    triples_map: NamedOrBlankNodeRef<'a>,
) -> Result<NamedOrBlankNodeRef<'a>, Error> {
    let error = |message: String| {
        Error::new(ErrorKind::Mapping, "subject-maps", message)
            .at(mapping.origin(triples_map))
    };
    match mapping.objects(triples_map, vocab::SUBJECT_MAP).as_slice() {
        [subject_map] => as_node(*subject_map)
            .ok_or_else(|| error("a subject map is a node".into())),
        subject_maps => Err(error(format!(
            "a triples map has one subject map, not {}",
            subject_maps.len()
        ))),
    }
}

/// Checks the subject map of a triples map without predicate-object maps,
/// which gives no branch, so that a feature it uses, such as a gather map, is
/// reported rather than dropped.
pub fn check_subject_map(
    mapping: &Mapping,
    triples_map: NamedOrBlankNodeRef<'_>,
) -> Result<(), Error> {
    let map = subject_map(mapping, triples_map)?;
    let mut context = Context::new(base_iri(mapping, triples_map), "");
    expression(mapping, map, Role::Subject, &mut context).map(|_| ())
}

/// Translates a normalized triples map: one subject map and one
/// predicate-object map with one predicate map, one object map and at most
/// one graph map. Every error of its term maps and its sources is reported.
pub fn translate(
    mapping: &Mapping,
    triples_map: NamedOrBlankNodeRef<'_>,
) -> Result<Branch, Vec<Error>> {
    let tm_origin = mapping.origin(triples_map);
    let subject_map = subject_map(mapping, triples_map).map_err(|e| vec![e])?;
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
            vec![Error::new(
                ErrorKind::Mapping,
                "predicate-object-map",
                "a predicate-object map has a predicate map and an object map"
                    .into(),
            )
            .at(mapping.origin(pom.unwrap_or(triples_map)))]
        })?;
    // A constant default graph is the absent graph of a quad pattern.
    let graph_map = mapping.node(pom, vocab::GRAPH_MAP).filter(|map| {
        mapping.object(*map, vocab::CONSTANT)
            != Some(vocab::DEFAULT_GRAPH.into())
    });
    let parent = match mapping.object(object_map, vocab::PARENT_TRIPLES_MAP) {
        Some(parent) => Some(as_node(parent).ok_or_else(|| {
            vec![Error::new(
                ErrorKind::Mapping,
                "parent-triples-map",
                "a parent triples map is a node".into(),
            )
            .at(mapping.origin(object_map))]
        })?),
        None => None,
    };
    let mut maps = vec![
        ("subject", Role::Subject, subject_map),
        ("predicate", Role::Predicate, predicate_map),
    ];
    if parent.is_none() {
        maps.push(("object", Role::Object, object_map));
    }
    if let Some(graph_map) = graph_map {
        maps.push(("graph", Role::Graph, graph_map));
    }

    let mut context = Context::new(base_iri(mapping, triples_map), "");
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
    let join = match parent {
        Some(parent) => {
            let object_origin = vec![tm_origin.clone(), pom_origin.clone()];
            match join(mapping, object_map, parent, &mut context, object_origin)
            {
                Ok((join, keys)) => {
                    pairs.extend(keys);
                    Some(join)
                }
                Err(found) => {
                    errors.extend(found);
                    None
                }
            }
        }
        None => None,
    };
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
        join,
        serializer: SerializerOp {
            patterns: vec![pattern],
        },
        origin: vec![tm_origin],
    })
}

/// The parent side of the referencing object map `object_map`,
/// and the child keys, which the child's Extend binds:
/// one pair of keys per join condition,
/// and the object from the parent's subject map,
/// evaluated after the Join with the parent's base IRI (Algorithm 1).
/// A join condition's maps are expression maps of any kind,
/// and the Join compares their values by lexical form.
fn join(
    mapping: &Mapping,
    object_map: NamedOrBlankNodeRef<'_>,
    parent: NamedOrBlankNodeRef<'_>,
    child: &mut Context,
    object_origin: Vec<String>,
) -> Result<(Join, Vec<ExtendPair>), Vec<Error>> {
    let rom_origin = mapping.origin(object_map);
    let unsupported = |node, message| {
        Error::new(ErrorKind::Unsupported, "feature", message)
            .at(mapping.origin(node))
    };
    let mapping_error = |node, message: String| {
        Error::new(ErrorKind::Mapping, "join-condition", message)
            .at(mapping.origin(node))
    };
    let mut errors = Vec::new();
    if let Err(message) = mapping.only_known_properties(
        object_map,
        &[vocab::PARENT_TRIPLES_MAP, vocab::JOIN_CONDITION],
    ) {
        errors.push(unsupported(object_map, message));
    }
    let mut parent_context = Context::new(base_iri(mapping, parent), PARENT);
    let object = subject_map(mapping, parent)
        .map_err(|error| vec![error])
        .and_then(|map| {
            expression(mapping, map, Role::Subject, &mut parent_context)
                .map(|expression| (map, expression))
                .map_err(|error| vec![error])
        });
    // Each join condition as its child and parent expressions, sorted, since
    // the normal form has no order.
    let mut conditions = Vec::new();
    for condition in mapping.objects(object_map, vocab::JOIN_CONDITION) {
        let Some(condition) = as_node(condition) else {
            errors.push(mapping_error(
                object_map,
                "a join condition is a node".into(),
            ));
            continue;
        };
        if let Err(message) = mapping.only_known_properties(
            condition,
            &[vocab::CHILD_MAP, vocab::PARENT_MAP],
        ) {
            errors.push(unsupported(condition, message));
            continue;
        }
        let key = |property, context: &mut Context| match mapping
            .objects(condition, property)
            .as_slice()
        {
            [map] => match as_node(*map) {
                Some(map) => expression(mapping, map, Role::Key, context)
                    .map(|expression| (expression, mapping.origin(map))),
                None => Err(mapping_error(
                    condition,
                    format!("a {property} is a node"),
                )),
            },
            maps => Err(mapping_error(
                condition,
                format!(
                    "a join condition has one {property}, not {}",
                    maps.len()
                ),
            )),
        };
        match (
            key(vocab::CHILD_MAP, child),
            key(vocab::PARENT_MAP, &mut parent_context),
        ) {
            (Ok(child), Ok(parent)) => {
                conditions.push((child, parent, mapping.origin(condition)))
            }
            (child, parent) => {
                errors.extend(child.err());
                errors.extend(parent.err());
            }
        }
    }
    // The parent's source is checked here only, if the parent has no
    // predicate-object map of its own.
    let source =
        crate::source::source(mapping, parent, &parent_context.queries);
    let ((parent_map, object), (source, source_origin)) = match (object, source)
    {
        (Ok(object), Ok(source)) if errors.is_empty() => (object, source),
        (object, source) => {
            errors.extend(object.err().into_iter().flatten());
            errors.extend(source.err());
            return Err(errors);
        }
    };
    conditions.sort_by_cached_key(|condition| format!("{condition:?}"));
    let mut child_keys = Vec::new();
    let mut parent_keys = Vec::new();
    let mut pairs = Vec::new();
    for (i, ((child, child_origin), (parent, parent_origin), origin)) in
        conditions.into_iter().enumerate()
    {
        // A shortcut's map has its join condition as origin.
        let origins = |map_origin: String| {
            let mut origins = vec![origin.clone(), map_origin];
            origins.dedup();
            origins
        };
        let left = format!("key:{i}");
        let right = format!("{PARENT}key:{i}");
        child_keys.push(ExtendPair {
            attribute: left.clone(),
            expression: child,
            origin: origins(child_origin),
        });
        parent_keys.push(ExtendPair {
            attribute: right.clone(),
            expression: parent,
            origin: origins(parent_origin),
        });
        pairs.push(JoinPair { left, right });
    }
    let mut origin = object_origin;
    origin.push(rom_origin.clone());
    origin.push(mapping.origin(parent_map));
    Ok((
        Join {
            source,
            source_origin,
            rename: RenameOp {
                pairs: vec![RenamePair {
                    from: Context::new(None, "").index(),
                    to: parent_context.index(),
                }],
            },
            keys: ExtendOp { pairs: parent_keys },
            join: JoinOp {
                kind: JoinKind::Inner,
                pairs,
            },
            object: ExtendOp {
                pairs: vec![ExtendPair {
                    attribute: "object".into(),
                    expression: object,
                    origin,
                }],
            },
            origin: vec![rom_origin],
        },
        child_keys,
    ))
}
