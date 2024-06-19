//! Scripting Commands
//!
//! - `wait-for [-TRANSPORT] -STATE...`: Wait for device to be in a given state.
//! - `get-state`: Print `offline` | `bootloader` | `device`.
//! - `get-serialno`: Print `SERIAL_NUMBER`.
//! - `get-devpath`: Print `DEVICE_PATH`.
//! - `remount [-R]`: Remount partitions read-write.
//! - `reboot [bootloader|recovery|sideload|sideload-auto-reboot]`: Reboot the device; defaults to booting system image but supports bootloader and recovery too.
//! - `sideload`: Reboots into recovery and automatically starts sideload mode.
//! - `sideload-auto-reboot`: Same as sideload but reboots after sideloading.
//! - `sideload OTAPACKAGE`: Sideload the given full OTA package `OTAPACKAGE`.
//! - `root`: Restart adbd with root permissions.
//! - `unroot`: Restart adbd without root permissions.
//! - `usb`: Restart adbd listening on USB.
//! - `tcpip PORT`: Restart adbd listening on TCP on `PORT`.
//!
//! See [Scripting Commands](https://android.googlesource.com/platform/packages/modules/adb/+/refs/heads/master/docs/user/adb.1.md#scripting).

use std::ffi::OsStr;
use std::fmt::Display;
use std::process::Command;
use std::str::FromStr;

use crate::command::AdbCommandBuilder;
use crate::error::ParseError;
use crate::{Adb, AdbCommand, AdbError};

/// A device state to wait for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AdbWaitForState {
    Device,
    Recovery,
    Rescue,
    Sideload,
    Bootloader,
    Disconnect,
}

impl AsRef<OsStr> for AdbWaitForState {
    fn as_ref(&self) -> &OsStr {
        match self {
            AdbWaitForState::Device => OsStr::new("device"),
            AdbWaitForState::Recovery => OsStr::new("recovery"),
            AdbWaitForState::Rescue => OsStr::new("rescue"),
            AdbWaitForState::Sideload => OsStr::new("sideload"),
            AdbWaitForState::Bootloader => OsStr::new("bootloader"),
            AdbWaitForState::Disconnect => OsStr::new("disconnect"),
        }
    }
}

impl Display for AdbWaitForState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            AdbWaitForState::Device => "device",
            AdbWaitForState::Recovery => "recovery",
            AdbWaitForState::Rescue => "rescue",
            AdbWaitForState::Sideload => "sideload",
            AdbWaitForState::Bootloader => "bootloader",
            AdbWaitForState::Disconnect => "disconnect",
        })
    }
}

impl FromStr for AdbWaitForState {
    type Err = AdbError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "device" => Ok(AdbWaitForState::Device),
            "recovery" => Ok(AdbWaitForState::Recovery),
            "rescue" => Ok(AdbWaitForState::Rescue),
            "sideload" => Ok(AdbWaitForState::Sideload),
            "bootloader" => Ok(AdbWaitForState::Bootloader),
            "disconnect" => Ok(AdbWaitForState::Disconnect),
            _ => Err(AdbError::Parse(ParseError::with_description(
                s,
                "AdbDeviceState",
                "Unknown device state",
            ))),
        }
    }
}

/// The transport to wait for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum AdbWaitForTransport {
    Usb,
    Local,
    #[default]
    Any,
}

impl AsRef<OsStr> for AdbWaitForTransport {
    fn as_ref(&self) -> &OsStr {
        match self {
            AdbWaitForTransport::Usb => OsStr::new("usb"),
            AdbWaitForTransport::Local => OsStr::new("local"),
            AdbWaitForTransport::Any => OsStr::new("any"),
        }
    }
}

impl Display for AdbWaitForTransport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            AdbWaitForTransport::Usb => "usb",
            AdbWaitForTransport::Local => "local",
            AdbWaitForTransport::Any => "any",
        })
    }
}

impl FromStr for AdbWaitForTransport {
    type Err = AdbError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "usb" => Ok(AdbWaitForTransport::Usb),
            "local" => Ok(AdbWaitForTransport::Local),
            "any" => Ok(AdbWaitForTransport::Any),
            _ => Err(AdbError::Parse(ParseError::with_description(
                s,
                "AdbWaitForTransport",
                "Unknown transport",
            ))),
        }
    }
}

/// - `wait-for [-TRANSPORT] -STATE...`: Wait for device to be in a given state.
///   - `STATE`: `device`, `recovery`, `rescue`, `sideload`, `bootloader`, or `disconnect`.
///   - `TRANSPORT`: `usb`, `local`, or `any` (default=`any`).
#[derive(Debug, Clone)]
pub struct AdbWaitFor<'a> {
    acb: AdbCommandBuilder<'a>,
    /// `STATE`: `device`, `recovery`, `rescue`, `sideload`, `bootloader`, or `disconnect`.
    state: AdbWaitForState,
    /// `TRANSPORT`: `usb`, `local`, or `any` (default=`any`).
    transport: Option<AdbWaitForTransport>,
}

impl<'a> AdbWaitFor<'a> {
    fn new(acb: AdbCommandBuilder<'a>, state: AdbWaitForState) -> Self {
        Self {
            acb,
            state,
            transport: None,
        }
    }

    /// `STATE`: `device`, `recovery`, `rescue`, `sideload`, `bootloader`, or `disconnect`.
    ///
    /// The previous state will be overwritten.
    pub fn state(mut self, state: AdbWaitForState) -> Self {
        self.state = state;
        self
    }

    /// `TRANSPORT`: `usb`, `local`, or `any` (default=`any`).
    ///
    /// The previous transport will be overwritten.
    pub fn transport(mut self, transport: AdbWaitForTransport) -> Self {
        self.transport = Some(transport);
        self
    }
}

impl<'a> AdbCommand for AdbWaitFor<'a> {
    fn build(self) -> Command {
        let mut cmd = self.acb.build();
        let mut arg = OsStr::new("wait-for").to_os_string();
        if let Some(transport) = self.transport {
            arg.push("-");
            arg.push(transport);
        }
        arg.push("-");
        arg.push(self.state);
        cmd.arg(arg);
        cmd
    }
}

impl Adb {
    /// - `wait-for [-TRANSPORT] -STATE...`: Wait for device to be in a given state.
    ///
    /// # Examples
    ///
    /// `adb wait-for-device shell getprop`
    ///
    /// ```no_run
    /// # use adbr::{Adb, AdbCommand};
    /// # use adbr::command::AdbWaitForState;
    /// # let adb = Adb::new();
    /// adb.wait_for(AdbWaitForState::Device)
    ///     .build()
    ///     .arg("shell")
    ///     .arg("getprop")
    ///     .status()
    ///     .expect("`adb wait-for-device shell getprop` failed");
    /// ```
    pub fn wait_for(&self, state: AdbWaitForState) -> AdbWaitFor {
        AdbWaitFor::new(self.command(), state)
    }
}

impl<'a> AdbCommandBuilder<'a> {
    /// - `wait-for [-TRANSPORT] -STATE...`: Wait for device to be in a given state.
    ///
    /// See [`Adb::wait_for`] for more information.
    pub fn wait_for(self, state: AdbWaitForState) -> AdbWaitFor<'a> {
        AdbWaitFor::new(self, state)
    }
}

/// - `get-state`: Print `offline` | `bootloader` | `device`.
#[derive(Debug, Clone)]
pub struct AdbGetState<'a>(AdbCommandBuilder<'a>);

impl<'a> AdbCommand for AdbGetState<'a> {
    fn build(self) -> Command {
        let mut cmd = self.0.build();
        cmd.arg("get-state");
        cmd
    }
}

impl Adb {
    /// - `get-state`: Print `offline` | `bootloader` | `device`.
    ///
    /// # Examples
    ///
    /// `adb get-state`
    ///
    /// ```no_run
    /// # use adbr::{Adb, AdbCommand};
    /// # let adb = Adb::new();
    /// adb.get_state()
    ///     .status()
    ///     .expect("`adb get-state` failed");
    /// ```
    pub fn get_state(&self) -> AdbGetState {
        AdbGetState(self.command())
    }
}

impl<'a> AdbCommandBuilder<'a> {
    /// - `get-state`: Print `offline` | `bootloader` | `device`.
    ///
    /// See [`Adb::get_state`] for more information.
    pub fn get_state(self) -> AdbGetState<'a> {
        AdbGetState(self)
    }
}

/// - `get-serialno`: Print `SERIAL_NUMBER`.
#[derive(Debug, Clone)]
pub struct AdbGetSerialNo<'a>(AdbCommandBuilder<'a>);

impl<'a> AdbCommand for AdbGetSerialNo<'a> {
    fn build(self) -> Command {
        let mut cmd = self.0.build();
        cmd.arg("get-serialno");
        cmd
    }
}

impl Adb {
    /// - `get-serialno`: Print `SERIAL_NUMBER`.
    ///
    /// # Examples
    ///
    /// `adb get-serialno`
    ///
    /// ```no_run
    /// # use adbr::{Adb, AdbCommand};
    /// # let adb = Adb::new();
    /// adb.get_serial_no()
    ///     .status()
    ///     .expect("`adb get-serialno` failed");
    /// ```
    pub fn get_serial_no(&self) -> AdbGetSerialNo {
        AdbGetSerialNo(self.command())
    }
}

impl<'a> AdbCommandBuilder<'a> {
    /// - `get-serialno`: Print `SERIAL_NUMBER`.
    ///
    /// See [`Adb::get_serial_no`] for more information.
    pub fn get_serial_no(self) -> AdbGetSerialNo<'a> {
        AdbGetSerialNo(self)
    }
}

/// - `get-devpath`: Print `DEVICE_PATH`.
#[derive(Debug, Clone)]
pub struct AdbGetDevPath<'a>(AdbCommandBuilder<'a>);

impl<'a> AdbCommand for AdbGetDevPath<'a> {
    fn build(self) -> Command {
        let mut cmd = self.0.build();
        cmd.arg("get-devpath");
        cmd
    }
}

impl Adb {
    /// - `get-devpath`: Print `DEVICE_PATH`.
    ///
    /// # Examples
    ///
    /// `adb get-devpath`
    ///
    /// ```no_run
    /// # use adbr::{Adb, AdbCommand};
    /// # let adb = Adb::new();
    /// adb.get_dev_path()
    ///     .status()
    ///     .expect("`adb get-devpath` failed");
    /// ```
    pub fn get_dev_path(&self) -> AdbGetDevPath {
        AdbGetDevPath(self.command())
    }
}

impl<'a> AdbCommandBuilder<'a> {
    /// - `get-devpath`: Print `DEVICE_PATH`.
    ///
    /// See [`Adb::get_dev_path`] for more information.
    pub fn get_dev_path(self) -> AdbGetDevPath<'a> {
        AdbGetDevPath(self)
    }
}

/// - `remount [-R]`: Remount partitions read-write.
///   - `-R`: Automatically reboot the device.
#[allow(non_snake_case)]
#[derive(Debug, Clone)]
pub struct AdbRemount<'a> {
    acb: AdbCommandBuilder<'a>,
    /// `-R`: Automatically reboot the device.
    R: bool,
}

impl<'a> AdbRemount<'a> {
    fn new(acb: AdbCommandBuilder<'a>) -> Self {
        Self { acb, R: false }
    }

    /// `-R`: Automatically reboot the device.
    ///
    /// The previous reboot option will be overwritten.
    #[allow(non_snake_case)]
    pub fn R(mut self, value: bool) -> Self {
        self.R = value;
        self
    }
}

impl<'a> AdbCommand for AdbRemount<'a> {
    fn build(self) -> Command {
        let mut cmd = self.acb.build();
        cmd.arg("remount");
        if self.R {
            cmd.arg("-R");
        }
        cmd
    }
}

impl Adb {
    /// - `remount [-R]`: Remount partitions read-write.
    ///
    /// # Examples
    ///
    /// `adb remount -R`
    ///
    /// ```no_run
    /// # use adbr::{Adb, AdbCommand};
    /// # let adb = Adb::new();
    /// adb.remount()
    ///     .R(true)
    ///     .status()
    ///     .expect("`adb remount -R` failed");
    /// ```
    pub fn remount(&self) -> AdbRemount {
        AdbRemount::new(self.command())
    }
}

impl<'a> AdbCommandBuilder<'a> {
    /// - `remount [-R]`: Remount partitions read-write.
    ///
    /// See [`Adb::remount`] for more information.
    pub fn remount(self) -> AdbRemount<'a> {
        AdbRemount::new(self)
    }
}

/// The target to reboot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AdbRebootTarget {
    Bootloader,
    Recovery,
    Sideload,
    SideloadAutoReboot,
}

impl AsRef<OsStr> for AdbRebootTarget {
    fn as_ref(&self) -> &OsStr {
        match self {
            AdbRebootTarget::Bootloader => OsStr::new("bootloader"),
            AdbRebootTarget::Recovery => OsStr::new("recovery"),
            AdbRebootTarget::Sideload => OsStr::new("sideload"),
            AdbRebootTarget::SideloadAutoReboot => OsStr::new("sideload-auto-reboot"),
        }
    }
}

impl Display for AdbRebootTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            AdbRebootTarget::Bootloader => "bootloader",
            AdbRebootTarget::Recovery => "recovery",
            AdbRebootTarget::Sideload => "sideload",
            AdbRebootTarget::SideloadAutoReboot => "sideload-auto-reboot",
        })
    }
}

impl FromStr for AdbRebootTarget {
    type Err = AdbError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "bootloader" => Ok(AdbRebootTarget::Bootloader),
            "recovery" => Ok(AdbRebootTarget::Recovery),
            "sideload" => Ok(AdbRebootTarget::Sideload),
            "sideload-auto-reboot" => Ok(AdbRebootTarget::SideloadAutoReboot),
            _ => Err(AdbError::Parse(ParseError::with_description(
                s,
                "AdbRebootTarget",
                "Unknown reboot target",
            ))),
        }
    }
}

/// - `reboot [bootloader|recovery|sideload|sideload-auto-reboot]`: Reboot the device; defaults to booting system image but supports bootloader and recovery too.
#[derive(Debug, Clone)]
pub struct AdbReboot<'a> {
    acb: AdbCommandBuilder<'a>,
    /// The target to reboot, defaults to booting system image.
    target: Option<AdbRebootTarget>,
}

impl<'a> AdbReboot<'a> {
    fn new(acb: AdbCommandBuilder<'a>) -> Self {
        Self { acb, target: None }
    }

    /// The target to reboot.
    ///
    /// The previous target will be overwritten.
    pub fn target(mut self, target: AdbRebootTarget) -> Self {
        self.target = Some(target);
        self
    }
}

impl<'a> AdbCommand for AdbReboot<'a> {
    fn build(self) -> Command {
        let mut cmd = self.acb.build();
        cmd.arg("reboot");
        if let Some(target) = self.target {
            cmd.arg(target);
        }
        cmd
    }
}

impl Adb {
    /// - `reboot [bootloader|recovery|sideload|sideload-auto-reboot]`: Reboot the device; defaults to booting system image but supports bootloader and recovery too.
    ///
    /// # Examples
    ///
    /// `adb reboot`
    ///
    /// ```no_run
    /// # use adbr::{Adb, AdbCommand};
    /// # let adb = Adb::new();
    /// adb.reboot()
    ///     .status()
    ///     .expect("`adb reboot` failed");
    /// ```
    pub fn reboot(&self) -> AdbReboot {
        AdbReboot::new(self.command())
    }
}

impl<'a> AdbCommandBuilder<'a> {
    /// - `reboot [bootloader|recovery|sideload|sideload-auto-reboot]`: Reboot the device; defaults to booting system image but supports bootloader and recovery too.
    ///
    /// See [`Adb::reboot`] for more information.
    pub fn reboot(self) -> AdbReboot<'a> {
        AdbReboot::new(self)
    }
}

/// - `sideload`: Reboots into recovery and automatically starts sideload mode.
/// - `sideload OTAPACKAGE`: Sideload the given full OTA package `OTAPACKAGE`.
#[derive(Debug, Clone)]
pub struct AdbSideload<'a, S: AsRef<OsStr>> {
    acb: AdbCommandBuilder<'a>,
    /// The full OTA package to sideload.
    ota_package: Option<S>,
}

impl<'a, S: AsRef<OsStr>> AdbSideload<'a, S> {
    fn new(acb: AdbCommandBuilder<'a>) -> Self {
        Self {
            acb,
            ota_package: None,
        }
    }

    /// The full OTA package to sideload.
    ///
    /// The previous OTA package will be overwritten.
    pub fn ota_package<S1: AsRef<OsStr>>(self, ota_package: S1) -> AdbSideload<'a, S1> {
        AdbSideload {
            acb: self.acb,
            ota_package: Some(ota_package),
        }
    }
}

impl<'a, S: AsRef<OsStr>> AdbCommand for AdbSideload<'a, S> {
    fn build(self) -> Command {
        let mut cmd = self.acb.build();
        cmd.arg("sideload");
        if let Some(ota_package) = self.ota_package {
            cmd.arg(ota_package);
        }
        cmd
    }
}

impl Adb {
    /// - `sideload`: Reboots into recovery and automatically starts sideload mode.
    /// - `sideload OTAPACKAGE`: Sideload the given full OTA package `OTAPACKAGE`.
    ///
    /// # Examples
    ///
    /// `adb sideload`
    ///
    /// ```no_run
    /// # use adbr::{Adb, AdbCommand};
    /// # let adb = Adb::new();
    /// adb.sideload()
    ///     .status()
    ///     .expect("`adb sideload` failed");
    /// ```
    ///
    /// `adb sideload OTAPACKAGE`
    ///
    /// ```no_run
    /// # use adbr::{Adb, AdbCommand};
    /// # let adb = Adb::new();
    /// adb.sideload()
    ///     .ota_package("OTAPACKAGE")
    ///     .status()
    ///     .expect("`adb sideload OTAPACKAGE` failed");
    /// ```
    pub fn sideload(&self) -> AdbSideload<&str> {
        AdbSideload::new(self.command())
    }
}

impl<'a> AdbCommandBuilder<'a> {
    /// - `sideload`: Reboots into recovery and automatically starts sideload mode.
    /// - `sideload OTAPACKAGE`: Sideload the given full OTA package `OTAPACKAGE`.
    ///
    /// See [`Adb::sideload`] for more information.
    pub fn sideload(self) -> AdbSideload<'a, &'a str> {
        AdbSideload::new(self)
    }
}

/// - `sideload-auto-reboot`: Same as sideload but reboots after sideloading.
#[derive(Debug, Clone)]
pub struct AdbSideloadAutoReboot<'a>(AdbCommandBuilder<'a>);

impl<'a> AdbCommand for AdbSideloadAutoReboot<'a> {
    fn build(self) -> Command {
        let mut cmd = self.0.build();
        cmd.arg("sideload-auto-reboot");
        cmd
    }
}

impl Adb {
    /// - `sideload-auto-reboot`: Same as sideload but reboots after sideloading.
    ///
    /// # Examples
    ///
    /// `adb sideload-auto-reboot`
    ///
    /// ```no_run
    /// # use adbr::{Adb, AdbCommand};
    /// # let adb = Adb::new();
    /// adb.sideload_auto_reboot()
    ///     .status()
    ///     .expect("`adb sideload-auto-reboot` failed");
    /// ```
    pub fn sideload_auto_reboot(&self) -> AdbSideloadAutoReboot {
        AdbSideloadAutoReboot(self.command())
    }
}

impl<'a> AdbCommandBuilder<'a> {
    /// - `sideload-auto-reboot`: Same as sideload but reboots after sideloading.
    ///
    /// See [`Adb::sideload_auto_reboot`] for more information.
    pub fn sideload_auto_reboot(self) -> AdbSideloadAutoReboot<'a> {
        AdbSideloadAutoReboot(self)
    }
}

/// - `root`: Restart adbd with root permissions.
#[derive(Debug, Clone)]
pub struct AdbRoot<'a>(AdbCommandBuilder<'a>);

impl<'a> AdbCommand for AdbRoot<'a> {
    fn build(self) -> Command {
        let mut cmd = self.0.build();
        cmd.arg("root");
        cmd
    }
}

impl Adb {
    /// - `root`: Restart adbd with root permissions.
    ///
    /// # Examples
    ///
    /// `adb root`
    ///
    /// ```no_run
    /// # use adbr::{Adb, AdbCommand};
    /// # let adb = Adb::new();
    /// adb.root()
    ///     .status()
    ///     .expect("`adb root` failed");
    /// ```
    pub fn root(&self) -> AdbRoot {
        AdbRoot(self.command())
    }
}

impl<'a> AdbCommandBuilder<'a> {
    /// - `root`: Restart adbd with root permissions.
    ///
    /// See [`Adb::root`] for more information.
    pub fn root(self) -> AdbRoot<'a> {
        AdbRoot(self)
    }
}

/// - `unroot`: Restart adbd without root permissions.
#[derive(Debug, Clone)]
pub struct AdbUnroot<'a>(AdbCommandBuilder<'a>);

impl<'a> AdbCommand for AdbUnroot<'a> {
    fn build(self) -> Command {
        let mut cmd = self.0.build();
        cmd.arg("unroot");
        cmd
    }
}

impl Adb {
    /// - `unroot`: Restart adbd without root permissions.
    ///
    /// # Examples
    ///
    /// `adb unroot`
    ///
    /// ```no_run
    /// # use adbr::{Adb, AdbCommand};
    /// # let adb = Adb::new();
    /// adb.unroot()
    ///     .status()
    ///     .expect("`adb unroot` failed");
    /// ```
    pub fn unroot(&self) -> AdbUnroot {
        AdbUnroot(self.command())
    }
}

impl<'a> AdbCommandBuilder<'a> {
    /// - `unroot`: Restart adbd without root permissions.
    ///
    /// See [`Adb::unroot`] for more information.
    pub fn unroot(self) -> AdbUnroot<'a> {
        AdbUnroot(self)
    }
}

/// - `usb`: Restart adbd listening on USB.
#[derive(Debug, Clone)]
pub struct AdbUsb<'a>(AdbCommandBuilder<'a>);

impl<'a> AdbCommand for AdbUsb<'a> {
    fn build(self) -> Command {
        let mut cmd = self.0.build();
        cmd.arg("usb");
        cmd
    }
}

impl Adb {
    /// - `usb`: Restart adbd listening on USB.
    ///
    /// # Examples
    ///
    /// `adb usb`
    ///
    /// ```no_run
    /// # use adbr::{Adb, AdbCommand};
    /// # let adb = Adb::new();
    /// adb.usb()
    ///     .status()
    ///     .expect("`adb usb` failed");
    /// ```
    pub fn usb(&self) -> AdbUsb {
        AdbUsb(self.command())
    }
}

impl<'a> AdbCommandBuilder<'a> {
    /// - `usb`: Restart adbd listening on USB.
    ///
    /// See [`Adb::usb`] for more information.
    pub fn usb(self) -> AdbUsb<'a> {
        AdbUsb(self)
    }
}

/// - `tcpip PORT`: Restart adbd listening on TCP on `PORT`.
#[derive(Debug, Clone)]
pub struct AdbTcpIp<'a> {
    acb: AdbCommandBuilder<'a>,
    /// The port to listen on.
    port: u16,
}

impl<'a> AdbTcpIp<'a> {
    fn new(acb: AdbCommandBuilder<'a>, port: u16) -> Self {
        Self { acb, port }
    }

    /// The port to listen on.
    ///
    /// The previous port will be overwritten.
    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }
}

impl<'a> AdbCommand for AdbTcpIp<'a> {
    fn build(self) -> Command {
        let mut cmd = self.acb.build();
        cmd.arg("tcpip");
        cmd.arg(self.port.to_string());
        cmd
    }
}

impl Adb {
    /// - `tcpip PORT`: Restart adbd listening on TCP on `PORT`.
    ///
    /// # Examples
    ///
    /// `adb tcpip 5555`
    ///
    /// ```no_run
    /// # use adbr::{Adb, AdbCommand};
    /// # let adb = Adb::new();
    /// adb.tcp_ip(5555)
    ///     .status()
    ///     .expect("`adb tcpip 5555` failed");
    /// ```
    pub fn tcp_ip(&self, port: u16) -> AdbTcpIp {
        AdbTcpIp::new(self.command(), port)
    }
}

impl<'a> AdbCommandBuilder<'a> {
    /// - `tcpip PORT`: Restart adbd listening on TCP on `PORT`.
    ///
    /// See [`Adb::tcp_ip`] for more information.
    pub fn tcp_ip(self, port: u16) -> AdbTcpIp<'a> {
        AdbTcpIp::new(self, port)
    }
}
