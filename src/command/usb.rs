//! Usb Commands
//!
//! **Only** valid when running with libusb backend.
//!
//! - `attach SERIAL`: Attach a detached USB device identified by its `SERIAL` number.
//! - `detach SERIAL`: Detach from a USB device identified by its `SERIAL` to allow use by other processes.
//!
//! See [USB Commands](https://android.googlesource.com/platform/packages/modules/adb/+/refs/heads/master/docs/user/adb.1.md#usb).

use std::ffi::OsStr;
use std::process::Command;

use crate::command::AdbCommandBuilder;
use crate::{Adb, AdbCommand};

/// `attach SERIAL`: Attach a detached USB device identified by its `SERIAL` number.
#[derive(Debug, Clone)]
pub struct AdbAttach<'a, S: AsRef<OsStr>> {
    acb: AdbCommandBuilder<'a>,
    /// `SERIAL`: The serial number of the device to attach.
    serial: S,
}

impl<'a, S: AsRef<OsStr>> AdbAttach<'a, S> {
    /// Creates a new `AdbAttach` command.
    fn new(acb: AdbCommandBuilder<'a>, serial: S) -> Self {
        Self { acb, serial }
    }

    /// `SERIAL`: The serial number of the device to attach.
    ///
    /// The previous serial will be overwritten.
    pub fn serial<S1: AsRef<OsStr>>(self, serial: S1) -> AdbAttach<'a, S1> {
        AdbAttach {
            acb: self.acb,
            serial,
        }
    }
}

impl<'a, S: AsRef<OsStr>> AdbCommand for AdbAttach<'a, S> {
    fn build(self) -> Command {
        let mut cmd = self.acb.build();
        cmd.arg("attach");
        cmd.arg(self.serial);
        cmd
    }
}

impl Adb {
    /// `attach SERIAL`: Attach a detached USB device identified by its `SERIAL` number.
    ///
    /// # Examples
    ///
    /// `adb attach serial`
    ///
    /// ```no_run
    /// # use adbr::{Adb, AdbCommand};
    /// # let adb = Adb::new().unwrap();
    /// adb.attach("serial")
    ///     .status()
    ///     .expect("`adb attach serial` failed");
    /// ```
    pub fn attach<S: AsRef<OsStr>>(&self, serial: S) -> AdbAttach<S> {
        AdbAttach::new(self.command(), serial)
    }
}

impl<'a> AdbCommandBuilder<'a> {
    /// `attach SERIAL`: Attach a detached USB device identified by its `SERIAL` number.
    ///
    /// See [`Adb::attach`] for more information.
    pub fn attach<S: AsRef<OsStr>>(self, serial: S) -> AdbAttach<'a, S> {
        AdbAttach::new(self, serial)
    }
}

/// `detach SERIAL`: Detach from a USB device identified by its `SERIAL` to allow use by other processes.
#[derive(Debug, Clone)]
pub struct AdbDetach<'a, S: AsRef<OsStr>> {
    acb: AdbCommandBuilder<'a>,
    /// `SERIAL`: The serial number of the device to detach.
    serial: S,
}

impl<'a, S: AsRef<OsStr>> AdbDetach<'a, S> {
    /// Creates a new `AdbDetach` command.
    fn new(acb: AdbCommandBuilder<'a>, serial: S) -> Self {
        Self { acb, serial }
    }

    /// `SERIAL`: The serial number of the device to detach.
    ///
    /// The previous serial will be overwritten.
    pub fn serial<S1: AsRef<OsStr>>(self, serial: S1) -> AdbDetach<'a, S1> {
        AdbDetach {
            acb: self.acb,
            serial,
        }
    }
}

impl<'a, S: AsRef<OsStr>> AdbCommand for AdbDetach<'a, S> {
    fn build(self) -> Command {
        let mut cmd = self.acb.build();
        cmd.arg("detach");
        cmd.arg(self.serial);
        cmd
    }
}

impl Adb {
    /// `detach SERIAL`: Detach from a USB device identified by its `SERIAL` to allow use by other processes.
    ///
    /// # Examples
    ///
    /// `adb detach serial`
    ///
    /// ```no_run
    /// # use adbr::{Adb, AdbCommand};
    /// # let adb = Adb::new().unwrap();
    /// adb.detach("serial")
    ///     .status()
    ///     .expect("`adb detach serial` failed");
    /// ```
    pub fn detach<S: AsRef<OsStr>>(&self, serial: S) -> AdbDetach<S> {
        AdbDetach::new(self.command(), serial)
    }
}

impl<'a> AdbCommandBuilder<'a> {
    /// `detach SERIAL`: Detach from a USB device identified by its `SERIAL` to allow use by other processes.
    ///
    /// See [`Adb::detach`] for more information.
    pub fn detach<S: AsRef<OsStr>>(self, serial: S) -> AdbDetach<'a, S> {
        AdbDetach::new(self, serial)
    }
}
