use std::io::Write;
use std::process::{Command, Stdio};

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

#[test]
fn stdin_keeps_line_breaks() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_translator-bin"))
        .arg("stdin")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(MAPPING_WITH_COMMENT.as_bytes())
        .unwrap();
    let output = child.wait_with_output().unwrap();

    let plan = String::from_utf8(output.stdout).unwrap();
    assert!(plan.contains("Serialize"), "no plan on stdout: {plan}");
}
