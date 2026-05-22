use std::process::Command;

fn run_vt(args: &[&str]) -> std::process::Output {
    let vt_bin = env!("CARGO_BIN_EXE_vt");
    Command::new(vt_bin)
        .args(args)
        .output()
        .expect("failed to execute vt binary")
}

fn run_vt_debug(fixture: &str) -> String {
    let fixture_path = format!("tests/fixtures/{}", fixture);
    let output = run_vt(&["compile", "--debug", &fixture_path]);
    String::from_utf8_lossy(&output.stdout).into_owned()
}

fn run_vt_normal(fixture: &str) -> String {
    let fixture_path = format!("tests/fixtures/{}", fixture);
    let output = run_vt(&["compile", &fixture_path]);
    String::from_utf8_lossy(&output.stdout).into_owned()
}

// ── Debug mode snapshots ──────────────────────────────────────────

#[test]
fn debug_sample() {
    insta::assert_snapshot!(run_vt_debug("sample.vt"));
}

#[test]
fn debug_empty() {
    insta::assert_snapshot!(run_vt_debug("empty.vt"));
}

#[test]
fn debug_multi_rule() {
    insta::assert_snapshot!(run_vt_debug("multi_rule.vt"));
}

// ── Non-debug mode: output unchanged ──────────────────────────────

#[test]
fn normal_sample_matches_debug_output() {
    let output = run_vt_normal("sample.vt");
    // Should contain the Debug representation of Program, not the debug trace
    assert!(
        output.contains("Program"),
        "non-debug output should contain AST Debug"
    );
    assert!(
        !output.contains("FASE 1"),
        "non-debug output should NOT contain debug trace"
    );
}

#[test]
fn normal_empty_has_parse_error() {
    let fixture_path = "tests/fixtures/empty.vt";
    let output = run_vt(&["compile", fixture_path]);
    assert!(
        !output.status.success(),
        "empty file should fail compilation"
    );
}
