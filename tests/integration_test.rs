use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_help_works() {
    Command::cargo_bin("git-shade")
        .unwrap()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("git-shade"));
}
