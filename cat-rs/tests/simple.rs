use std::path::Path;

use assert_cmd::Command;

#[test]
fn no_arguments() {
    let mut command = Command::cargo_bin("cat-rs").unwrap();
    command.assert().failure();
}

#[test]
fn empty() {
    test_helper("tests/empty.txt", include_str!("empty.txt"));
}

#[test]
fn numbers() {
    test_helper("tests/numbers.txt", include_str!("numbers.txt"));
}

#[test]
fn numbers_multilines() {
    test_helper(
        "tests/numbers_multilines.txt",
        include_str!("numbers_multilines.txt"),
    );
}

fn test_helper<T>(path: T, expected: &'static str)
where
    T: AsRef<Path>,
{
    let mut command = Command::cargo_bin("cat-rs").unwrap();
    command
        .arg(path.as_ref())
        .assert()
        .stdout(expected)
        .success();
}
