use assert_cmd::prelude::*;
use predicates::prelude::predicate::str::*;
use std::process::Command;

#[test]
fn test_version_flag() {
    let mut cmd = Command::cargo_bin("decision-record").unwrap();
    cmd.arg("--version");

    cmd.assert().success().stdout(contains("0.0.1"));
}