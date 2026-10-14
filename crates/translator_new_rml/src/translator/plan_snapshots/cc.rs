use crate::translator::plan_snapshots::TestExecutor;
use std::error::Error;

const TEST_DIR: &str = "rml-cc-tests";

#[test]
#[ignore = "rml:gather not supported: Predicate Object Map has 0 object maps"]
fn rmltc_cc_0001_alt() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC-CC-0001-Alt", true)
}

#[test]
#[ignore = "rml:gather not supported: Predicate Object Map has 0 object maps"]
fn rmltc_cc_0001_bag() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC-CC-0001-Bag", true)
}

#[test]
#[ignore = "rml:gather not supported: Predicate Object Map has 0 object maps"]
fn rmltc_cc_0001_list() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC-CC-0001-List", true)
}

#[test]
#[ignore = "rml:gather not supported: Predicate Object Map has 0 object maps"]
fn rmltc_cc_0001_seq() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC-CC-0001-Seq", true)
}

#[test]
fn rmltc_cc_0002_bag() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC-CC-0002-Bag", true)
}

#[test]
fn rmltc_cc_0002_list() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC-CC-0002-List", true)
}

#[test]
#[ignore = "rml:gather not supported: Predicate Object Map has 0 object maps"]
fn rmltc_cc_0003_eb() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC-CC-0003-EB", true)
}

#[test]
#[ignore = "rml:gather not supported: Predicate Object Map has 0 object maps"]
fn rmltc_cc_0003_el() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC-CC-0003-EL", true)
}

#[test]
fn rmltc_cc_0003_el_bn() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC-CC-0003-EL-BN", true)
}

#[test]
fn rmltc_cc_0003_el_named() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC-CC-0003-EL-Named", true)
}

#[test]
#[ignore = "rml:gather not supported: Predicate Object Map has 0 object maps"]
fn rmltc_cc_0003_neb() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC-CC-0003-NEB", true)
}

#[test]
#[ignore = "rml:gather not supported: Predicate Object Map has 0 object maps"]
fn rmltc_cc_0003_nel() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC-CC-0003-NEL", true)
}

#[test]
#[ignore = "rml:gather not supported: Predicate Object Map has 0 object maps"]
fn rmltc_cc_0003_nelb() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC-CC-0003-NELb", true)
}

#[test]
#[ignore = "rml:gather in subject map not supported: term map has no expression map"]
fn rmltc_cc_0004_sm1() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC-CC-0004-SM1", true)
}

#[test]
#[ignore = "rml:gather in subject map not supported: term map has no expression map"]
fn rmltc_cc_0004_sm2() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC-CC-0004-SM2", true)
}

#[test]
#[ignore = "rml:gather in subject map not supported: term map has no expression map"]
fn rmltc_cc_0004_sm3() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC-CC-0004-SM3", true)
}

#[test]
#[ignore = "rml:gather in subject map not supported: term map has no expression map"]
fn rmltc_cc_0004_sm4() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC-CC-0004-SM4", true)
}

#[test]
#[ignore = "rml:gather in subject map not supported: term map has no expression map"]
fn rmltc_cc_0004_sm5() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC-CC-0004-SM5", true)
}

#[test]
#[ignore = "rml:gather not supported: Predicate Object Map has 0 object maps"]
fn rmltc_cc_0005_app1() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC-CC-0005-App1", true)
}

#[test]
#[ignore = "rml:gather not supported: Predicate Object Map has 0 object maps"]
fn rmltc_cc_0005_app2() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC-CC-0005-App2", true)
}

#[test]
#[ignore = "rml:gather not supported: Predicate Object Map has 0 object maps"]
fn rmltc_cc_0005_car1() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC-CC-0005-Car1", true)
}

#[test]
#[ignore = "rml:gather not supported: Predicate Object Map has 0 object maps"]
fn rmltc_cc_0005_car2() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC-CC-0005-Car2", true)
}

#[test]
#[ignore = "rml:gather not supported: Predicate Object Map has 0 object maps"]
fn rmltc_cc_0006_it0() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC-CC-0006-IT0", true)
}

#[test]
fn rmltc_cc_0006_it1() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC-CC-0006-IT1", true)
}

#[test]
fn rmltc_cc_0006_it2() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC-CC-0006-IT2", true)
}

#[test]
#[ignore = "rml:gather not supported: Predicate Object Map has 0 object maps"]
fn rmltc_cc_0006_it3() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC-CC-0006-IT3", true)
}

#[test]
fn rmltc_cc_0006_it4() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC-CC-0006-IT4", true)
}

#[test]
fn rmltc_cc_0006_it5() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC-CC-0006-IT5", true)
}

#[test]
#[ignore = "rml:gather not supported: Predicate Object Map has 0 object maps"]
fn rmltc_cc_0007_nes() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC-CC-0007-NES", true)
}

#[test]
fn rmltc_cc_0008_roma() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC-CC-0008-ROMa", true)
}

#[test]
fn rmltc_cc_0008_romb() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC-CC-0008-ROMb", true)
}

#[test]
#[ignore = "rml:gather not supported: Predicate Object Map has 0 object maps"]
fn rmltc_cc_0009_dup_bag() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC-CC-0009-DUP-Bag", true)
}

#[test]
#[ignore = "rml:gather not supported: Predicate Object Map has 0 object maps"]
fn rmltc_cc_0009_dup_list() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC-CC-0009-DUP-List", true)
}

#[test]
#[ignore = "rml:gather not supported: Predicate Object Map has 0 object maps"]
fn rmltc_cc_0010_lista() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC-CC-0010-Lista", true)
}

#[test]
fn rmltc_cc_0010_listb() -> Result<(), Box<dyn Error>> {
    TestExecutor::new(TEST_DIR).execute("RMLTC-CC-0010-Listb", true)
}
