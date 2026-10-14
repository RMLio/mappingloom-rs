use crate::translator::plan_snapshots::TestExecutor;
use std::error::Error;

const TEST_DIR: &str = "rml-core-tests";
#[test]
fn rmltc0000_json() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC0000-JSON", true)
}

#[test]
fn rmltc0001a_json() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC0001a-JSON", true)
}

#[test]
fn rmltc0001b_json() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC0001b-JSON", true)
}

#[test]
fn rmltc0002a_json() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC0002a-JSON", true)
}

#[test]
fn rmltc0002b_json() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC0002b-JSON", true)
}

#[test]
// Data error, so mapping should succeed.
fn rmltc0002e_json() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC0002e-JSON", true)
}

#[test]
// Data error, so mapping should succeed.
fn rmltc0002g_json() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC0002g-JSON", true)
}

#[test]
fn rmltc0003c_json() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC0003c-JSON", true)
}

#[test]
fn rmltc0004a_json() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC0004a-JSON", true)
}

#[test]
fn rmltc0004b_json() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC0004b-JSON", false)
}

#[test]
fn rmltc0005a_json() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC0005a-JSON", true)
}

#[test]
fn rmltc0006a_json() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC0006a-JSON", true)
}

#[test]
fn rmltc0007a_json() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC0007a-JSON", true)
}

#[test]
fn rmltc0007b_json() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC0007b-JSON", true)
}

#[test]
fn rmltc0007c_json() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC0007c-JSON", true)
}

#[test]
fn rmltc0007d_json() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC0007d-JSON", true)
}

#[test]
fn rmltc0007e_json() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC0007e-JSON", true)
}

#[test]
fn rmltc0007f_json() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC0007f-JSON", true)
}

#[test]
fn rmltc0007g_json() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC0007g-JSON", true)
}

#[test]
fn rmltc0007h_json() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC0007h-JSON", false)
}

#[test]
fn rmltc0008a_json() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC0008a-JSON", true)
}

#[test]
fn rmltc0008b_json() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC0008b-JSON", true)
}

#[test]
fn rmltc0008c_json() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC0008c-JSON", true)
}

#[test]
fn rmltc0009a_json() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC0009a-JSON", true)
}

#[test]
fn rmltc0009b_json() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC0009b-JSON", true)
}

#[test]
fn rmltc0010a_json() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC0010a-JSON", true)
}

#[test]
fn rmltc0010b_json() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC0010b-JSON", true)
}

#[test]
fn rmltc0010c_json() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC0010c-JSON", true)
}

#[test]
fn rmltc0011b_json() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC0011b-JSON", true)
}

#[test]
fn rmltc0012a_json() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC0012a-JSON", true)
}

#[test]
fn rmltc0012b_json() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC0012b-JSON", true)
}

#[test]
fn rmltc0012c_json() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC0012c-JSON", false)
}

#[test]
fn rmltc0012d_json() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC0012d-JSON", false)
}

#[test]
fn rmltc0012e_json() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC0012e-JSON", true)
}

#[test]
fn rmltc0013a_json() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC0013a-JSON", true)
}

#[test]
fn rmltc0015a_json() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC0015a-JSON", true)
}

#[test]
// Data error, so mapping should succeed.
fn rmltc0015b_json() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC0015b-JSON", true)
}

#[test]
fn rmltc0019a_json() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC0019a-JSON", true)
}

#[test]
// Data error, so mapping should succeed.
fn rmltc0019b_json() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC0019b-JSON", true)
}

#[test]
fn rmltc0020a_json() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC0020a-JSON", true)
}

#[test]
fn rmltc0021a_json() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC0021a-JSON", true)
}

#[test]
fn rmltc0022a_json() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC0022a-JSON", true)
}

#[test]
fn rmltc0022b_json() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC0022b-JSON", true)
}

#[test]
fn rmltc0022c_json() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC0022c-JSON", true)
}

#[test]
fn rmltc0022d_json() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC0022d-JSON", true)
}

#[test]
fn rmltc0022e_json() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC0022e-JSON", true)
}

#[test]
fn rmltc0023a_json() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC0023a-JSON", false)
}

#[test]
fn rmltc0023b_json() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC0023b-JSON", false)
}

#[test]
fn rmltc0023c_json() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC0023c-JSON", false)
}

#[test]
fn rmltc0023d_json() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC0023d-JSON", false)
}

#[test]
fn rmltc0023e_json() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC0023e-JSON", false)
}

#[test]
fn rmltc0023f_json() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC0023f-JSON", true)
}

#[test]
fn rmltc0024a_json() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC0024a-JSON", false)
}

#[test]
fn rmltc0025a_json() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC0025a-JSON", true)
}

#[test]
// Data error, so mapping should succeed.
fn rmltc0025b_json() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC0025b-JSON", true)
}

#[test]
fn rmltc0025c_json() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC0025c-JSON", true)
}

#[test]
fn rmltc0026a_json() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC0026a-JSON", true)
}

#[test]
fn rmltc0026b_json() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC0026b-JSON", true)
}

#[test]
fn rmltc0026c_json() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC0026c-JSON", true)
}

#[test]
fn rmltc0026d_json() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC0026d-JSON", true)
}

#[test]
fn rmltc0027a_json() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC0027a-JSON", true)
}

#[test]
fn rmltc0027b_json() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC0027b-JSON", true)
}

#[test]
fn rmltc0027c_json() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC0027c-JSON", true)
}

#[test]
fn rmltc0028a_json() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC0028a-JSON", true)
}

#[test]
fn rmltc0028b_json() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC0028b-JSON", true)
}

#[test]
fn rmltc0028c_json() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC0028c-JSON", true)
}

#[test]
fn rmltc0029a_json() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC0029a-JSON", true)
}

#[test]
fn rmltc0030a_json() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC0030a-JSON", true)
}

#[test]
fn rmltc0030b_json() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC0030b-JSON", true)
}

#[test]
fn rmltc0030c_json() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC0030c-JSON", true)
}

#[test]
fn rmltc0030d_json() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC0030d-JSON", true)
}

#[test]
fn rmltc0030e_json() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC0030e-JSON", true)
}

#[test]
fn rmltc0030f_json() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC0030f-JSON", true)
}

#[test]
fn rmltc0031a_json() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC0031a-JSON", true)
}

#[test]
fn rmltc0031b_json() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC0031b-JSON", true)
}

#[test]
fn rmltc0031c_json() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC0031c-JSON", true)
}
