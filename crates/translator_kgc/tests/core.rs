//! The canonical translator on the kg-construct RML-Core cases, which
//! `translator_new_rml` keeps under its `resources/test/rml-core-tests/`.
//! Each case has a snapshot in `tests/core/<case>.json`: the plan, validated
//! against the plan schema, or the translation errors. Whether a plan is
//! right is decided by running it (HANDBOOK: Conformance).

use std::fs;
use std::path::Path;

use pipeline_test::{assert_json_snapshot, assert_plan_snapshot};
use serde_json::{json, Value};

const BASE: Option<&str> = Some("http://example.com/base/");

#[test]
fn core_cases() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let cases =
        root.join("../translator_new_rml/resources/test/rml-core-tests");
    let mut cases: Vec<_> = fs::read_dir(cases)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .collect();
    cases.sort();
    for case in cases {
        let name = case.file_name().unwrap().to_string_lossy().to_string();
        let mapping = fs::read_to_string(case.join("mapping.ttl")).unwrap();
        let snapshot = root.join("tests/core").join(format!("{name}.json"));
        match translator_kgc::translate(&mapping, BASE) {
            Ok(plan) => assert_plan_snapshot(&plan.document(), &snapshot),
            Err(errors) => {
                let errors: Value = errors
                    .iter()
                    .map(|error| {
                        json!({
                            "kind": format!("{:?}", error.kind),
                            "code": error.code,
                            "message": error.message,
                        })
                    })
                    .collect();
                assert_json_snapshot(&json!({ "errors": errors }), &snapshot);
            }
        }
    }
}
