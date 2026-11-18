//! Test harness of the canonical pipeline: graph cases for the normalizer
//! rules, and plan snapshots, validated against the plan schema, for the
//! translator and the optimizer rules.

use std::fs;
use std::path::Path;

use algemaploom_plan::PlanDocument;
use oxigraph::io::{RdfFormat, RdfParser, RdfSerializer};
use oxigraph::model::dataset::CanonicalizationAlgorithm;
use oxigraph::model::{Graph, Triple};
use serde_json::Value;

/// Parses a Turtle document into a graph.
pub fn parse_graph(turtle: &str) -> Graph {
    RdfParser::from_format(RdfFormat::Turtle)
        .for_slice(turtle)
        .map(|quad| Triple::from(quad.expect("valid Turtle")))
        .collect()
}

/// Asserts that two graphs are isomorphic, and prints both on failure.
pub fn assert_isomorphic(actual: &Graph, expected: &Graph) {
    let canonical = |graph: &Graph| {
        let mut graph = graph.clone();
        graph.canonicalize(CanonicalizationAlgorithm::Unstable);
        graph
    };
    assert!(
        canonical(actual) == canonical(expected),
        "graphs differ\n--- actual\n{}\n--- expected\n{}",
        to_ntriples(actual),
        to_ntriples(expected)
    );
}

fn to_ntriples(graph: &Graph) -> String {
    let mut serializer =
        RdfSerializer::from_format(RdfFormat::NTriples).for_writer(vec![]);
    for triple in graph {
        serializer.serialize_triple(triple).unwrap();
    }
    let mut lines: Vec<_> = String::from_utf8(serializer.finish().unwrap())
        .unwrap()
        .lines()
        .map(str::to_string)
        .collect();
    lines.sort();
    lines.join("\n")
}

/// Runs the cases in `dir`: a case is a folder with `input.ttl` and
/// `expected.ttl`, either `dir` itself or one of its subfolders.
/// `apply` turns the input text into the actual graph, which must be
/// isomorphic to the expected one. Every failing case is reported.
pub fn assert_graph_cases(
    dir: &Path,
    apply: impl Fn(&str) -> Graph + std::panic::RefUnwindSafe,
) {
    let mut cases: Vec<_> = fs::read_dir(dir)
        .unwrap_or_else(|e| panic!("{}: {e}", dir.display()))
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.is_dir())
        .chain(dir.join("input.ttl").is_file().then(|| dir.to_path_buf()))
        .collect();
    cases.sort();
    assert!(!cases.is_empty(), "no cases in {}", dir.display());
    let failures: Vec<_> = cases
        .iter()
        .filter_map(|case| {
            let read = |name| fs::read_to_string(case.join(name)).unwrap();
            let expected = parse_graph(&read("expected.ttl"));
            std::panic::catch_unwind(|| {
                assert_isomorphic(&apply(&read("input.ttl")), &expected)
            })
            .err()
            .map(|_| case.display().to_string())
        })
        .collect();
    assert!(failures.is_empty(), "failing cases: {failures:#?}");
}

/// Asserts that a plan document is valid under the plan schema.
pub fn assert_valid_plan(document: &PlanDocument) {
    let schema = algemaploom_plan::schema();
    let validator = jsonschema::validator_for(&schema).unwrap();
    let instance = serde_json::to_value(document).unwrap();
    let errors: Vec<_> = validator
        .iter_errors(&instance)
        .map(|error| format!("{}: {error}", error.instance_path()))
        .collect();
    assert!(errors.is_empty(), "plan violates the schema: {errors:#?}");
}

/// Asserts that a plan document is valid and equals the snapshot at `path`, compared as JSON.
/// The snapshot leaves out `version`, so a version bump or a release rewrites none.
/// `UPDATE_SNAPSHOTS=1` writes the snapshot instead.
pub fn assert_plan_snapshot(document: &PlanDocument, path: &Path) {
    assert_valid_plan(document);
    let mut actual = serde_json::to_value(document).unwrap();
    actual.as_object_mut().unwrap().remove("version");
    assert_json_snapshot(&actual, path);
}

/// Asserts that a JSON value equals the snapshot at `path`.
/// `UPDATE_SNAPSHOTS=1` writes the snapshot instead.
pub fn assert_json_snapshot(actual: &Value, path: &Path) {
    if std::env::var_os("UPDATE_SNAPSHOTS").is_some() {
        let text = serde_json::to_string_pretty(actual).unwrap();
        fs::write(path, text + "\n").unwrap();
    }
    let snapshot: Value = serde_json::from_str(
        &fs::read_to_string(path)
            .unwrap_or_else(|e| panic!("{}: {e}", path.display())),
    )
    .unwrap();
    assert_eq!(
        actual,
        &snapshot,
        "JSON differs from {}; UPDATE_SNAPSHOTS=1 rewrites it",
        path.display()
    );
}
