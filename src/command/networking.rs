//! Networking commands
//!
//! - `connect HOST[:PORT]`: Connect to a device via TCP/IP (default `PORT=5555`).
//! - `disconnect [HOST[:PORT]]`: Disconnect from given TCP/IP device (default `PORT=5555`), or all.
//! - `pair HOST[:PORT] [PAIRING_CODE]`: Pair with a device for secure TCP/IP communication.
//! - `forward --list | [--no-rebind] LOCAL_REMOTE | --remove LOCAL | --remove-all`: Forward socket connections.
//! - `reverse --list | [--no-rebind] LOCAL_REMOTE | --remove LOCAL | --remove-all`: Reverse socket connections.
//! - `mdns check | services`: Perform mDNS subcommands.
//!
//! See [Networking Commands](https://android.googlesource.com/platform/packages/modules/adb/+/refs/heads/master/docs/user/adb.1.md#networking).

use std::ffi::OsStr;
use std::process::Command;

use crate::command::AdbCommandBuilder;
use crate::{Adb, AdbCommand};

/// `connect HOST[:PORT]`: Connect to a device via TCP/IP (default `PORT=5555`).
#[derive(Debug, Clone)]
pub struct AdbConnect<'a, S: AsRef<OsStr>> {
    acb: AdbCommandBuilder<'a>,
    /// `HOST`: The host to connect to.
    host: S,
    /// `PORT`: The optional port to connect to, default is `5555`.
    port: Option<u16>,
}

impl<'a, S: AsRef<OsStr>> AdbConnect<'a, S> {
    /// Creates a new `Connect` command without the port.
    fn new(acb: AdbCommandBuilder<'a>, host: S) -> Self {
        Self {
            acb,
            host,
            port: None,
        }
    }

    /// `HOST`: The host to connect to.
    ///
    /// The previous host will be overwritten.
    pub fn host<S1: AsRef<OsStr>>(self, host: S1) -> AdbConnect<'a, S1> {
        AdbConnect {
            acb: self.acb,
            host,
            port: self.port,
        }
    }

    /// `PORT`: The optional port to connect to, default is `5555`.
    ///
    /// The previous port will be overwritten.
    pub fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }
}

impl<'a, S: AsRef<OsStr>> AdbCommand for AdbConnect<'a, S> {
    fn build(self) -> Command {
        let mut cmd = self.acb.build();
        cmd.arg("connect");
        if let Some(port) = self.port {
            let mut arg = self.host.as_ref().to_os_string();
            arg.push(":");
            arg.push(port.to_string());
            cmd.arg(arg);
        } else {
            cmd.arg(self.host);
        };
        cmd
    }
}

impl Adb {
    /// `connect HOST[:PORT]`: Connect to a device via TCP/IP (default `PORT=5555`).
    ///
    /// # Note
    ///
    /// The host can be an IP address or a domain name.
    /// However, the validity of the host is not checked.
    ///
    /// # Example
    ///
    /// `adb connect localhost:5555`
    ///
    /// ```no_run
    /// # use adbr::{Adb, AdbCommand};
    /// # let adb = Adb::new();
    /// adb.connect("localhost")
    ///     .port(5555) // optional
    ///     .status()
    ///     .expect("adb connect failed");
    /// ```
    pub fn connect<S: AsRef<OsStr>>(&self, host: S) -> AdbConnect<S> {
        AdbConnect::new(self.command(), host)
    }
}

impl<'a> AdbCommandBuilder<'a> {
    /// `connect HOST[:PORT]`: Connect to a device via TCP/IP (default `PORT=5555`).
    ///
    /// See [`Adb::connect`] for more information.
    pub fn connect<S: AsRef<OsStr>>(self, host: S) -> AdbConnect<'a, S> {
        AdbConnect::new(self, host)
    }
}

/// `disconnect [HOST[:PORT]]`: Disconnect from given TCP/IP device (default `PORT=5555`), or all.
#[derive(Debug, Clone)]
pub struct AdbDisconnect<'a, S: AsRef<OsStr>> {
    acb: AdbCommandBuilder<'a>,
    /// `HOST`: The optional host to disconnect from.
    host: Option<S>,
    /// `PORT`: The optional port to disconnect from, default is `5555`.
    port: Option<u16>,
}

impl<'a, S: AsRef<OsStr>> AdbDisconnect<'a, S> {
    /// Creates a new `Disconnect` command without the host and port.
    fn new(acb: AdbCommandBuilder<'a>) -> Self {
        Self {
            acb,
            host: None,
            port: None,
        }
    }

    /// `HOST`: The optional host to disconnect from.
    ///
    /// The previous host will be overwritten.
    ///
    /// # Note
    ///
    /// The host can be an IP address or a domain name.
    /// However, the validity of the host is not checked.
    pub fn host<S1: AsRef<OsStr>>(self, host: S1) -> AdbDisconnect<'a, S1> {
        AdbDisconnect {
            acb: self.acb,
            host: Some(host),
            port: self.port,
        }
    }

    /// `PORT`: The optional port to disconnect from, default is `5555`.
    ///
    /// The previous port will be overwritten.
    pub fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }
}

impl<'a, S: AsRef<OsStr>> AdbCommand for AdbDisconnect<'a, S> {
    fn build(self) -> Command {
        let mut cmd = self.acb.build();
        cmd.arg("disconnect");
        if let Some(host) = self.host {
            let mut arg = host.as_ref().to_os_string();
            if let Some(port) = self.port {
                arg.push(":");
                arg.push(port.to_string());
            }
            cmd.arg(arg);
        }
        cmd
    }
}

impl Adb {
    /// `disconnect [HOST[:PORT]]`: Disconnect from given TCP/IP device (default `PORT=5555`), or all.
    ///
    /// # Example
    ///
    /// `adb disconnect localhost:5555`
    ///
    /// ```no_run
    /// # use adbr::{Adb, AdbCommand};
    /// # let adb = Adb::new();
    /// adb.disconnect()
    ///     .host("localhost")  // optional
    ///     .port(5555)         // optional
    ///     .status()
    ///     .expect("adb disconnect failed");
    /// ```
    pub fn disconnect(&self) -> AdbDisconnect<&str> {
        AdbDisconnect::new(self.command())
    }
}

impl<'a> AdbCommandBuilder<'a> {
    /// `disconnect [HOST[:PORT]]`: Disconnect from given TCP/IP device (default `PORT=5555`), or all.
    ///
    /// See [`Adb::disconnect`] for more information.
    pub fn disconnect(self) -> AdbDisconnect<'a, &'a str> {
        AdbDisconnect::new(self)
    }
}

/// `pair HOST[:PORT] [PAIRING_CODE]`: Pair with a device for secure TCP/IP communication.
#[derive(Debug, Clone)]
pub struct AdbPair<'a, S1: AsRef<OsStr>, S2: AsRef<OsStr>> {
    acb: AdbCommandBuilder<'a>,
    /// `HOST`: The host to pair with.
    host: S1,
    /// `PORT`: The optional port to pair with, default is `5555`.
    port: Option<u16>,
    /// `PAIRING_CODE`: The optional pairing code.
    pairing_code: Option<S2>,
}

impl<'a, S1, S2> AdbPair<'a, S1, S2>
where
    S1: AsRef<OsStr>,
    S2: AsRef<OsStr>,
{
    /// Creates a new `Pair` command with the host.
    fn new(acb: AdbCommandBuilder<'a>, host: S1) -> Self {
        Self {
            acb,
            host,
            port: None,
            pairing_code: None,
        }
    }

    /// `HOST`: The host to pair with.
    ///
    /// The previous host will be overwritten.
    pub fn host<S: AsRef<OsStr>>(self, host: S) -> AdbPair<'a, S, S2> {
        AdbPair {
            acb: self.acb,
            host,
            port: self.port,
            pairing_code: self.pairing_code,
        }
    }

    /// `PORT`: The optional port to pair with, default is `5555`.
    ///
    /// The previous port will be overwritten.
    pub fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    /// `PAIRING_CODE`: The optional pairing code.
    ///
    /// The previous pairing code will be overwritten.
    pub fn pairing_code<S: AsRef<OsStr>>(self, pairing_code: S) -> AdbPair<'a, S1, S> {
        AdbPair {
            acb: self.acb,
            host: self.host,
            port: self.port,
            pairing_code: Some(pairing_code),
        }
    }
}

impl<'a, S1, S2> AdbCommand for AdbPair<'a, S1, S2>
where
    S1: AsRef<OsStr>,
    S2: AsRef<OsStr>,
{
    fn build(self) -> Command {
        let mut cmd = self.acb.build();
        cmd.arg("pair");
        let mut host_and_port = self.host.as_ref().to_os_string();
        if let Some(port) = self.port {
            host_and_port.push(":");
            host_and_port.push(port.to_string());
        }
        cmd.arg(host_and_port);
        if let Some(pairing_code) = self.pairing_code {
            cmd.arg(pairing_code);
        }
        cmd
    }
}

impl Adb {
    /// `pair HOST[:PORT] [PAIRING_CODE]`: Pair with a device for secure TCP/IP communication.
    ///
    /// # Note
    ///
    /// The host can be an IP address or a domain name.
    /// However, the validity of the host is not checked.
    ///
    /// # Example
    ///
    /// `adb pair localhost:5555 123456`
    ///
    /// ```no_run
    /// # use adbr::{Adb, AdbCommand};
    /// # let adb = Adb::new();
    /// adb.pair("localhost")
    ///     .port(5555)             // optional
    ///     .pairing_code("123456") // optional
    ///     .status()
    ///     .expect("adb pair failed");
    /// ```
    pub fn pair<S: AsRef<OsStr>>(&self, host: S) -> AdbPair<S, S> {
        AdbPair::new(self.command(), host)
    }
}

impl<'a> AdbCommandBuilder<'a> {
    /// `pair HOST[:PORT] [PAIRING_CODE]`: Pair with a device for secure TCP/IP communication.
    ///
    /// See [`Adb::pair`] for more information.
    pub fn pair<S: AsRef<OsStr>>(self, host: S) -> AdbPair<'a, S, S> {
        AdbPair::new(self, host)
    }
}

/// `forward --list | [--no-rebind] LOCAL REMOTE | --remove LOCAL | --remove-all`
/// - `--list`: List all forward socket connections.
/// - `[--no-rebind] LOCAL REMOTE`: Forward socket connection using one of the followings.
///   - `tcp:PORT` (local may be `tcp:0` to pick any open port).
///   - `localreserved:UNIX_DOMAIN_SOCKET_NAME`.
///   - `localfilesystem:UNIX_DOMAIN_SOCKET_NAME`.
///   - `jdwp:PROCESS PID` (remote only).
///   - `vsock:CID:PORT` (remote only).
///   - `acceptfd:FD` (listen only).
///   - `dev:DEVICE_NAME`.
///   - `dev-raw:DEVICE_NAME`. (open device in raw mode).
/// - `--remove LOCAL`: Remove specific forward socket connection.
/// - `--remove-all`: Remove all forward socket connections.
#[derive(Debug, Clone)]
pub struct AdbForward<'a>(AdbCommandBuilder<'a>);

impl<'a> AdbForward<'a> {
    /// `LOCAL REMOTE`: local and remote socket address
    pub fn arg<S1, S2>(self, local: S1, remote: S2) -> AdbForwardNoRebind<'a, S1, S2>
    where
        S1: AsRef<OsStr>,
        S2: AsRef<OsStr>,
    {
        AdbForwardNoRebind::new(self.0, local, remote)
    }

    /// `--list`: List all forward socket connections.
    pub fn list(self) -> AdbForwardList<'a> {
        AdbForwardList(self.0)
    }

    /// `--no-rebind LOCAL REMOTE`: Forward socket connection without rebinding.
    pub fn no_rebind<S1, S2>(self, local: S1, remote: S2) -> AdbForwardNoRebind<'a, S1, S2>
    where
        S1: AsRef<OsStr>,
        S2: AsRef<OsStr>,
    {
        AdbForwardNoRebind::new(self.0, local, remote).no_rebind()
    }

    /// `--remove LOCAL`: Remove specific forward socket connection.
    pub fn remove<S: AsRef<OsStr>>(self, local: S) -> AdbForwardRemove<'a, S> {
        AdbForwardRemove::new(self.0, local)
    }

    /// `--remove-all`: Remove all forward socket connections.
    pub fn remove_all(self) -> AdbForwardRemoveAll<'a> {
        AdbForwardRemoveAll(self.0)
    }
}

impl Adb {
    /// `forward --list | [--no-rebind] LOCAL REMOTE | --remove LOCAL | --remove-all`
    /// - `--list`: List all forward socket connections.
    /// - `[--no-rebind] LOCAL REMOTE`: Forward socket connection using one of the followings.
    ///   - `tcp:PORT` (local may be `tcp:0` to pick any open port).
    ///   - `localreserved:UNIX_DOMAIN_SOCKET_NAME`.
    ///   - `localfilesystem:UNIX_DOMAIN_SOCKET_NAME`.
    ///   - `jdwp:PROCESS PID` (remote only).
    ///   - `vsock:CID:PORT` (remote only).
    ///   - `acceptfd:FD` (listen only).
    ///   - `dev:DEVICE_NAME`.
    ///   - `dev-raw:DEVICE_NAME`. (open device in raw mode).
    /// - `--remove LOCAL`: Remove specific forward socket connection.
    /// - `--remove-all`: Remove all forward socket connections.
    ///
    /// # Example
    ///
    /// - `adb forward tcp:1234 tcp:5678`
    ///
    /// ```no_run
    /// # use adbr::{Adb, AdbCommand};
    /// # let adb = Adb::new();
    /// adb.forward()
    ///     .arg("tcp:1234", "tcp:5678")
    ///     .status()
    ///     .expect("`adb forward tcp:1234 tcp:5678` failed");
    /// ```
    ///
    /// - `adb forward --list`
    ///
    /// ```no_run
    /// # use adbr::{Adb, AdbCommand};
    /// # let adb = Adb::new();
    /// adb.forward()
    ///     .list()
    ///     .status()
    ///     .expect("`adb forward --list` failed");
    /// ```
    ///
    /// - `adb forward --no-rebind tcp:1234 tcp:5678`
    ///
    /// ```no_run
    /// # use adbr::{Adb, AdbCommand};
    /// # let adb = Adb::new();
    /// adb.forward()
    ///     .no_rebind("tcp:1234", "tcp:5678")
    ///     .status()
    ///     .expect("`adb forward --no-rebind tcp:1234 tcp:5678` failed");
    /// // or
    /// adb.forward()
    ///     .arg("tcp:1234", "tcp:5678")
    ///     .no_rebind()
    ///     .status()
    ///     .expect("`adb forward --no-rebind tcp:1234 tcp:5678` failed");
    /// ```
    ///
    /// - `adb forward --remove tcp:5555`
    ///
    /// ```no_run
    /// # use adbr::{Adb, AdbCommand};
    /// # let adb = Adb::new();
    /// adb.forward()
    ///     .remove("tcp:5555")
    ///     .status()
    ///     .expect("`adb forward --remove tcp:5555` failed");
    /// ```
    ///
    /// - `adb forward --remove-all`
    ///
    /// ```no_run
    /// # use adbr::{Adb, AdbCommand};
    /// # let adb = Adb::new();
    /// adb.forward()
    ///     .remove_all()
    ///     .status()
    ///     .expect("`adb forward --remove-all` failed");
    /// ```
    pub fn forward(&self) -> AdbForward {
        AdbForward(self.command())
    }
}

impl<'a> AdbCommandBuilder<'a> {
    /// `forward --list | [--no-rebind] LOCAL REMOTE | --remove LOCAL | --remove-all`
    /// - `--list`: List all forward socket connections.
    /// - `[--no-rebind] LOCAL REMOTE`: Forward socket connection using one of the followings.
    ///   - `tcp:PORT` (local may be `tcp:0` to pick any open port).
    ///   - `localreserved:UNIX_DOMAIN_SOCKET_NAME`.
    ///   - `localfilesystem:UNIX_DOMAIN_SOCKET_NAME`.
    ///   - `jdwp:PROCESS PID` (remote only).
    ///   - `vsock:CID:PORT` (remote only).
    ///   - `acceptfd:FD` (listen only).
    ///   - `dev:DEVICE_NAME`.
    ///   - `dev-raw:DEVICE_NAME`. (open device in raw mode).
    /// - `--remove LOCAL`: Remove specific forward socket connection.
    /// - `--remove-all`: Remove all forward socket connections.
    ///
    /// See [`Adb::forward`] for more information.
    pub fn forward(self) -> AdbForward<'a> {
        AdbForward(self)
    }
}

/// A subcommand of `forward`.
///
/// `forward --list`: List all forward socket connections.
#[derive(Debug, Clone)]
pub struct AdbForwardList<'a>(AdbCommandBuilder<'a>);

impl<'a> AdbCommand for AdbForwardList<'a> {
    fn build(self) -> Command {
        let mut cmd = self.0.build();
        cmd.arg("forward").arg("--list");
        cmd
    }
}

/// A subcommand of `forward`.
///
/// `forward --no-rebind LOCAL REMOTE`: Forward socket connection without rebinding.
#[derive(Debug, Clone)]
pub struct AdbForwardNoRebind<'a, S1: AsRef<OsStr>, S2: AsRef<OsStr>> {
    acb: AdbCommandBuilder<'a>,
    /// `--no-rebind`: Whether to rebind the connection.
    no_rebind: bool,
    /// `LOCAL`: The local socket to forward.
    local: S1,
    /// `REMOTE`: The remote socket to forward.
    remote: S2,
}

impl<'a, S1, S2> AdbForwardNoRebind<'a, S1, S2>
where
    S1: AsRef<OsStr>,
    S2: AsRef<OsStr>,
{
    fn new(acb: AdbCommandBuilder<'a>, local: S1, remote: S2) -> Self {
        Self {
            acb,
            no_rebind: false,
            local,
            remote,
        }
    }

    /// `LOCAL`: The local socket to forward.
    ///
    /// The previous local socket will be overwritten.
    pub fn local<S: AsRef<OsStr>>(self, local: S) -> AdbForwardNoRebind<'a, S, S2> {
        AdbForwardNoRebind {
            acb: self.acb,
            no_rebind: self.no_rebind,
            local,
            remote: self.remote,
        }
    }

    /// `REMOTE`: The remote socket to forward.
    ///
    /// The previous remote socket will be overwritten.
    pub fn remote<S: AsRef<OsStr>>(self, remote: S) -> AdbForwardNoRebind<'a, S1, S> {
        AdbForwardNoRebind {
            acb: self.acb,
            no_rebind: self.no_rebind,
            local: self.local,
            remote,
        }
    }

    /// `--no-rebind`: Whether to rebind the connection.
    pub fn no_rebind(mut self) -> Self {
        self.no_rebind = true;
        self
    }
}

impl<'a, S1, S2> AdbCommand for AdbForwardNoRebind<'a, S1, S2>
where
    S1: AsRef<OsStr>,
    S2: AsRef<OsStr>,
{
    fn build(self) -> Command {
        let mut cmd = self.acb.build();
        cmd.arg("forward");
        if self.no_rebind {
            cmd.arg("--no-rebind");
        }
        cmd.arg(self.local).arg(self.remote);
        cmd
    }
}

/// A subcommand of `forward`.
///
/// `forward --remove LOCAL`: Remove specific forward socket connection.
#[derive(Debug, Clone)]
pub struct AdbForwardRemove<'a, S: AsRef<OsStr>> {
    acb: AdbCommandBuilder<'a>,
    /// `LOCAL`: The local socket to remove.
    local: S,
}

impl<'a, S: AsRef<OsStr>> AdbForwardRemove<'a, S> {
    fn new(acb: AdbCommandBuilder<'a>, local: S) -> Self {
        Self { acb, local }
    }

    /// `LOCAL`: The local socket to remove.
    ///
    /// The previous local socket will be overwritten.
    pub fn local<S1: AsRef<OsStr>>(self, local: S1) -> AdbForwardRemove<'a, S1> {
        AdbForwardRemove::new(self.acb, local)
    }
}

impl<'a, S: AsRef<OsStr>> AdbCommand for AdbForwardRemove<'a, S> {
    fn build(self) -> Command {
        let mut cmd = self.acb.build();
        cmd.arg("forward").arg("--remove").arg(self.local);
        cmd
    }
}

/// A subcommand of `forward`.
///
/// `forward --remove-all`: Remove all forward socket connections.
#[derive(Debug, Clone)]
pub struct AdbForwardRemoveAll<'a>(AdbCommandBuilder<'a>);

impl<'a> AdbCommand for AdbForwardRemoveAll<'a> {
    fn build(self) -> Command {
        let mut cmd = self.0.build();
        cmd.arg("forward").arg("--remove-all");
        cmd
    }
}

/// `reverse --list | [--no-rebind] LOCAL REMOTE | --remove LOCAL | --remove-all`
/// - `--list`: List all reverse socket connections from device.
/// - `[--no-rebind] REMOTE LOCAL`: Reverse socket connection using one of the following.
///   - `tcp:PORT` (REMOTE may be `tcp:0` to pick any open port).
///   - `localabstract:UNIX_DOMAIN_SOCKET_NAME`.
///   - `localreserved:UNIX_DOMAIN_SOCKET_NAME`.
///   - `localfilesystem:UNIX_DOMAIN_SOCKET_NAME`.
/// - `--remove REMOTE`: Remove specific reverse socket connection.
/// - `--remove-all`: Remove all reverse socket connections from device.
#[derive(Debug, Clone)]
pub struct AdbReverse<'a>(AdbCommandBuilder<'a>);

impl<'a> AdbReverse<'a> {
    /// `REMOTE LOCAL`: remote and local socket address
    pub fn arg<S1, S2>(self, remote: S1, local: S2) -> AdbReverseNoRebind<'a, S1, S2>
    where
        S1: AsRef<OsStr>,
        S2: AsRef<OsStr>,
    {
        AdbReverseNoRebind::new(self.0, remote, local)
    }

    /// `--list`: List all reverse socket connections from device.
    pub fn list(self) -> AdbReverseList<'a> {
        AdbReverseList(self.0)
    }

    /// `--no-rebind REMOTE LOCAL`: Reverse socket connection without rebinding.
    pub fn no_rebind<S1, S2>(self, remote: S1, local: S2) -> AdbReverseNoRebind<'a, S1, S2>
    where
        S1: AsRef<OsStr>,
        S2: AsRef<OsStr>,
    {
        AdbReverseNoRebind::new(self.0, remote, local).no_rebind()
    }

    /// `--remove REMOTE`: Remove specific reverse socket connection.
    pub fn remove<S: AsRef<OsStr>>(self, remote: S) -> AdbReverseRemove<'a, S> {
        AdbReverseRemove::new(self.0, remote)
    }

    /// `--remove-all`: Remove all reverse socket connections from device.
    pub fn remove_all(self) -> AdbReverseRemoveAll<'a> {
        AdbReverseRemoveAll(self.0)
    }
}

impl Adb {
    /// `reverse --list | [--no-rebind] LOCAL REMOTE | --remove LOCAL | --remove-all`
    /// - `--list`: List all reverse socket connections from device.
    /// - `[--no-rebind] REMOTE LOCAL`: Reverse socket connection using one of the following.
    ///   - `tcp:PORT` (REMOTE may be `tcp:0` to pick any open port).
    ///   - `localabstract:UNIX_DOMAIN_SOCKET_NAME`.
    ///   - `localreserved:UNIX_DOMAIN_SOCKET_NAME`.
    ///   - `localfilesystem:UNIX_DOMAIN_SOCKET_NAME`.
    /// - `--remove REMOTE`: Remove specific reverse socket connection.
    /// - `--remove-all`: Remove all reverse socket connections from device.
    ///
    /// # Example
    ///
    /// - `adb reverse tcp:1234 tcp:5678`
    ///
    /// ```no_run
    /// # use adbr::{Adb, AdbCommand};
    /// # let adb = Adb::new();
    /// adb.reverse()
    ///     .arg("tcp:1234", "tcp:5678")
    ///     .status()
    ///     .expect("`adb reverse tcp:1234 tcp:5678` failed");
    /// ```
    ///
    /// - `adb reverse --list`
    ///
    /// ```no_run
    /// # use adbr::{Adb, AdbCommand};
    /// # let adb = Adb::new();
    /// adb.reverse()
    ///     .list()
    ///     .status()
    ///     .expect("`adb reverse --list` failed");
    /// ```
    ///
    /// - `adb reverse --no-rebind tcp:1234 tcp:5678`
    ///
    /// ```no_run
    /// # use adbr::{Adb, AdbCommand};
    /// # let adb = Adb::new();
    /// adb.reverse()
    ///     .no_rebind("tcp:1234", "tcp:5678")
    ///     .status()
    ///     .expect("`adb reverse --no-rebind tcp:1234 tcp:5678` failed");
    /// // or
    /// adb.reverse()
    ///     .arg("tcp:1234", "tcp:5678")
    ///     .no_rebind()
    ///     .status()
    ///     .expect("`adb reverse --no-rebind tcp:1234 tcp:5678` failed");
    /// ```
    ///
    /// - `adb reverse --remove tcp:5555`
    ///
    /// ```no_run
    /// # use adbr::{Adb, AdbCommand};
    /// # let adb = Adb::new();
    /// adb.reverse()
    ///     .remove("tcp:5555")
    ///     .status()
    ///     .expect("`adb reverse --remove tcp:5555` failed");
    /// ```
    ///
    /// - `adb reverse --remove-all`
    ///
    /// ```no_run
    /// # use adbr::{Adb, AdbCommand};
    /// # let adb = Adb::new();
    /// adb.reverse()
    ///     .remove_all()
    ///     .status()
    ///     .expect("`adb reverse --remove-all` failed");
    /// ```
    pub fn reverse(&self) -> AdbReverse {
        AdbReverse(self.command())
    }
}

impl<'a> AdbCommandBuilder<'a> {
    /// `reverse --list | [--no-rebind] LOCAL REMOTE | --remove LOCAL | --remove-all`
    /// - `--list`: List all reverse socket connections from device.
    /// - `[--no-rebind] REMOTE LOCAL`: Reverse socket connection using one of the following.
    ///   - `tcp:PORT` (REMOTE may be `tcp:0` to pick any open port).
    ///   - `localabstract:UNIX_DOMAIN_SOCKET_NAME`.
    ///   - `localreserved:UNIX_DOMAIN_SOCKET_NAME`.
    ///   - `localfilesystem:UNIX_DOMAIN_SOCKET_NAME`.
    /// - `--remove REMOTE`: Remove specific reverse socket connection.
    /// - `--remove-all`: Remove all reverse socket connections from device.
    ///
    /// See [`Adb::reverse`] for more information.
    pub fn reverse(self) -> AdbReverse<'a> {
        AdbReverse(self)
    }
}

/// A subcommand of `reverse`.
///
/// `reverse --list`: List all reverse socket connections from device.
#[derive(Debug, Clone)]
pub struct AdbReverseList<'a>(AdbCommandBuilder<'a>);

impl<'a> AdbCommand for AdbReverseList<'a> {
    fn build(self) -> Command {
        let mut cmd = self.0.build();
        cmd.arg("reverse").arg("--list");
        cmd
    }
}

/// A subcommand of `reverse`.
///
/// `reverse --no-rebind REMOTE LOCAL`: Reverse socket connection without rebinding.
#[derive(Debug, Clone)]
pub struct AdbReverseNoRebind<'a, S1: AsRef<OsStr>, S2: AsRef<OsStr>> {
    acb: AdbCommandBuilder<'a>,
    /// `--no-rebind`: Whether to rebind the connection.
    no_rebind: bool,
    /// `REMOTE`: The remote socket to reverse.
    remote: S1,
    /// `LOCAL`: The local socket to reverse.
    local: S2,
}

impl<'a, S1, S2> AdbReverseNoRebind<'a, S1, S2>
where
    S1: AsRef<OsStr>,
    S2: AsRef<OsStr>,
{
    fn new(acb: AdbCommandBuilder<'a>, remote: S1, local: S2) -> Self {
        Self {
            acb,
            no_rebind: false,
            remote,
            local,
        }
    }

    /// `--no-rebind`: Whether to rebind the connection.
    pub fn no_rebind(mut self) -> Self {
        self.no_rebind = true;
        self
    }

    /// `REMOTE`: The remote socket to reverse.
    ///
    /// The previous remote socket will be overwritten.
    pub fn remote<S: AsRef<OsStr>>(self, remote: S) -> AdbReverseNoRebind<'a, S, S2> {
        AdbReverseNoRebind {
            acb: self.acb,
            no_rebind: self.no_rebind,
            remote,
            local: self.local,
        }
    }

    /// `LOCAL`: The local socket to reverse.
    ///
    /// The previous local socket will be overwritten.
    pub fn local<S: AsRef<OsStr>>(self, local: S) -> AdbReverseNoRebind<'a, S1, S> {
        AdbReverseNoRebind {
            acb: self.acb,
            no_rebind: self.no_rebind,
            remote: self.remote,
            local,
        }
    }
}

impl<'a, S1, S2> AdbCommand for AdbReverseNoRebind<'a, S1, S2>
where
    S1: AsRef<OsStr>,
    S2: AsRef<OsStr>,
{
    fn build(self) -> Command {
        let mut cmd = self.acb.build();
        cmd.arg("reverse");
        if self.no_rebind {
            cmd.arg("--no-rebind");
        }
        cmd.arg(self.remote).arg(self.local);
        cmd
    }
}

/// A subcommand of `reverse`.
///
/// `reverse --remove REMOTE`: Remove specific reverse socket connection.
#[derive(Debug, Clone)]
pub struct AdbReverseRemove<'a, S: AsRef<OsStr>> {
    acb: AdbCommandBuilder<'a>,
    /// `REMOTE`: The remote socket to remove.
    remote: S,
}

impl<'a, S: AsRef<OsStr>> AdbReverseRemove<'a, S> {
    fn new(acb: AdbCommandBuilder<'a>, remote: S) -> Self {
        Self { acb, remote }
    }

    /// `REMOTE`: The remote socket to remove.
    ///
    /// The previous remote socket will be overwritten.
    pub fn remote<S1: AsRef<OsStr>>(self, remote: S1) -> AdbReverseRemove<'a, S1> {
        AdbReverseRemove::new(self.acb, remote)
    }
}

impl<'a, S: AsRef<OsStr>> AdbCommand for AdbReverseRemove<'a, S> {
    fn build(self) -> Command {
        let mut cmd = self.acb.build();
        cmd.arg("reverse").arg("--remove").arg(self.remote);
        cmd
    }
}

/// A subcommand of `reverse`.
///
/// `reverse --remove-all`: Remove all reverse socket connections from device.
#[derive(Debug, Clone)]
pub struct AdbReverseRemoveAll<'a>(AdbCommandBuilder<'a>);

impl<'a> AdbCommand for AdbReverseRemoveAll<'a> {
    fn build(self) -> Command {
        let mut cmd = self.0.build();
        cmd.arg("reverse").arg("--remove-all");
        cmd
    }
}

/// `mdns check | services`: Perform mDNS subcommands.
/// - `check`: Check if mdns discovery is available.
/// - `services`: List all discovered services.
#[derive(Debug, Clone)]
pub struct AdbMdns<'a>(AdbCommandBuilder<'a>);

impl<'a> AdbMdns<'a> {
    /// `check`: Check mDNS services.
    pub fn check(self) -> AdbMdnsCheck<'a> {
        AdbMdnsCheck(self.0)
    }

    /// `services`: List mDNS services.
    pub fn services(self) -> AdbMdnsServices<'a> {
        AdbMdnsServices(self.0)
    }
}

impl Adb {
    /// `mdns check | services`: Perform mDNS subcommands.
    /// - `check`: Check if mdns discovery is available.
    /// - `services`: List all discovered services.
    ///
    /// # Example
    ///
    /// - `adb mdns check`
    ///
    /// ```no_run
    /// # use adbr::{Adb, AdbCommand};
    /// # let adb = Adb::new();
    /// adb.mdns()
    ///     .check()
    ///     .status()
    ///     .expect("`adb mdns check` failed");
    /// ```
    ///
    /// - `adb mdns services`
    ///
    /// ```no_run
    /// # use adbr::{Adb, AdbCommand};
    /// # let adb = Adb::new();
    /// adb.mdns()
    ///     .services()
    ///     .status()
    ///     .expect("`adb mdns services` failed");
    /// ```
    pub fn mdns(&self) -> AdbMdns {
        AdbMdns(self.command())
    }
}

impl<'a> AdbCommandBuilder<'a> {
    /// `mdns check | services`: Perform mDNS subcommands.
    /// - `check`: Check if mdns discovery is available.
    /// - `services`: List all discovered services.
    ///
    /// See [`Adb::mdns`] for more information.
    pub fn mdns(self) -> AdbMdns<'a> {
        AdbMdns(self)
    }
}

/// A subcommand of `mdns`.
///
/// `mdns check`: Check if mdns discovery is available.
#[derive(Debug, Clone)]
pub struct AdbMdnsCheck<'a>(AdbCommandBuilder<'a>);

impl<'a> AdbCommand for AdbMdnsCheck<'a> {
    fn build(self) -> Command {
        let mut cmd = self.0.build();
        cmd.arg("mdns").arg("check");
        cmd
    }
}

/// A subcommand of `mdns`.
///
/// `mdns services`: List all discovered services.
#[derive(Debug, Clone)]
pub struct AdbMdnsServices<'a>(AdbCommandBuilder<'a>);

impl<'a> AdbCommand for AdbMdnsServices<'a> {
    fn build(self) -> Command {
        let mut cmd = self.0.build();
        cmd.arg("mdns").arg("services");
        cmd
    }
}
