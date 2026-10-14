use crate::translator::plan_snapshots::TestExecutor;
use std::error::Error;

const TEST_DIR: &str = "rml-io-tests";

#[test]
fn rmlstc0001a() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLSTC0001a", true)
}

#[test]
fn rmlstc0001b() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLSTC0001b", true)
}

#[test]
fn rmlstc0002a() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLSTC0002a", true)
}

#[test]
fn rmlstc0002b() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLSTC0002b", true)
}

#[test]
fn rmlstc0002c() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLSTC0002c", true)
}

#[test]
fn rmlstc0002d() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLSTC0002d", true)
}

#[test]
fn rmlstc0002e() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLSTC0002e", true)
}

#[test]
#[ignore = "SPARQL_Results_CSV reference formulation not supported"]
fn rmlstc0003() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLSTC0003", true)
}

#[test]
fn rmlstc0004a() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLSTC0004a", true)
}

#[test]
fn rmlstc0004b() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLSTC0004b", true)
}

#[test]
fn rmlstc0004c() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLSTC0004c", true)
}

#[test]
fn rmlstc0006a() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLSTC0006a", true)
}

#[test]
fn rmlstc0006b() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLSTC0006b", true)
}

#[test]
fn rmlstc0007a() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLSTC0007a", true)
}

#[test]
fn rmlstc0007b() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLSTC0007b", true)
}

#[test]
fn rmlstc0007c() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLSTC0007c", true)
}

#[test]
fn rmlstc0007d() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLSTC0007d", true)
}

#[test]
fn rmlstc0008a() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLSTC0008a", true)
}

#[test]
fn rmlstc0008b() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLSTC0008b", true)
}

#[test]
// Data error, so mapping should succeed.
fn rmlstc0009a() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLSTC0009a", true)
}

#[test]
// Data error, so mapping should succeed.
fn rmlstc0010a() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLSTC0010a", true)
}

#[test]
// Data error, so mapping should succeed.
fn rmlstc0010b() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLSTC0010b", true)
}

#[test]
fn rmlstc0011a() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLSTC0011a", true)
}

#[test]
fn rmlstc0011b() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLSTC0011b", true)
}

#[test]
fn rmlstc0011c() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLSTC0011c", true)
}

#[test]
fn rmlstc0011d() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLSTC0011d", true)
}

#[test]
fn rmlstc0011e() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLSTC0011e", true)
}

#[test]
fn rmlstc0012a() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLSTC0012a", true)
}

#[test]
fn rmlstc0012b() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLSTC0012b", true)
}

#[test]
fn rmlstc0012c() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLSTC0012c", true)
}

#[test]
fn rmlstc0012d() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLSTC0012d", true)
}

#[test]
fn rmlstc0012e() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLSTC0012e", true)
}

#[test]
fn rmlttc0000() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTTC0000", true)
}

#[test]
fn rmlttc0001a() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTTC0001a", true)
}

#[test]
fn rmlttc0001b() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTTC0001b", true)
}

#[test]
fn rmlttc0001c() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTTC0001c", true)
}

#[test]
fn rmlttc0001d() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTTC0001d", true)
}

#[test]
fn rmlttc0001e() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTTC0001e", true)
}

#[test]
fn rmlttc0001f() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTTC0001f", true)
}

#[test]
fn rmlttc0002a() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTTC0002a", true)
}

#[test]
fn rmlttc0002b() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTTC0002b", true)
}

#[test]
fn rmlttc0002c() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTTC0002c", true)
}

#[test]
fn rmlttc0002d() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTTC0002d", true)
}

#[test]
fn rmlttc0002e() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTTC0002e", true)
}

#[test]
fn rmlttc0002f() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTTC0002f", true)
}

#[test]
fn rmlttc0002g() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTTC0002g", true)
}

#[test]
fn rmlttc0002h() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTTC0002h", true)
}

#[test]
fn rmlttc0002i() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTTC0002i", true)
}

#[test]
fn rmlttc0002j() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTTC0002j", true)
}

#[test]
fn rmlttc0002k() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTTC0002k", true)
}

#[test]
fn rmlttc0002l() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTTC0002l", true)
}

#[test]
fn rmlttc0002m() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTTC0002m", true)
}

#[test]
fn rmlttc0002n() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTTC0002n", true)
}

#[test]
fn rmlttc0002o() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTTC0002o", true)
}

#[test]
fn rmlttc0002p() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTTC0002p", true)
}

#[test]
fn rmlttc0002q() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTTC0002q", true)
}

#[test]
fn rmlttc0002r() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTTC0002r", true)
}

#[test]
fn rmlttc0003a() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTTC0003a", true)
}

#[test]
fn rmlttc0004a() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTTC0004a", true)
}

#[test]
fn rmlttc0004b() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTTC0004b", true)
}

#[test]
fn rmlttc0004c() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTTC0004c", true)
}

#[test]
fn rmlttc0004d() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTTC0004d", true)
}

#[test]
fn rmlttc0004e() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTTC0004e", true)
}

#[test]
fn rmlttc0004f() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTTC0004f", true)
}

#[test]
fn rmlttc0004g() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTTC0004g", true)
}

#[test]
fn rmlttc0005a() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTTC0005a", true)
}

#[test]
fn rmlttc0005b() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTTC0005b", true)
}

#[test]
fn rmlttc0006a() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTTC0006a", true)
}

#[test]
fn rmlttc0006b() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTTC0006b", true)
}

#[test]
fn rmlttc0006c() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTTC0006c", true)
}

#[test]
fn rmlttc0006d() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTTC0006d", true)
}

#[test]
fn rmlttc0006e() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTTC0006e", true)
}

#[test]
fn rmlttc0007a() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTTC0007a", true)
}

// CSV on the Web: the table names the file and the values standing for no value, and the
// dialect it points at says how the rows are written.

#[test]
fn rmlioregtc0012a() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLIOREGTC0012a", true)
}

#[test]
fn rmlioregtc0012b() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLIOREGTC0012b", true)
}

#[test]
fn rmlioregtc0012c() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLIOREGTC0012c", true)
}

#[test]
fn rmlioregtc0012d() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLIOREGTC0012d", true)
}

#[test]
fn rmlioregtc0012e() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLIOREGTC0012e", true)
}

#[test]
fn rmlioregtc0012f() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLIOREGTC0012f", true)
}

#[test]
fn rmlioregtc0012g() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLIOREGTC0012g", true)
}

#[test]
fn rmlioregtc0012h() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLIOREGTC0012h", true)
}

#[test]
fn rmlioregtc0012i() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLIOREGTC0012i", true)
}
