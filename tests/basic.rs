const STAP_PATH: &str = std::env!("CARGO_BIN_EXE_stap");

#[test]
fn no_args_fails() {
    let output = stap(&[]);

    assert!(!output.success);
    assert_eq!(&output.stdout, "");
    assert_eq!(&output.stderr, "Usage: stap FILE\n");
}

#[test]
fn silly() {
    let output = stap(&["tests/silly.stap"]);

    assert!(output.success);
    assert_eq!(&output.stdout, "");
    assert_eq!(&output.stderr, "Usage: stap FILE\n");
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
