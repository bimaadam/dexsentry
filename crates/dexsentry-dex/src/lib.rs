use thiserror::Error;

#[derive(Debug)]
pub struct Dex;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DexError {
    #[error("DEX parsing is not implemented yet")]
    Unsupported,
}

/// Accepts raw DEX bytes and reports that parsing is not implemented yet.
pub fn parse(_bytes: &[u8]) -> Result<Dex, DexError> {
    Err(DexError::Unsupported)
}
