//! Debugging commands.
//!
//! - `bugreport [PATH]`: Write bugreport to given PATH (default=`bugreport.zip`);
//!     if `PATH` is a directory, the bug report is saved in that directory.
//!     devices that don't support zipped bug reports output to stdout.
//! - `jdwp`: List pids of processes hosting a JDWP transport.
//! - `logcat`: Show device log.
//!
//! See [Debugging Commands](https://android.googlesource.com/platform/packages/modules/adb/+/refs/heads/master/docs/user/adb.1.md#debugging).

use std::ffi::{OsStr, OsString};
use std::process::Command;

use crate::command::AdbCommandBuilder;
use crate::{Adb, AdbCommand};

/// `bugreport [PATH]`: Write bugreport to given PATH (default=`bugreport.zip`).
///
/// If `PATH` is a directory, the bug report is saved in that directory.
/// Devices that don't support zipped bug reports output to stdout.
#[derive(Debug, Clone)]
pub struct AdbBugReport<'a, S: AsRef<OsStr>> {
    acb: AdbCommandBuilder<'a>,
    /// `PATH`: The path to write the bugreport to.
    path: Option<S>,
}

impl<'a, S: AsRef<OsStr>> AdbBugReport<'a, S> {
    /// Creates a new `AdbBugReport` command.
    fn new(acb: AdbCommandBuilder<'a>) -> Self {
        Self { acb, path: None }
    }

    /// `PATH`: The path to write the bugreport to.
    ///
    /// The previous path will be overwritten.
    pub fn path<S1: AsRef<OsStr>>(self, path: S1) -> AdbBugReport<'a, S1> {
        AdbBugReport {
            acb: self.acb,
            path: Some(path),
        }
    }
}

impl<'a, S: AsRef<OsStr>> AdbCommand for AdbBugReport<'a, S> {
    fn build(self) -> Command {
        let mut cmd = self.acb.build();
        cmd.arg("bugreport");
        if let Some(path) = self.path {
            cmd.arg(path);
        }
        cmd
    }
}

impl Adb {
    /// `bugreport [PATH]`: Write bugreport to given PATH (default=`bugreport.zip`).
    ///
    /// If `PATH` is a directory, the bug report is saved in that directory.
    /// Devices that don't support zipped bug reports output to stdout.
    ///
    /// # Examples
    ///
    /// `adb bugreport /path/to/bugreport.zip`
    ///
    /// ```no_run
    /// # use adbr::{Adb, AdbCommand};
    /// # let adb = Adb::new();
    /// adb.bug_report()
    ///     .path("/path/to/bugreport.zip") // optional
    ///     .status()
    ///     .expect("`adb bugreport /path/to/bugreport.zip` failed");
    /// ```
    pub fn bug_report(&self) -> AdbBugReport<OsString> {
        AdbBugReport::new(self.command())
    }
}

impl<'a> AdbCommandBuilder<'a> {
    /// `bugreport [PATH]`: Write bugreport to given PATH (default=`bugreport.zip`).
    ///
    /// See [`Adb::bug_report`] for more information.
    pub fn bug_report<S: AsRef<OsStr>>(self) -> AdbBugReport<'a, S> {
        AdbBugReport::new(self)
    }
}

/// `jdwp`: List pids of processes hosting a JDWP transport.
#[derive(Debug, Clone)]
pub struct AdbJdwp<'a>(AdbCommandBuilder<'a>);

impl<'a> AdbCommand for AdbJdwp<'a> {
    fn build(self) -> Command {
        let mut cmd = self.0.build();
        cmd.arg("jdwp");
        cmd
    }
}

impl Adb {
    /// `jdwp`: List pids of processes hosting a JDWP transport.
    ///
    /// # Examples
    ///
    /// `adb jdwp`
    ///
    /// ```no_run
    /// # use adbr::{Adb, AdbCommand};
    /// # let adb = Adb::new();
    /// adb.jdwp().status().expect("`adb jdwp` failed");
    /// ```
    pub fn jdwp(&self) -> AdbJdwp {
        AdbJdwp(self.command())
    }
}

impl<'a> AdbCommandBuilder<'a> {
    /// `jdwp`: List pids of processes hosting a JDWP transport.
    ///
    /// See [`Adb::jdwp`] for more information.
    pub fn jdwp(self) -> AdbJdwp<'a> {
        AdbJdwp(self)
    }
}

/// `logcat`: Show device log.
#[derive(Debug, Clone)]
pub struct AdbLogcat<'a>(AdbCommandBuilder<'a>);

impl<'a> AdbCommand for AdbLogcat<'a> {
    fn build(self) -> Command {
        let mut cmd = self.0.build();
        cmd.arg("logcat");
        cmd
    }
}

impl Adb {
    /// `logcat`: Show device log.
    ///
    /// # Examples
    ///
    /// `adb logcat`
    ///
    /// ```no_run
    /// # use adbr::{Adb, AdbCommand};
    /// # let adb = Adb::new();
    /// adb.logcat().status().expect("`adb logcat` failed");
    /// ```
    pub fn logcat(&self) -> AdbLogcat {
        AdbLogcat(self.command())
    }
}

impl<'a> AdbCommandBuilder<'a> {
    /// `logcat`: Show device log.
    ///
    /// See [`Adb::logcat`] for more information.
    pub fn logcat(self) -> AdbLogcat<'a> {
        AdbLogcat(self)
    }
}
