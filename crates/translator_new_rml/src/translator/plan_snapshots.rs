//! Plan snapshots: each test translates the `mapping.ttl` of a kg-construct
//! test case and compares the plan with the recorded `mapping.json`. They
//! detect changes to the plan; they check neither the plan's correctness nor
//! conformance, which the conformance run of MappingWeaver checks (HANDBOOK,
//! Test resources). The comparison hashes the sorted characters of the plan
//! JSON, so it is blind to any change that keeps the same characters.

mod cc;
mod core;
mod fnml;
mod io;
mod logical_view;
mod star;
mod rml_lv_fnml;

use crate::translator::NewRMLDocumentTranslator;
use std::error::Error;
use std::fs;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::path::PathBuf;
use translator_api::LanguageTranslator;

struct TestExecutor {
    test_dir: PathBuf,
}

impl TestExecutor {
    fn new(test_dir_name: &str) -> Self {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources");
        path.push("test");
        path.push(test_dir_name);

        TestExecutor {
            test_dir: path
        }
    }

    pub fn execute(&self, test_name: &str, positive: bool) -> Result<(), Box<dyn Error>> {
        let test_path = self.test_dir.join(test_name);
        let mapping_path = test_path.join("mapping.ttl");
        let plan_result = NewRMLDocumentTranslator::translate_to_plan(mapping_path.as_path());
        if let Ok(plan) = plan_result {
            if !positive {
                panic!("A plan was generated but the test was expected to fail");
            }
            let actual_json_plan = plan.to_json_string()?;
            let actual_plan_hash = json_hash(&actual_json_plan);

            let expected_json_path = test_path.join("mapping.json");
            let expected_json_str = fs::read_to_string(expected_json_path)?;
            let expected_plan_hash = json_hash(&expected_json_str);
            assert_eq!(actual_plan_hash, expected_plan_hash, "Generated plan did not match expected plan.\nActual:\n{}\nExpected:\n{}", actual_json_plan, expected_json_str);
        } else {
            if positive {
                panic!("No plan was generated but the test was expected to pass");
            }
        }

        Ok(())
    }
}

/// Deserializes a JSON string to a JSON object and then serializes it back to a string.
/// Then the hash of this string is returned.
/// This workaround catches the case where comparing JSON objects is not enough,
/// e.g. if strings within a template are switched between different runs.
fn json_hash(json_str: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    let value = serde_json::from_str::<serde_json::Value>(json_str).unwrap();
    let unsorted_str = serde_json::to_string(&value).unwrap();
    let mut char_str = unsorted_str.chars().collect::<Vec<char>>();
    char_str.sort();
    char_str.hash(&mut hasher);
    hasher.finish()
}