use std::path::Path;

use dexsentry_core::analyze;

#[test]
fn placeholder_report_snapshot() {
    let report = match analyze(Path::new("sample.apk")) {
        Ok(report) => report,
        Err(error) => panic!("placeholder analysis failed: {error}"),
    };
    assert_eq!(report.apk.file_name.as_deref(), Some("sample.apk"));
    insta::assert_snapshot!(
        format!("{report:?}"),
        @r###"ScanReport { apk: ApkInfo { file_name: Some("sample.apk") }, findings: [], risk_score: 0 }"###
    );
}
