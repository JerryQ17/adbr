use std::error::Error;
use std::fmt::Display;
use std::io;

use thiserror::Error;

/// Adb errors.
#[derive(Debug, Error)]
pub enum AdbError {
    /// IO error.
    #[error(transparent)]
    Io(#[from] io::Error),
    /// Parse error.
    #[error(transparent)]
    Parse(ParseError),
}

/// Information about a parse failure.
/// 
/// `description` and `source` are optional.
#[derive(Debug, Error)]
pub struct ParseError {
    /// The value that failed to parse.
    pub value: String,
    /// The target type.
    pub target: &'static str,
    /// An optional description of the error, may be empty.
    pub description: &'static str,
    /// An optional source of the error.
    pub source: Option<Box<dyn Error + Send + Sync>>,
}

impl ParseError {
    /// Creates a new `ParseError` with a description.
    pub fn with_description<T: ToString>(
        value: T,
        target: &'static str,
        description: &'static str,
    ) -> Self {
        Self {
            value: value.to_string(),
            target,
            description,
            source: None,
        }
    }

    /// Creates a new `ParseError` with a source.
    pub fn with_source<T, E>(value: T, target: &'static str, source: E) -> Self
    where
        T: ToString,
        E: Error + Send + Sync + 'static,
    {
        Self {
            value: value.to_string(),
            target,
            description: "",
            source: Some(Box::new(source)),
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to parse `{}` to `{}`", self.value, self.target)?;
        if !self.description.is_empty() {
            write!(f, ": {}", self.description)?;
        }
        if let Some(source) = &self.source {
            write!(f, ". source: {}", source)?;
        }
        Ok(())
    }
}

impl From<ParseError> for AdbError {
    fn from(err: ParseError) -> Self {
        Self::Parse(err)
    }
}
