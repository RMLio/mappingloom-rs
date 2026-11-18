use std::path::Path;

use algemaploom_plan::*;
use pipeline_test::{assert_isomorphic, assert_plan_snapshot, parse_graph};

#[test]
fn isomorphism_ignores_blank_node_labels() {
    let graph = |label| {
        parse_graph(&format!("_:{label} <http://ex/p> [ <http://ex/q> 1 ] ."))
    };
    assert_isomorphic(&graph("a"), &graph("b"));
}

#[test]
#[should_panic(expected = "graphs differ")]
fn isomorphism_tells_graphs_apart() {
    assert_isomorphic(
        &parse_graph("<http://ex/s> <http://ex/p> 1 ."),
        &parse_graph("<http://ex/s> <http://ex/p> 2 ."),
    );
}

/// RMLTC0000-JSON as the canonical translator emits it: every core operator
/// and the term expressions, valid under the schema.
#[test]
fn core_plan_is_valid() {
    let tm = "http://example.com/base/TriplesMap1";
    let origin = |map: &str| vec![tm.into(), map.into()];
    let attribute = |name: &str| Position::Attribute {
        attribute: name.into(),
    };
    let mut plan = Plan::new();
    let source = plan.add(
        Operator::Source(SourceOp {
            access: Access::File {
                path: "student.json".into(),
                root: Root::MappingDirectory,
            },
            query_language: Some("http://w3id.org/rml/JSONPath".into()),
            iterator: Some("$.students[*]".into()),
            null_values: vec![],
            fields: vec![Field {
                attribute: "name".into(),
                reference: "$.Name".into(),
                single_valued: true,
            }],
        }),
        vec!["http://example.com/base/ls".into()],
    );
    let extend = plan.add(
        Operator::Extend(ExtendOp {
            pairs: vec![
                ExtendPair {
                    attribute: "s".into(),
                    expression: Expression::Iri {
                        inner: Box::new(Expression::Template {
                            pieces: vec![
                                Piece::Text("http://example.com/".into()),
                                Piece::Attribute("name".into()),
                            ],
                            encoding: Encoding::IriSafe,
                        }),
                        base: Some("http://example.com/base/".into()),
                    },
                    origin: origin("http://example.com/base/sm"),
                },
                ExtendPair {
                    attribute: "o".into(),
                    expression: Expression::Literal {
                        inner: Box::new(Expression::Reference {
                            attribute: "name".into(),
                        }),
                        datatype: None,
                        language: None,
                    },
                    origin: origin("http://example.com/base/om"),
                },
            ],
        }),
        vec![],
    );
    let project = plan.add(
        Operator::Project(ProjectOp {
            attributes: vec!["s".into(), "o".into()],
        }),
        vec![],
    );
    let serializer = plan.add(
        Operator::Serializer(SerializerOp {
            patterns: vec![QuadPattern {
                subject: attribute("s"),
                predicate: Position::Constant {
                    term: Term::Iri {
                        iri: "http://xmlns.com/foaf/0.1/name".into(),
                    },
                    origin: vec!["http://example.com/base/pm".into()],
                },
                object: attribute("o"),
                graph: None,
                origin: origin("http://example.com/base/pom"),
            }],
        }),
        vec![tm.into()],
    );
    let target =
        plan.add(Operator::Target(TargetOp { default: true }), vec![]);
    plan.connect(source, extend);
    plan.connect(extend, project);
    plan.connect(project, serializer);
    plan.connect(serializer, target);

    assert_plan_snapshot(
        &plan.document(),
        &Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/resources/core-plan.json"),
    );
}
