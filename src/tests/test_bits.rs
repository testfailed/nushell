use crate::tests::{fail_test, run_test, TestResult};

#[test]
fn bits_and() -> TestResult {
    run_test("2 | bits and 4", "0")
}

#[test]
fn bits_and_negative() -> TestResult {
    run_test("-3 | bits and 5", "5")
}

#[test]
fn bits_and_list() -> TestResult {
    run_test("[1 2 3 8 9 10] | bits and 2 | str collect '.'", "0.2.2.0.0.2")
}

#[test]
fn bits_or() -> TestResult {
    run_test("2 | bits or 3", "3")
}

#[test]
fn bits_or_negative() -> TestResult {
    run_test("-3 | bits or 5", "-3")
}

#[test]
fn bits_or_list() -> TestResult {
    run_test("[1 2 3 8 9 10] | bits or 2 | str collect '.'", "3.2.3.10.11.10")
}

#[test]
fn bits_xor() -> TestResult {
    run_test("2 | bits xor 3", "1")
}

#[test]
fn bits_xor_negative() -> TestResult {
    run_test("-3 | bits xor 5", "-8")
}

#[test]
fn bits_xor_list() -> TestResult {
    run_test("[1 2 3 8 9 10] | bits xor 2 | str collect '.'", "3.0.1.10.11.8")
}
