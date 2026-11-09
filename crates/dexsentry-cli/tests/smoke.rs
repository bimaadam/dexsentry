use std::process::Command;

#[test]
fn cli_scan_command_smoke_test() {
    let status = Command::new(env!("CARGO_BIN_EXE_dexsentry"))
        .args(["scan", "synthetic.apk"])
        .status();
    assert!(matches!(status, Ok(status) if status.success()));
}
