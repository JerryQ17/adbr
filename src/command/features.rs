//! Feature Commands
//!
//! - `host-features`: list features supported by adb server.
//! - `features`: list features supported by both adb server and device.
//!
//! See [Feature Commands](https://android.googlesource.com/platform/packages/modules/adb/+/refs/heads/master/docs/user/adb.1.md#features).

use std::process::Command;

use crate::command::AdbCommandBuilder;
use crate::{Adb, AdbCommand};

/// `host-features`: List features supported by adb server.
#[derive(Debug, Clone)]
pub struct AdbHostFeatures<'a>(AdbCommandBuilder<'a>);

impl<'a> AdbCommand for AdbHostFeatures<'a> {
    fn build(self) -> Command {
        let mut cmd = self.0.build();
        cmd.arg("host-features");
        cmd
    }
}

impl Adb {
    /// `host-features`: List features supported by adb server.
    ///
    /// # Examples
    ///
    /// `adb host-features`
    ///
    /// ```no_run
    /// # use adbr::{Adb, AdbCommand};
    /// # let adb = Adb::new();
    /// adb.host_features()
    ///     .status()
    ///     .expect("`adb host-features` failed");
    /// ```
    pub fn host_features(&self) -> AdbHostFeatures {
        AdbHostFeatures(self.command())
    }
}

impl<'a> AdbCommandBuilder<'a> {
    /// `host-features`: List features supported by adb server.
    ///
    /// See [`Adb::host_features`] for more information.
    pub fn host_features(self) -> AdbHostFeatures<'a> {
        AdbHostFeatures(self)
    }
}

/// `features`: List features supported by both adb server and device.
#[derive(Debug, Clone)]
pub struct AdbFeatures<'a>(AdbCommandBuilder<'a>);

impl<'a> AdbCommand for AdbFeatures<'a> {
    fn build(self) -> Command {
        let mut cmd = self.0.build();
        cmd.arg("features");
        cmd
    }
}

impl Adb {
    /// `features`: List features supported by both adb server and device.
    ///
    /// # Examples
    ///
    /// `adb features`
    ///
    /// ```no_run
    /// # use adbr::{Adb, AdbCommand};
    /// # let adb = Adb::new();
    /// adb.features()
    ///     .status()
    ///     .expect("`adb features` failed");
    /// ```
    pub fn features(&self) -> AdbFeatures {
        AdbFeatures(self.command())
    }
}

impl<'a> AdbCommandBuilder<'a> {
    /// `features`: List features supported by both adb server and device.
    ///
    /// See [`Adb::features`] for more information.
    pub fn features(self) -> AdbFeatures<'a> {
        AdbFeatures(self)
    }
}
