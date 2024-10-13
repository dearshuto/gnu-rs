use assert_cmd::Command;

#[test]
fn no_arguments() {
    let mut command = Command::cargo_bin("echo-rs").unwrap();
    command.assert().failure();
}

#[test]
fn hello_world() {
    let mut command = Command::cargo_bin("echo-rs").unwrap();

    let expected = "Hello World!\n";
    command
        .arg("Hello World!")
        .assert()
        .success()
        .stdout(expected);
}
