use dexsentry_apk::Apk;
use dexsentry_dex::Dex;
use serde::Deserialize;
use thiserror::Error;

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Info,
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct Rule {
    pub id: String,
    pub title: String,
    pub severity: Severity,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Finding {
    pub rule_id: String,
    pub severity: Severity,
    pub title: String,
}

#[derive(Debug, Error)]
pub enum RuleError {
    #[error("rule evaluation is not implemented yet")]
    NotImplemented,
    #[error("invalid rule data: {0}")]
    InvalidRule(#[from] toml::de::Error),
}

pub fn parse_rule(source: &str) -> Result<Rule, RuleError> {
    toml::from_str(source).map_err(RuleError::from)
}

/// Placeholder for future rules evaluated against parsed APK and DEX data.
pub fn evaluate(_apk: &Apk, _dex: &Dex) -> Result<Vec<Finding>, RuleError> {
    Err(RuleError::NotImplemented)
}

/// Placeholder risk score. Scoring remains unavailable until rule evaluation exists.
pub fn risk_score(_findings: &[Finding]) -> Result<u8, RuleError> {
    Err(RuleError::NotImplemented)
}
