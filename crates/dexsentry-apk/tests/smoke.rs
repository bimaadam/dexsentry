use dexsentry_apk::{ApkError, parse};

#[test]
fn parser_returns_a_typed_placeholder_error() {
    assert!(matches!(
        parse(b"synthetic apk"),
        Err(ApkError::Unsupported)
    ));
}
