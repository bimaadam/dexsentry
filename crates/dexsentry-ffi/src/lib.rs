use std::path::Path;

use dexsentry_core::{ApkInfo, CoreError, Finding, ScanReport, Severity};
use thiserror::Error;

uniffi::setup_scaffolding!();

#[derive(Debug, uniffi::Enum)]
pub enum FfiSeverity {
    Info,
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, uniffi::Record)]
pub struct FfiApkInfo {
    pub file_name: Option<String>,
}

#[derive(Debug, uniffi::Record)]
pub struct FfiFinding {
    pub id: String,
    pub severity: FfiSeverity,
    pub title: String,
    pub evidence: Vec<String>,
}

#[derive(Debug, uniffi::Record)]
pub struct FfiScanReport {
    pub apk: FfiApkInfo,
    pub findings: Vec<FfiFinding>,
    pub risk_score: u8,
}

#[derive(Debug, Error, uniffi::Error)]
pub enum FfiError {
    #[error("analysis failed: {reason}")]
    AnalysisFailed { reason: String },
}

impl From<Severity> for FfiSeverity {
    fn from(value: Severity) -> Self {
        match value {
            Severity::Info => Self::Info,
            Severity::Low => Self::Low,
            Severity::Medium => Self::Medium,
            Severity::High => Self::High,
            Severity::Critical => Self::Critical,
        }
    }
}

impl From<ApkInfo> for FfiApkInfo {
    fn from(value: ApkInfo) -> Self {
        Self {
            file_name: value.file_name,
        }
    }
}

impl From<Finding> for FfiFinding {
    fn from(value: Finding) -> Self {
        Self {
            id: value.id,
            severity: value.severity.into(),
            title: value.title,
            evidence: value.evidence,
        }
    }
}

impl From<ScanReport> for FfiScanReport {
    fn from(value: ScanReport) -> Self {
        Self {
            apk: value.apk.into(),
            findings: value.findings.into_iter().map(FfiFinding::from).collect(),
            risk_score: value.risk_score,
        }
    }
}

impl From<CoreError> for FfiError {
    fn from(value: CoreError) -> Self {
        Self::AnalysisFailed {
            reason: value.to_string(),
        }
    }
}

#[uniffi::export]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_owned()
}

#[uniffi::export]
pub fn analyze_apk(path: String) -> Result<FfiScanReport, FfiError> {
    dexsentry_core::analyze(Path::new(&path))
        .map(FfiScanReport::from)
        .map_err(FfiError::from)
}
