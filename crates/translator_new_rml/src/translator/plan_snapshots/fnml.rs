use crate::translator::plan_snapshots::TestExecutor;
use std::error::Error;

const TEST_DIR: &str = "rml-fnml-tests";

#[test]
fn rmlfnmltc0001_csv() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLFNMLTC0001-CSV", true)
}

#[test]
fn rmlfnmltc0002_csv() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLFNMLTC0002-CSV", true)
}

#[test]
fn rmlfnmltc0003_csv() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLFNMLTC0003-CSV", true)
}

#[test]
fn rmlfnmltc0004_csv() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLFNMLTC0004-CSV", true)
}

#[test]
fn rmlfnmltc0005_csv() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLFNMLTC0005-CSV", true)
}

#[test]
fn rmlfnmltc0007_csv() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLFNMLTC0007-CSV", true)
}

#[test]
fn rmlfnmltc0008_csv() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLFNMLTC0008-CSV", true)
}

#[test]
fn rmlfnmltc0011_csv() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLFNMLTC0011-CSV", true)
}

#[test]
fn rmlfnmltc0021_csv() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLFNMLTC0021-CSV", true)
}

#[test]
fn rmlfnmltc0031_csv() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLFNMLTC0031-CSV", true)
}

#[test]
fn rmlfnmltc0032_csv() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLFNMLTC0032-CSV", true)
}

#[test]
fn rmlfnmltc0041_csv() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLFNMLTC0041-CSV", true)
}

#[test]
fn rmlfnmltc0051_csv() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLFNMLTC0051-CSV", true)
}

#[test]
fn rmlfnmltc0061_csv() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLFNMLTC0061-CSV", true)
}

#[test]
fn rmlfnmltc0071_csv() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLFNMLTC0071-CSV", true)
}

#[test]
fn rmlfnmltc0081_csv() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLFNMLTC0081-CSV", true)
}

#[test]
// Wrong function/parameter is only caught at execution, not during translation, so a plan is generated.
fn rmlfnmltc0101_csv() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLFNMLTC0101-CSV", true)
}

#[test]
// Wrong function/parameter is only caught at execution, not during translation, so a plan is generated.
fn rmlfnmltc0102_csv() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLFNMLTC0102-CSV", true)
}

#[test]
// Wrong function/parameter is only caught at execution, not during translation, so a plan is generated.
fn rmlfnmltc0103_csv() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLFNMLTC0103-CSV", true)
}

#[test]
// Wrong function/parameter is only caught at execution, not during translation, so a plan is generated.
fn rmlfnmltc0104_csv() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLFNMLTC0104-CSV", true)
}
