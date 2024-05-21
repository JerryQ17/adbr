use std::io;

use thiserror::Error;

/// Adb errors.
#[derive(Debug, Error)]
pub enum AdbError {
    /// IO error.
    #[error(transparent)]
    Io(#[from] io::Error),
}
