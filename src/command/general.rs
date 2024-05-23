//! General commands.
//!
//! See [General Commands](https://android.googlesource.com/platform/packages/modules/adb/+/refs/heads/master/docs/user/adb.1.md#general-commands).

use std::process::Command;

use crate::command::{AdbCommand, AdbCommandBuilder};
use crate::Adb;

/// `devices [-l]`: List connected devices.
///
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
    ///
    /// - `-l`: Use long output.
    pub fn devices(&self) -> Devices {
        Devices::new(self.command())
    }
}

impl<'a> AdbCommandBuilder<'a> {
    /// `devices [-l]`: List connected devices.
    ///
    /// - `-l`: Use long output.
    pub fn devices(self) -> Devices<'a> {
        Devices::new(self)
    }
}

/// `help`: Show help message.
pub struct Help<'a>(AdbCommandBuilder<'a>);

impl<'a> Help<'a> {
    fn new(acb: AdbCommandBuilder<'a>) -> Self {
        Self(acb)
    }
}

impl<'a> AdbCommand for Help<'a> {
    fn build(self) -> Command {
        let mut cmd = self.0.build();
        cmd.arg("help");
        cmd
    }
}

impl Adb {
    /// `help`: Show help message.
    pub fn help(&self) -> Help {
        Help::new(self.command())
    }
}

impl<'a> AdbCommandBuilder<'a> {
    /// `help`: Show help message.
    pub fn help(self) -> Help<'a> {
        Help::new(self)
    }
}

/// `version`: Show version number.
pub struct Version<'a>(AdbCommandBuilder<'a>);

impl<'a> Version<'a> {
    fn new(acb: AdbCommandBuilder<'a>) -> Self {
        Self(acb)
    }
}

impl<'a> AdbCommand for Version<'a> {
    fn build(self) -> Command {
        let mut cmd = self.0.build();
        cmd.arg("version");
        cmd
    }
}

impl Adb {
    /// `version`: Show version number.
    pub fn version(&self) -> Version {
        Version::new(self.command())
    }
}

impl<'a> AdbCommandBuilder<'a> {
    /// `version`: Show version number.
    pub fn version(self) -> Version<'a> {
        Version::new(self)
    }
}
