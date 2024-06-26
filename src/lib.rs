//! A Rust library for interacting with the Android Debug Bridge (adb).

pub mod command;
pub mod envs;
pub mod error;
pub mod global_option;
pub mod socket;

use std::fs::canonicalize;
use std::io;
use std::path::{Path, PathBuf};

use crate::command::AdbCommandBuilder;

pub use crate::command::AdbCommand;
pub use crate::envs::AdbEnvs;
pub use crate::error::AdbError;
pub use crate::global_option::AdbGlobalOption;
pub use crate::socket::*;

/// Adb result type, where the error is [`AdbError`].
pub type AdbResult<T> = Result<T, AdbError>;

/// A wrapper around the adb binary.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Adb {
    /// The canonical directory where the adb binary is located.
    /// If None, the adb binary should be in `PATH`.
    working_directory: Option<PathBuf>,
    /// Adb environment variables.
    envs: AdbEnvs,
}

impl Adb {
    /// Creates a new [`Adb`] instance.
    ///
    /// The adb binary is assumed to be in `PATH`.
    pub fn new() -> Self {
        Self {
            working_directory: None,
            envs: AdbEnvs::default(),
        }
    }

    /// Creates a new `Adb` instance with the adb binary located at `path`.
    ///
    /// See [`Self::set_working_directory`] for more information.
    pub fn with_working_directory<P: AsRef<Path>>(path: P) -> AdbResult<Self> {
        let mut adb = Self::new();
        adb.set_working_directory(path)?;
        Ok(adb)
    }

    /// The canonical directory where the adb binary is located.
    ///
    /// If [`None`], the adb binary will be searched in an OS-defined way in `PATH`.
    pub fn working_directory(&self) -> Option<&Path> {
        self.working_directory.as_deref()
    }

    /// Sets the directory where the adb binary is located.
    ///
    /// The input `path` will be canonicalized.
    ///
    /// # Note
    ///
    /// This method doesn't check if the adb binary is actually in `path`,
    /// resulting in a `NotFound` error when running adb commands.
    ///
    /// # Errors
    ///
    /// Including but not limited to:
    ///
    /// - `path` doesn't exist.
    /// - `path` isn't a directory.
    pub fn set_working_directory<P: AsRef<Path>>(&mut self, path: P) -> AdbResult<&mut Self> {
        let dir = canonicalize(path)?;
        if dir.is_dir() {
            self.working_directory = Some(dir);
            Ok(self)
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("not a directory: {}", dir.display()),
            )
            .into())
        }
    }

    /// Resets the working directory.
    ///
    /// The adb binary will be searched in an OS-defined way in `PATH`.
    pub fn reset_working_directory(&mut self) -> &mut Self {
        self.working_directory = None;
        self
    }

    /// Gets the adb environment variables.
    pub fn envs(&self) -> &AdbEnvs {
        &self.envs
    }

    /// Creates a new [`AdbCommandBuilder`].
    fn command(&self) -> AdbCommandBuilder {
        AdbCommandBuilder::new(self)
    }
}
