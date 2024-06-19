//! General commands.
//!
//! - `devices [-l]`: List connected devices.
//! - `help`: Show help message.
//! - `version`: Show version number.
//!
//! See [General Commands](https://android.googlesource.com/platform/packages/modules/adb/+/refs/heads/master/docs/user/adb.1.md#general-commands).

use std::process::Command;

use crate::command::{AdbCommand, AdbCommandBuilder};
use crate::Adb;

/// `devices [-l]`: List connected devices.
/// - `-l`: Use long output.
pub struct Devices<'a> {
    acb: AdbCommandBuilder<'a>,
    /// `-l`: Use long output.
    l: bool,
}

impl<'a> Devices<'a> {
    fn new(acb: AdbCommandBuilder<'a>) -> Self {
        Self { acb, l: false }
    }

    /// `-l`: Use long output.
    pub fn l(mut self) -> Self {
        self.l = true;
        self
    }
}

impl<'a> AdbCommand for Devices<'a> {
    fn build(self) -> Command {
        let mut cmd = self.acb.build();
        cmd.arg("devices");
        if self.l {
            cmd.arg("-l");
        }
        cmd
    }
}

impl Adb {
    /// `devices [-l]`: List connected devices.
    /// - `-l`: Use long output.
    ///
    /// # Examples
    ///
    /// `adb devices -l`
    ///
    /// ```no_run
    /// # use adbr::{Adb, AdbCommand};
    /// # let adb = Adb::new();
    /// adb.devices()
    ///     .l()
    ///     .status()
    ///     .expect("`adb devices -l` failed");
    /// ```
    pub fn devices(&self) -> Devices {
        Devices::new(self.command())
    }
}

impl<'a> AdbCommandBuilder<'a> {
    /// `devices [-l]`: List connected devices.
    /// - `-l`: Use long output.
    ///
    /// See [`Adb::devices`] for more information.
    pub fn devices(self) -> Devices<'a> {
        Devices::new(self)
    }
}

/// `help`: Show help message.
pub struct Help<'a>(AdbCommandBuilder<'a>);

impl<'a> AdbCommand for Help<'a> {
    fn build(self) -> Command {
        let mut cmd = self.0.build();
        cmd.arg("help");
        cmd
    }
}

impl Adb {
    /// `help`: Show help message.
    ///
    /// # Examples
    ///
    /// `adb help`
    ///
    /// ```no_run
    /// # use adbr::{Adb, AdbCommand};
    /// # let adb = Adb::new();
    /// adb.help().status().expect("`adb help` failed");
    /// ```
    pub fn help(&self) -> Help {
        Help(self.command())
    }
}

impl<'a> AdbCommandBuilder<'a> {
    /// `help`: Show help message.
    ///
    /// See [`Adb::help`] for more information.
    pub fn help(self) -> Help<'a> {
        Help(self)
    }
}

/// `version`: Show version number.
pub struct Version<'a>(AdbCommandBuilder<'a>);

impl<'a> AdbCommand for Version<'a> {
    fn build(self) -> Command {
        let mut cmd = self.0.build();
        cmd.arg("version");
        cmd
    }
}

impl Adb {
    /// `version`: Show version number.
    ///
    /// # Examples
    ///
    /// `adb version`
    ///
    /// ```no_run
    /// # use adbr::{Adb, AdbCommand};
    /// # let adb = Adb::new();
    /// adb.version().status().expect("`adb version` failed");
    /// ```
    pub fn version(&self) -> Version {
        Version(self.command())
    }
}

impl<'a> AdbCommandBuilder<'a> {
    /// `version`: Show version number.
    ///
    /// See [`Adb::version`] for more information.
    pub fn version(self) -> Version<'a> {
        Version(self)
    }
}
