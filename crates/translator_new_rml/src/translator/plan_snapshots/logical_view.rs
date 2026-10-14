use crate::translator::plan_snapshots::TestExecutor;
use std::error::Error;

const TEST_DIR: &str = "rml-lv-tests";

#[test]
fn rmllvtc0000a() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLLVTC0000a", true)
}

#[test]
fn rmllvtc0000b() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLLVTC0000b", true)
}

#[test]
fn rmllvtc0000c() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLLVTC0000c", true)
}

#[test]
fn rmllvtc0001a() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLLVTC0001a", true)
}

#[test]
fn rmllvtc0001b() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLLVTC0001b", true)
}

#[test]
fn rmllvtc0001c() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLLVTC0001c", true)
}

#[test]
fn rmllvtc0001d() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLLVTC0001d", true)
}

#[test]
fn rmllvtc0002a() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLLVTC0002a", true)
}

#[test]
fn rmllvtc0002b() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLLVTC0002b", true)
}

#[test]
fn rmllvtc0002c() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLLVTC0002c", true)
}

#[test]
fn rmllvtc0003a() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLLVTC0003a", true)
}

#[test]
fn rmllvtc0003b() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLLVTC0003b", true)
}

#[test]
fn rmllvtc0003c() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLLVTC0003c", true)
}

#[test]
fn rmllvtc0004a() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLLVTC0004a", true)
}

#[test]
fn rmllvtc0004b() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLLVTC0004b", true)
}

#[test]
fn rmllvtc0004c() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLLVTC0004c", true)
}

#[test]
fn rmllvtc0004d() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLLVTC0004d", true)
}

#[test]
// Data error, so mapping should succeed
fn rmllvtc0005a() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLLVTC0005a", true)
}

#[test]
// Data error, so mapping should succeed
fn rmllvtc0005b() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLLVTC0005b", true)
}

#[test]
// Data error, so mapping should succeed
 fn rmllvtc0005c() -> Result<(), Box<dyn Error>> {
     TestExecutor::new(TEST_DIR).execute("RMLLVTC0005c", true)
 }

#[test]
fn rmllvtc0006a() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLLVTC0006a", true)
}

#[test]
fn rmllvtc0006b() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLLVTC0006b", true)
}

#[test]
fn rmllvtc0006c() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLLVTC0006c", true)
}

#[test]
fn rmllvtc0006d() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLLVTC0006d", true)
}

#[test]
fn rmllvtc0006e() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLLVTC0006e", true)
}

#[test]
fn rmllvtc0006f() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLLVTC0006f", true)
}

#[test]
fn rmllvtc0007a() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLLVTC0007a", true)
}

#[test]
fn rmllvtc0007b() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLLVTC0007b", true)
}

#[test]
fn rmllvtc0007c() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLLVTC0007c", true)
}

#[test]
fn rmllvtc0008a() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLLVTC0008a", false)
}

#[test]
fn rmllvtc0008b() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLLVTC0008b", false)
}

#[test]
fn rmllvtc0008c() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLLVTC0008c", false)
}

#[test]
fn rmllvtc0008d() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLLVTC0008d", false)
}

#[test]
fn rmllvtc0009a() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLLVTC0009a", false)
}

#[test]
#[ignore = "Name collision in join field not detected"]
fn rmllvtc0009b() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLLVTC0009b", false)
}

#[test]
#[ignore = "Name collision between fields from different joins not detected"]
fn rmllvtc0009c() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLLVTC0009c", false)
}

#[test]
fn rmllvtc0010a() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLLVTC0010a", true)
}

#[test]
fn rmllvtc0010b() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLLVTC0010b", true)
}

#[test]
fn rmllvtc0010c() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLLVTC0010c", true)
}

#[test]
fn rmllvtc0010d() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLLVTC0010d", true)
}

#[test]
fn rmllvtc0010e() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLLVTC0010e", true)
}