//! A simple and easy-to-use library for interacting with the Android Debug Bridge (adb).
//!
//! # Features
//!
//! - set the working directory
//! - set adb environment variables
//! - call the (supported) command to execute in a chain (with or without global options)
//! - build and execute the command (provided by trait [`AdbCommand`])
//!
//! Currently, it only supports commands mentioned in [adb man page](https://android.googlesource.com/platform/packages/modules/adb/+/refs/heads/master/docs/user/adb.1.md).
//!
//! However, adbr is designed to be highly extensible, so you can easily add new commands.
//! Please don't hesitate to open an issue or a pull request if you have any suggestions or improvements!
//!
//! # Fast Start
//!
//! First, add adbr to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! adbr = "0.1"
//! ```
//!
//! Then, create an `Adb` instance:
//!
//! ```
//! use adbr::{Adb, AdbCommand};
//!
//! let adb = Adb::new().unwrap();   // adb binary is assumed to be in PATH
//! // you can also specify the working directory
//! // let adb = Adb::with_working_directory("/path/to/adb").unwrap();
//! ```
//!
//! Now, you can execute adb commands, `adb devices` for example:
//!
//! ```no_run
//! # use adbr::{Adb, AdbCommand};
//! # let adb = Adb::new().unwrap();
//! let output = adb.devices().output().unwrap();
//! println!("{}", String::from_utf8_lossy(&output.stdout));
//! ```
//!
//! A more complex example:
//!
//! `adb push --sync -z lz4 /path/to/local1 /path/to/local2 /path/to/remote`
//!
//! ```no_run
//! # use adbr::{Adb, AdbCommand};
//! # let adb = Adb::new().unwrap();
//! use adbr::command::AdbCompressionAlgorithm;
//!
//! adb.push(&["/path/to/local1", "/path/to/local2"], "/path/to/remote")
//!     .sync()
//!     .z(AdbCompressionAlgorithm::Lz4)
//!     .status()
//!     .unwrap();
//! ```
//!
//! # Environment Variables
//!
//! The adb command will be executed with the environment variables stored in the `Adb` instance.
//!
//! The initial values is determined by the constructor you use:
//!
//! - [`Adb::new`] | [`Adb::with_working_directory`]: inherits all environment variables from the current process.
//! - [`Adb::default`]: removes all adb environment variables (see [`envs`] for all adb environment variables).
//!
//! To get and modify the environment variables stored in [`Adb`],
//! you can use [`Adb::envs`] and [`Adb::envs_mut`].
//!
//! ```
//! # use std::ops::Deref;
//! # use adbr::{Adb, AdbEnvs};
//! # let mut adb = Adb::new().unwrap();
//! adb.envs_mut().set_adb_local_transport_max_port(1234);
//! assert_eq!(
//!     adb.envs().adb_local_transport_max_port(),
//!     Some(1234),
//! );
//! ```
//!
//! To get and modify the environment variables of the current process,
//! you can use [`envs::AdbEnv::get`] and [`envs::AdbEnv::set`] methods.
//!
//! ```
//! use adbr::envs::{AdbEnv, AdbLocalTransportMaxPort};
//!
//! AdbLocalTransportMaxPort(1234).set().unwrap();
//! assert_eq!(
//!     AdbLocalTransportMaxPort::get().unwrap(),
//!     AdbLocalTransportMaxPort(1234),
//! );
//! ```

pub mod command;
pub mod envs;
pub mod error;
pub mod socket;

use std::fs::canonicalize;
use std::io;
use std::path::{Path, PathBuf};

use command::AdbCommandBuilder;

pub use command::global_option::AdbGlobalOption;
pub use command::AdbCommand;
pub use envs::AdbEnvs;
pub use error::AdbError;
pub use socket::*;

/// Adb result type, where the error is [`AdbError`].
pub type AdbResult<T> = Result<T, AdbError>;

/// A wrapper around the adb binary.
/// It contains working directory and environment variables to build and execute adb commands.
///
/// See [crate level documentation](crate) for more information.
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
    pub fn new() -> AdbResult<Self> {
        Ok(Self {
            working_directory: None,
            envs: AdbEnvs::new()?,
        })
    }

    /// Creates a new `Adb` instance with the adb binary located at `path`.
    ///
    /// See [`Self::set_working_directory`] for more information.
    pub fn with_working_directory<P: AsRef<Path>>(path: P) -> AdbResult<Self> {
        let mut adb = Self::new()?;
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

    /// Gets the mutable adb environment variables.
    pub fn envs_mut(&mut self) -> &mut AdbEnvs {
        &mut self.envs
    }

    /// Creates a new [`AdbCommandBuilder`].
    fn command(&self) -> AdbCommandBuilder {
        AdbCommandBuilder::new(self)
    }
}
