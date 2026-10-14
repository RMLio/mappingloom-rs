use crate::translator::plan_snapshots::TestExecutor;
use std::error::Error;

const TEST_DIR: &str = "rml-star-tests";

#[test]
#[ignore = "RDF-star quoted triples not supported: term map has no expression map"]
fn rmlstartc001a() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLSTARTC001a", true)
}

#[test]
#[ignore = "RDF-star quoted triples not supported: term map has no expression map"]
fn rmlstartc001b() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLSTARTC001b", true)
}

#[test]
#[ignore = "RDF-star quoted triples not supported: term map has no expression map"]
fn rmlstartc002a() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLSTARTC002a", true)
}

#[test]
#[ignore = "RDF-star quoted triples not supported: term map has no expression map"]
fn rmlstartc002b() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLSTARTC002b", true)
}

#[test]
#[ignore = "RDF-star quoted triples not supported: term map has no expression map"]
fn rmlstartc003a() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLSTARTC003a", true)
}

#[test]
#[ignore = "RDF-star quoted triples not supported: term map has no expression map"]
fn rmlstartc003b() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLSTARTC003b", true)
}

#[test]
#[ignore = "RDF-star quoted triples not supported: term map has no expression map"]
fn rmlstartc004a() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLSTARTC004a", true)
}

#[test]
#[ignore = "RDF-star quoted triples not supported: term map has no expression map"]
fn rmlstartc004b() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLSTARTC004b", true)
}

#[test]
#[ignore = "RDF-star quoted triples not supported: term map has no expression map"]
fn rmlstartc005a() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLSTARTC005a", true)
}

#[test]
#[ignore = "RDF-star quoted triples not supported: term map has no expression map"]
fn rmlstartc005b() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLSTARTC005b", true)
}

#[test]
#[ignore = "RDF-star quoted triples not supported: term map has no expression map"]
fn rmlstartc006a() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLSTARTC006a", true)
}

#[test]
#[ignore = "RDF-star quoted triples not supported: term map has no expression map"]
fn rmlstartc006b() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLSTARTC006b", true)
}

#[test]
#[ignore = "RDF-star quoted triples not supported: term map has no expression map"]
fn rmlstartc007a() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLSTARTC007a", true)
}

#[test]
#[ignore = "RDF-star quoted triples not supported: term map has no expression map"]
fn rmlstartc007b() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLSTARTC007b", true)
}

#[test]
#[ignore = "RDF-star quoted triples not supported: term map has no expression map"]
fn rmlstartc008a() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLSTARTC008a", true)
}

#[test]
#[ignore = "RDF-star quoted triples not supported: term map has no expression map"]
fn rmlstartc008b() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLSTARTC008b", true)
}

#[test]
fn rmlstartc009() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLSTARTC009", false)
}

#[test]
fn rmlstartc010() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLSTARTC010", false)
}
