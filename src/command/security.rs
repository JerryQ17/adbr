//! Security commands.
//!
//! - `disable-verity`: Disable dm-verity checking on userdebug builds.
//! - `enable-verity`: Re-enable dm-verity checking on userdebug builds.
//! - `keygen FILE`: Generate adb public/private key; private key stored in `FILE`.
//!
//! See [Security Commands](https://android.googlesource.com/platform/packages/modules/adb/+/refs/heads/master/docs/user/adb.1.md#security).

use std::ffi::OsStr;
use std::process::Command;

use crate::command::AdbCommandBuilder;
use crate::{Adb, AdbCommand};

/// `disable-verity`: Disable dm-verity checking on userdebug builds.
#[derive(Debug, Clone)]
pub struct AdbDisableVerity<'a>(AdbCommandBuilder<'a>);

impl<'a> AdbCommand for AdbDisableVerity<'a> {
    fn build(self) -> Command {
        let mut cmd = self.0.build();
        cmd.arg("disable-verity");
        cmd
    }
}

impl Adb {
    /// `disable-verity`: Disable dm-verity checking on userdebug builds.
    ///
    /// # Examples
    ///
    /// `adb disable-verity`
    ///
    /// ```no_run
    /// # use adbr::{Adb, AdbCommand};
    /// # let adb = Adb::new();
    /// adb.disable_verity()
    ///     .status()
    ///     .expect("`adb disable-verity` failed");
    /// ```
    pub fn disable_verity(&self) -> AdbDisableVerity {
        AdbDisableVerity(self.command())
    }
}

impl<'a> AdbCommandBuilder<'a> {
    /// `disable-verity`: Disable dm-verity checking on userdebug builds.
    ///
    /// See [`Adb::disable_verity`] for more information.
    pub fn disable_verity(self) -> AdbDisableVerity<'a> {
        AdbDisableVerity(self)
    }
}

/// `enable-verity`: Re-enable dm-verity checking on userdebug builds.
#[derive(Debug, Clone)]
pub struct AdbEnableVerity<'a>(AdbCommandBuilder<'a>);

impl<'a> AdbCommand for AdbEnableVerity<'a> {
    fn build(self) -> Command {
        let mut cmd = self.0.build();
        cmd.arg("enable-verity");
        cmd
    }
}

impl Adb {
    /// `enable-verity`: Re-enable dm-verity checking on userdebug builds.
    ///
    /// # Examples
    ///
    /// `adb enable-verity`
    ///
    /// ```no_run
    /// # use adbr::{Adb, AdbCommand};
    /// # let adb = Adb::new();
    /// adb.enable_verity()
    ///     .status()
    ///     .expect("`adb enable-verity` failed");
    /// ```
    pub fn enable_verity(&self) -> AdbEnableVerity {
        AdbEnableVerity(self.command())
    }
}

impl<'a> AdbCommandBuilder<'a> {
    /// `enable-verity`: Re-enable dm-verity checking on userdebug builds.
    ///
    /// See [`Adb::enable_verity`] for more information.
    pub fn enable_verity(self) -> AdbEnableVerity<'a> {
        AdbEnableVerity(self)
    }
}

/// `keygen FILE`: Generate adb public/private key; private key stored in `FILE`.
#[derive(Debug, Clone)]
pub struct AdbKeygen<'a, S: AsRef<OsStr>> {
    acb: AdbCommandBuilder<'a>,
    file: S,
}

impl<'a, S: AsRef<OsStr>> AdbKeygen<'a, S> {
    fn new(acb: AdbCommandBuilder<'a>, file: S) -> Self {
        Self { acb, file }
    }

    /// `FILE`: the file where the private key will be stored.
    ///
    /// The previous file will be overwritten.
    pub fn file<S1: AsRef<OsStr>>(self, file: S1) -> AdbKeygen<'a, S1> {
        AdbKeygen::new(self.acb, file)
    }
}

impl<'a, S: AsRef<OsStr>> AdbCommand for AdbKeygen<'a, S> {
    fn build(self) -> Command {
        let mut cmd = self.acb.build();
        cmd.arg("keygen").arg(self.file);
        cmd
    }
}

impl Adb {
    /// `keygen FILE`: Generate adb public/private key; private key stored in `FILE`.
    ///
    /// # Examples
    ///
    /// `adb keygen /path/to/private/key`
    ///
    /// ```no_run
    /// # use adbr::{Adb, AdbCommand};
    /// # let adb = Adb::new();
    /// adb.keygen("/path/to/private/key")
    ///     .status()
    ///     .expect("`adb keygen /path/to/private/key` failed");
    /// ```
    pub fn keygen<S: AsRef<OsStr>>(&self, file: S) -> AdbKeygen<S> {
        AdbKeygen::new(self.command(), file)
    }
}

impl<'a> AdbCommandBuilder<'a> {
    /// `keygen FILE`: Generate adb public/private key; private key stored in `FILE`.
    ///
    /// See [`Adb::keygen`] for more information.
    pub fn keygen<S: AsRef<OsStr>>(self, file: S) -> AdbKeygen<'a, S> {
        AdbKeygen::new(self, file)
    }
}
