//! Each rule of the KGC rule set on its own cases, and the whole set on
//! mappings (`resources/test/kgc/<rule id>/`).

use std::path::PathBuf;

use normalizer::{apply, kgc, normalize_kgc};
use oxigraph::model::Term;
use pipeline_test::{assert_graph_cases, parse_graph};

const BASE: Option<&str> = Some("http://example.com/base/");

fn cases(dir: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("resources/test/kgc")
        .join(dir)
}

fn assert_rule(id: &str) {
    let rule = kgc::RULES.iter().find(|rule| rule.id == id).unwrap();
    assert_graph_cases(&cases(id), |input| {
        apply(input, BASE, std::slice::from_ref(rule))
            .unwrap()
            .normalized
    });
}

#[test]
fn constant_shortcuts() {
    assert_rule("2-constant-shortcuts");
}

#[test]
fn datatype_and_language_shortcuts() {
    assert_rule("datatype-and-language-shortcuts");
}

#[test]
fn join_condition_shortcuts() {
    assert_rule("join-condition-shortcuts");
}

#[test]
fn term_types() {
    assert_rule("term-types");
}

#[test]
fn classes() {
    assert_rule("1-classes");
}

#[test]
fn same_source_joins() {
    assert_rule("same-source-joins");
}

#[test]
fn split_predicate_object_maps() {
    assert_rule("3-split-predicate-object-maps");
}

#[test]
fn subject_graph_maps() {
    assert_rule("subject-graph-maps");
}

#[test]
fn base_iris() {
    assert_rule("base-iris");
}

#[test]
fn split_triples_maps() {
    assert_rule("5-split-triples-maps");
}

#[test]
fn rule_set() {
    assert_graph_cases(&cases("rule-set"), |input| {
        normalize_kgc(input, BASE).unwrap().normalized
    });
}

/// Every `nf:origin` link names a node of the original document: an IRI of
/// it, or the skolem IRI of one of its blank nodes, so the translator can
/// give every node an origin.
#[test]
fn origins_are_original_nodes() {
    let mapping =
        std::fs::read_to_string(cases("rule-set").join("input.ttl")).unwrap();
    let original = parse_graph(&mapping);
    let normal_form = normalize_kgc(&mapping, BASE).unwrap();
    let origin = format!("{}origin", normalizer::NF);
    let targets: Vec<Term> = normal_form
        .normalized
        .iter()
        .filter(|triple| triple.predicate.as_str() == origin)
        .map(|triple| triple.object.into_owned())
        .collect();
    assert!(targets
        .iter()
        .any(|target| target.to_string().contains(normalizer::GENID)));
    for target in targets {
        let original_iri = matches!(target, Term::NamedNode(_))
            && original.iter().any(|triple| {
                Term::from(triple.subject.into_owned()) == target
            });
        let skolem_iri = normal_form
            .skolem
            .values()
            .any(|iri| Term::from(iri.clone()) == target);
        assert!(
            original_iri || skolem_iri,
            "{target} is not a node of the original document"
        );
    }
}
