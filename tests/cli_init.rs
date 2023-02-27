use assert_cmd::prelude::*;
use predicates::prelude::predicate::str::*;
use std::process::Command;

#[test]
fn test_init_flag() {
    let mut cmd = Command::cargo_bin("decision-record").unwrap();
    cmd.arg("init");

    cmd.assert().success();
}

#[test]
fn test_init_with_doc_path_flag() {
    let mut cmd = Command::cargo_bin("decision-record").unwrap();
    cmd.arg("init").arg("--doc-path").arg("docs");

    cmd.assert().success().stdout(contains("docs"));
}