//! Internal debugging commands.
//!
//! - `start-server`: Ensure that there is a server running.
//! - `kill-server`: Kill the server if it is running.
//! - `reconnect`: Close connection from host side to force reconnect.
//! - `reconnect device`: Close connection from device side to force reconnect.
//! - `reconnect offline`: Reset offline/unauthorized devices to force reconnect.
//!
//! See [Internal Debugging Commands](https://android.googlesource.com/platform/packages/modules/adb/+/refs/heads/master/docs/user/adb.1.md#internal-debugging).

use std::process::Command;

use crate::command::AdbCommandBuilder;
use crate::{Adb, AdbCommand};

/// `start-server`: Ensure that there is a server running.
#[derive(Debug, Clone)]
pub struct AdbStartServer<'a>(AdbCommandBuilder<'a>);

impl<'a> AdbCommand for AdbStartServer<'a> {
    fn build(self) -> Command {
        let mut cmd = self.0.build();
        cmd.arg("start-server");
        cmd
    }
}

impl Adb {
    /// `start-server`: Ensure that there is a server running.
    ///
    /// # Examples
    ///
    /// `adb start-server`
    ///
    /// ```no_run
    /// # use adbr::{Adb, AdbCommand};
    /// # let adb = Adb::new().unwrap();
    /// adb.start_server()
    ///     .status()
    ///     .expect("`adb start-server` failed");
    /// ```
    pub fn start_server(&self) -> AdbStartServer {
        AdbStartServer(self.command())
    }
}

impl<'a> AdbCommandBuilder<'a> {
    /// `start-server`: Ensure that there is a server running.
    ///
    /// See [`Adb::start_server`] for more information.
    pub fn start_server(self) -> AdbStartServer<'a> {
        AdbStartServer(self)
    }
}

/// `kill-server`: Kill the server if it is running.
#[derive(Debug, Clone)]
pub struct AdbKillServer<'a>(AdbCommandBuilder<'a>);

impl<'a> AdbCommand for AdbKillServer<'a> {
    fn build(self) -> Command {
        let mut cmd = self.0.build();
        cmd.arg("kill-server");
        cmd
    }
}

impl Adb {
    /// `kill-server`: Kill the server if it is running.
    ///
    /// # Examples
    ///
    /// `adb kill-server`
    ///
    /// ```no_run
    /// # use adbr::{Adb, AdbCommand};
    /// # let adb = Adb::new().unwrap();
    /// adb.kill_server()
    ///     .status()
    ///     .expect("`adb kill-server` failed");
    /// ```
    pub fn kill_server(&self) -> AdbKillServer {
        AdbKillServer(self.command())
    }
}

impl<'a> AdbCommandBuilder<'a> {
    /// `kill-server`: Kill the server if it is running.
    ///
    /// See [`Adb::kill_server`] for more information.
    pub fn kill_server(self) -> AdbKillServer<'a> {
        AdbKillServer(self)
    }
}

/// `reconnect`: Close connection from host side to force reconnect.
#[derive(Debug, Clone)]
pub struct AdbReconnect<'a>(AdbCommandBuilder<'a>);

impl<'a> AdbReconnect<'a> {
    /// `device`: Close connection from device side to force reconnect.
    ///
    /// # Examples
    ///
    /// `adb reconnect device`
    ///
    /// ```no_run
    /// # use adbr::{Adb, AdbCommand};
    /// # let adb = Adb::new().unwrap();
    /// adb.reconnect()
    ///     .device()
    ///     .status()
    ///     .expect("`adb reconnect device` failed");
    /// ```
    pub fn device(self) -> AdbReconnectDevice<'a> {
        AdbReconnectDevice(self.0)
    }

    /// `offline`: Reset offline/unauthorized devices to force reconnect.
    ///
    /// # Examples
    ///
    /// `adb reconnect offline`
    ///
    /// ```no_run
    /// # use adbr::{Adb, AdbCommand};
    /// # let adb = Adb::new().unwrap();
    /// adb.reconnect()
    ///     .offline()
    ///     .status()
    ///     .expect("`adb reconnect offline` failed");
    /// ```
    pub fn offline(self) -> AdbReconnectOffline<'a> {
        AdbReconnectOffline(self.0)
    }
}

impl<'a> AdbCommand for AdbReconnect<'a> {
    fn build(self) -> Command {
        let mut cmd = self.0.build();
        cmd.arg("reconnect");
        cmd
    }
}

impl Adb {
    /// `reconnect`: Close connection from host side to force reconnect.
    ///
    /// # Examples
    ///
    /// `adb reconnect`
    ///
    /// ```no_run
    /// # use adbr::{Adb, AdbCommand};
    /// # let adb = Adb::new().unwrap();
    /// adb.reconnect()
    ///     .status()
    ///     .expect("`adb reconnect` failed");
    /// ```
    pub fn reconnect(&self) -> AdbReconnect {
        AdbReconnect(self.command())
    }
}

impl<'a> AdbCommandBuilder<'a> {
    /// `reconnect`: Close connection from host side to force reconnect.
    ///
    /// See [`Adb::reconnect`] for more information.
    pub fn reconnect(self) -> AdbReconnect<'a> {
        AdbReconnect(self)
    }
}

/// `reconnect device`: Close connection from device side to force reconnect.
#[derive(Debug, Clone)]
pub struct AdbReconnectDevice<'a>(AdbCommandBuilder<'a>);

impl<'a> AdbCommand for AdbReconnectDevice<'a> {
    fn build(self) -> Command {
        let mut cmd = self.0.build();
        cmd.arg("reconnect").arg("device");
        cmd
    }
}

/// `reconnect offline`: Reset offline/unauthorized devices to force reconnect.
#[derive(Debug, Clone)]
pub struct AdbReconnectOffline<'a>(AdbCommandBuilder<'a>);

impl<'a> AdbCommand for AdbReconnectOffline<'a> {
    fn build(self) -> Command {
        let mut cmd = self.0.build();
        cmd.arg("reconnect").arg("offline");
        cmd
    }
}
