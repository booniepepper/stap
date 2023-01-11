const STAP_PATH: &str = std::env!("CARGO_BIN_EXE_stap");

#[test]
fn run_greet() {
    let output = stap(&["run", &example("greet")]);

    assert!(output.success);
    assert_eq!(&output.stdout, "Hello, world!\n");
    assert_eq!(&output.stderr, "");
}

#[test]
fn run_silly() {
    let output = stap(&["run", &example("silly")]);

    assert!(output.success);
    assert_eq!(&output.stdout, "y so srs\n");
    assert_eq!(&output.stderr, "");
}

#[test]
fn run_one_plus_one() {
    let output = stap(&["run", &example("one-plus-one")]);

    assert!(output.success);
    assert_eq!(&output.stdout, "2\n");
    assert_eq!(&output.stderr, "");
}

#[test]
fn run_missing_file_fails() {
    let output = stap(&["run"]);

    assert!(!output.success);
    assert_eq!(&output.stdout, "");
    assert!(output
        .stderr
        .contains("required arguments were not provided"));
}

/* ---------------- */

fn example(name: &str) -> String {
    format!("tests/example/{}.stap", name)
}

#[allow(dead_code)]
fn stap(args: &[&str]) -> Output {
    Command::new(STAP_PATH)
        .args(args)
        .output()
        .map(Output::from)
        .expect("Unable to launch stap.")
}

struct Output {
    stdout: String,
    stderr: String,
    success: bool,
}

use std::process::{Command, Output as RawOutput};

impl From<RawOutput> for Output {
    fn from(output: RawOutput) -> Self {
        Self {
            success: output.status.success(),
            stdout: String::from_utf8(output.stdout).expect("Couldn't read stdout"),
            stderr: String::from_utf8(output.stderr).expect("Couldn't read stderr"),
        }
    }
}
