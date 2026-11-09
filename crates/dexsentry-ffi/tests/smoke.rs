use dexsentry_ffi::{analyze_apk, version};

#[test]
fn ffi_exports_smoke_test() {
    assert!(!version().is_empty());
    assert!(analyze_apk("synthetic.apk".to_owned()).is_ok());
}
