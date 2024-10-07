use std::{
    error::Error,
    fs::{self, File},
    io::Write,
    process::{Command, Stdio},
};

use tempfile::{NamedTempFile, TempDir};

fn test_cli(data: String, contains: &str, additional_args: Option<&str>) -> bool {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("T")
        .arg(data)
        .arg(additional_args.unwrap_or("-e some"))
        .output()
        .expect("failed to run cargo run");
    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("{stdout}");
    stdout.contains(contains)
}
fn create_temp_file(content: &str, temp_dir: &TempDir) -> Result<(), Box<dyn Error>> {
    let name = format!("temp_file_{content}.txt");
    let file_path = temp_dir.path().join(name);
    let mut file = File::create(file_path)?;
    writeln!(file, "{}", content)?;
    Ok(())
}

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

    assert!(stdout.contains("1"), "shouldnt be empty")
}

#[test]
fn test_cli_with_file() {
    let mut temp_file = NamedTempFile::new().expect("failed to create temp file: ");
    writeln!(temp_file, "Thats good").expect("failed to write in file");
    {
        let path_to_temp = temp_file.path().display().to_string();
        let text_from_file = fs::read_to_string(&path_to_temp).expect("failed to read file text");
        assert_eq!("Thats good", text_from_file.trim());
    }
    let path = temp_file.path().display().to_string();
    assert!(test_cli(path, "count of matches: 1", None))
}

#[test]
fn test_cli_with_dir() {
    let temp_dir = TempDir::new().expect("failed to create temp dir");
    let _ = create_temp_file("Thats good", &temp_dir);
    let _ = create_temp_file("Thats good1", &temp_dir);
    let _ = create_temp_file("Not bad", &temp_dir);
    let path = format!("{}/", temp_dir.path().display());
    assert!(test_cli(path, "count of matches: 2", None))
}

#[test]
fn test_cli_with_excludes() {
    let temp_dir = TempDir::new().expect("failed to create temp dir");
    let _ = create_temp_file("Thats good", &temp_dir);
    let _ = create_temp_file("Thats not good", &temp_dir);
    let _ = create_temp_file("Thats very good", &temp_dir);
    let path = format!("{}/", temp_dir.path().display());
    assert!(test_cli(path, "count of matches: 2", Some("-e very")))
}
