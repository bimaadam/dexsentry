use dexsentry_dex::{DexError, parse};
use proptest::prelude::*;

#[test]
fn parser_returns_a_typed_placeholder_error() {
    assert!(matches!(
        parse(b"synthetic dex"),
        Err(DexError::Unsupported)
    ));
}

proptest! {
    #[test]
    fn parser_never_accepts_unimplemented_input(bytes in prop::collection::vec(any::<u8>(), 0..256)) {
        prop_assert!(matches!(parse(&bytes), Err(DexError::Unsupported)));
    }
}
