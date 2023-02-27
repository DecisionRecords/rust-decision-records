use assert_cmd::prelude::*;
use predicates::prelude::predicate::str::*;
use std::process::Command;

#[test]
fn test_missing_new_flag() {
    let mut cmd = Command::cargo_bin("decision-record").unwrap();
    cmd.arg("new");

    cmd.assert().failure().stderr(contains("the following required arguments were not provided"));
}