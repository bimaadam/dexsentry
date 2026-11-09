use std::path::Path;

use thiserror::Error;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ApkInfo {
    pub file_name: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Severity {
    Info,
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Finding {
    pub id: String,
    pub severity: Severity,
    pub title: String,
    pub evidence: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScanReport {
    pub apk: ApkInfo,
    pub findings: Vec<Finding>,
    pub risk_score: u8,
}

#[derive(Debug, Error)]
pub enum CoreError {
    #[error("analysis is not implemented yet")]
    NotImplemented,
    #[error(transparent)]
    Rules(#[from] dexsentry_rules::RuleError),
}

/// Returns an empty report while APK and DEX analysis is being scaffolded.
pub fn analyze(path: &Path) -> Result<ScanReport, CoreError> {
    tracing::debug!(path = %path.display(), "returning placeholder scan report");
    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .map(str::to_owned);

    Ok(ScanReport {
        apk: ApkInfo { file_name },
        findings: Vec::new(),
        risk_score: 0,
    })
}
