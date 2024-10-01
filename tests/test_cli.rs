use std::{
    io::Write,
    process::{Command, Stdio},
};

#[test]
fn test_cli_with_stdin() {
    let echo = Command::new("echo")
        .arg("Some text")
        .output()
        .expect("failed to exec ls");

    assert!(echo.status.success(), "echo command failed");
    let mut cargo = Command::new("cargo")
        .arg("run")
        .arg("--")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .arg("Some text")
        .spawn()
        .expect("failed to spawn");

    {
        let stdin = cargo.stdin.as_mut().expect("failed to open stdin");
        stdin
            .write_all(echo.stdout.as_slice())
            .expect("failed to write in stdin");
    }

    let output = cargo.wait_with_output().expect("failed to get output");
    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("{}", stdout);

    assert!(stdout.contains("1"), "shouldnt be empty")
}
