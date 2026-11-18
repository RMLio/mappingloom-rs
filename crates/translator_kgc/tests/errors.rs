//! Mapping errors that no kg-construct case covers, each on a small mapping
//! with one triples map.

use translator_kgc::{translate, ErrorKind};

/// The error codes of a triples map with `subject` as subject map and `pom`
/// as predicate-object map.
fn error_codes(subject: &str, pom: &str) -> Vec<(ErrorKind, &'static str)> {
    let mapping = format!(
        "@prefix rml: <http://w3id.org/rml/> .
        <http://example.com/tm> rml:subjectMap {subject} ;
          rml:predicateObjectMap {pom} ."
    );
    translate(&mapping, Some("http://example.com/"))
        .expect_err("a mapping error")
        .iter()
        .map(|error| (error.kind, error.code))
        .collect()
}

const SUBJECT: &str = "[ rml:constant <http://example.com/s> ]";

#[test]
fn a_predicate_object_map_needs_an_object_map() {
    assert_eq!(
        error_codes(SUBJECT, "[ rml:predicate <http://example.com/p> ]"),
        [(ErrorKind::Mapping, "predicate-object-map")]
    );
}

#[test]
fn a_subject_map_is_a_node() {
    assert_eq!(
        error_codes(
            "\"s\"",
            "[ rml:predicate <http://example.com/p> ; rml:object 1 ]"
        ),
        [(ErrorKind::Mapping, "subject-maps")]
    );
}

#[test]
fn a_fresh_blank_node_is_no_predicate() {
    assert_eq!(
        error_codes(
            SUBJECT,
            "[ rml:predicateMap [ rml:termType rml:BlankNode ] ; rml:object 1 ]"
        ),
        [(ErrorKind::Mapping, "invalid-term-type")]
    );
}

#[test]
fn a_constant_keeps_its_own_term_type() {
    assert_eq!(
        error_codes(
            SUBJECT,
            "[ rml:predicate <http://example.com/p> ;
               rml:objectMap [ rml:constant \"a\" ; rml:termType rml:IRI ] ]"
        ),
        [(ErrorKind::Mapping, "invalid-term-type")]
    );
}

#[test]
fn a_term_map_has_one_expression() {
    assert_eq!(
        error_codes(
            "[ rml:reference \"id\" ; rml:template \"http://example.com/{id}\" ]",
            "[ rml:predicate <http://example.com/p> ; rml:object 1 ]"
        ),
        [(ErrorKind::Mapping, "expressions")]
    );
}

#[test]
fn a_function_in_a_language_map_is_unsupported() {
    assert_eq!(
        error_codes(
            SUBJECT,
            "[ rml:predicate <http://example.com/p> ;
               rml:objectMap [ rml:reference \"a\" ;
                 rml:languageMap [ rml:functionExecution [] ] ] ]"
        ),
        [
            (ErrorKind::Mapping, "reference-without-logical-source"),
            (ErrorKind::Unsupported, "feature")
        ]
    );
}

#[test]
fn every_term_map_error_is_reported() {
    assert_eq!(
        error_codes(
            "[ rml:template \"{a\" ]",
            "[ rml:predicate <http://example.com/p> ;
               rml:objectMap [ rml:constant 1 ;
                 rml:logicalTarget [ rml:target [] ] ] ]"
        ),
        [
            (ErrorKind::Mapping, "invalid-template"),
            (ErrorKind::Unsupported, "feature")
        ]
    );
}

#[test]
fn a_condition_is_unsupported() {
    assert_eq!(
        error_codes(
            "[ rml:constant <http://example.com/s> ;
               rml:condition [ rml:functionExecution [] ] ]",
            "[ rml:predicate <http://example.com/p> ; rml:object 1 ]"
        ),
        [(ErrorKind::Unsupported, "feature")]
    );
}

#[test]
fn the_subject_map_of_a_triples_map_without_pom_is_checked() {
    let mapping = "@prefix rml: <http://w3id.org/rml/> .
        <http://example.com/tm> rml:subjectMap [
          rml:gather ( [ rml:constant <http://example.com/a> ] ) ] .";
    let errors = translate(mapping, None).expect_err("an unsupported error");
    assert_eq!(
        errors.iter().map(|e| (e.kind, e.code)).collect::<Vec<_>>(),
        [(ErrorKind::Unsupported, "feature")]
    );
}

#[test]
fn a_join_condition_has_one_child_map() {
    assert_eq!(
        error_codes(
            SUBJECT,
            "[ rml:predicate <http://example.com/p> ;
               rml:objectMap [ rml:parentTriplesMap <http://example.com/tm> ;
                 rml:joinCondition [
                   rml:childMap [ rml:constant 1 ], [ rml:constant 2 ] ;
                   rml:parentMap [ rml:constant 1 ] ] ] ]"
        ),
        [(ErrorKind::Mapping, "join-condition")]
    );
}
