use vt_parser::parse;

#[test]
fn valid_example_full() {
    let src = include_str!("fixtures/valid/example_full.vt");
    let result = parse(src);
    assert!(
        result.is_ok(),
        "expected successful parse: {:?}",
        result.err()
    );
    insta::assert_debug_snapshot!(result.unwrap());
}

#[test]
fn valid_minimal_no_mode() {
    let src = include_str!("fixtures/valid/minimal_no_mode.vt");
    let result = parse(src);
    assert!(
        result.is_ok(),
        "expected successful parse: {:?}",
        result.err()
    );
    let program = result.unwrap();
    assert!(program.mode.is_none(), "mode should be None");
    assert_eq!(program.rules.len(), 1);
}

#[test]
fn valid_all_comparators() {
    let src = include_str!("fixtures/valid/all_comparators.vt");
    let result = parse(src);
    assert!(
        result.is_ok(),
        "expected successful parse: {:?}",
        result.err()
    );
    let program = result.unwrap();
    assert_eq!(program.rules.len(), 5);
}

#[test]
fn valid_all_units() {
    let src = include_str!("fixtures/valid/all_units.vt");
    let result = parse(src);
    assert!(
        result.is_ok(),
        "expected successful parse: {:?}",
        result.err()
    );
    let program = result.unwrap();
    assert_eq!(program.rules.len(), 4);
}

#[test]
fn valid_whitespace_comments() {
    let src = include_str!("fixtures/valid/whitespace_comments.vt");
    let result = parse(src);
    assert!(
        result.is_ok(),
        "expected successful parse: {:?}",
        result.err()
    );
    let program = result.unwrap();
    assert_eq!(program.rules.len(), 2);
}

#[test]
fn invalid_missing_brace() {
    let src = include_str!("fixtures/invalid/missing_brace.vt");
    assert!(parse(src).is_err());
}

#[test]
fn invalid_unknown_key() {
    let src = include_str!("fixtures/invalid/unknown_key.vt");
    assert!(parse(src).is_err());
}

#[test]
fn invalid_missing_value() {
    let src = include_str!("fixtures/invalid/missing_value.vt");
    assert!(parse(src).is_err());
}

#[test]
fn invalid_empty() {
    let src = include_str!("fixtures/invalid/empty.vt");
    assert!(parse(src).is_err());
}
