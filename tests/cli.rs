use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn no_such_files() {
    let mut cmd = Command::cargo_bin("difft").unwrap();

    cmd.arg("no_such_file").arg("no_such_file_either");
    cmd.assert().failure().code(2);
}

#[test]
fn has_changes_default_exit_code() {
    let mut cmd = Command::cargo_bin("difft").unwrap();

    cmd.arg("sample_files/simple_before.js").arg("sample_files/simple_after.js");
    cmd.assert().success();
}

#[test]
fn has_changes_requested_exit_code() {
    let mut cmd = Command::cargo_bin("difft").unwrap();

    cmd.arg("--exit-code").arg("sample_files/simple_before.js").arg("sample_files/simple_after.js");
    cmd.assert().failure().code(1);
}