use std::io::Write;
use std::process::{Command, Output, Stdio};

use serde_json::Value;

// A Turtle comment ends at the newline, so the mapping after it must survive
// reading from stdin.
const MAPPING_WITH_COMMENT: &str = r#"# Students to FOAF
@prefix rr: <http://www.w3.org/ns/r2rml#> .
@prefix foaf: <http://xmlns.com/foaf/0.1/> .
@prefix rml: <http://semweb.mmlab.be/ns/rml#> .
@prefix ql: <http://semweb.mmlab.be/ns/ql#> .

<http://example.com/base/TriplesMap1> a rr:TriplesMap ;
  rml:logicalSource [ rml:source "student.csv" ; rml:referenceFormulation ql:CSV ] ;
  rr:subjectMap [ rr:template "http://example.com/{Name}" ] ;
  rr:predicateObjectMap [ rr:predicate foaf:name ; rr:objectMap [ rml:reference "Name" ] ] .
"#;

fn run_stdin(args: &[&str], mapping: &str) -> Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_translator-bin"))
        .args(args)
        .arg("stdin")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(mapping.as_bytes())
        .unwrap();
    child.wait_with_output().unwrap()
}

fn report(output: &Output) -> Value {
    serde_json::from_slice(&output.stdout).unwrap()
}

#[test]
fn stdin_keeps_line_breaks() {
    let output = run_stdin(&["--language", "rml-io"], MAPPING_WITH_COMMENT);
    let plan = String::from_utf8(output.stdout).unwrap();
    assert!(plan.contains("Serialize"), "no plan on stdout: {plan}");
    assert_eq!(output.status.code(), Some(0));
}

#[test]
fn the_exit_code_follows_the_error_kind() {
    let syntax = run_stdin(&["--language", "rml-kgc"], "<a> <b> .");
    assert_eq!(syntax.status.code(), Some(1));
    assert!(syntax.stdout.trim_ascii().is_empty());

    let no_language = run_stdin(&[], MAPPING_WITH_COMMENT);
    assert_eq!(no_language.status.code(), Some(2));

    let unknown_rule = run_stdin(
        &["--language", "rml-io", "--optimizer", "R9=true"],
        MAPPING_WITH_COMMENT,
    );
    assert_eq!(unknown_rule.status.code(), Some(2));
}

#[test]
fn report_writes_the_result_document() {
    let plan =
        run_stdin(&["--language", "rml-io", "--report"], MAPPING_WITH_COMMENT);
    assert_eq!(plan.status.code(), Some(0));
    assert!(report(&plan)["plan"]["nodes"].is_array());

    let errors = run_stdin(&["--language", "rml-kgc", "--report"], "<a> <b> .");
    assert_eq!(errors.status.code(), Some(1));
    let errors = report(&errors);
    assert_eq!(errors["errors"][0]["kind"], "syntax");
    assert_eq!(errors["errors"][0]["location"]["line"], 1);
}

#[test]
fn the_flags_override_the_options_file() {
    let options = std::env::temp_dir().join(format!(
        "translator-bin-options-{}.json",
        std::process::id()
    ));
    std::fs::write(
        &options,
        r#"{"language": "shexml", "base_iri": "http://other.org/"}"#,
    )
    .unwrap();
    let output = run_stdin(
        &[
            "--options",
            options.to_str().unwrap(),
            "--language",
            "rml-io",
        ],
        MAPPING_WITH_COMMENT,
    );
    std::fs::remove_file(&options).unwrap();
    assert_eq!(output.status.code(), Some(0));
    let plan = String::from_utf8(output.stdout).unwrap();
    assert!(plan.contains("http://other.org/"), "{plan}");
}

#[test]
fn report_writes_a_request_error() {
    let output = run_stdin(&["--language", "rml", "--report"], "");
    assert_eq!(output.status.code(), Some(2));
    assert_eq!(report(&output)["errors"][0]["code"], "invalid-options");
}
