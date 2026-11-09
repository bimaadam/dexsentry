use thiserror::Error;

#[derive(Debug)]
pub struct Apk;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ApkError {
    #[error("APK parsing is not implemented yet")]
    Unsupported,
}

/// Accepts raw APK bytes and reports that parsing is not implemented yet.
pub fn parse(_bytes: &[u8]) -> Result<Apk, ApkError> {
    Err(ApkError::Unsupported)
}
