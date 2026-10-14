use crate::translator::plan_snapshots::TestExecutor;
use std::error::Error;

const TEST_DIR: &str = "rml-lv-fnml";

#[test]
fn rml_lv_fnml_00001a() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLLVFNML0001a-CSV", true)
}