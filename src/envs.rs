//! Environment variables of adb.
//!
//! [`AdbEnvs`] stores the values of adb environment variables used when building and executing adb commands.
//!
//! The following structs are provided to get and set the values of adb environment variables:
//!
//! - [`AdbTrace`]: Comma (or space) separated list of debug info to log.
//! - [`AdbVendorKeys`]: Colon-separated list of keys (files or directories).
//! - [`AndroidSerial`]: Serial number to connect to (see -s [`crate::AdbGlobalOption::Serial`]).
//! - [`AndroidLogTags`]: Tags to be used by logcat (see `logcat --help`).
//! - [`AdbLocalTransportMaxPort`]: Max emulator scan port (default 5585, 16 emulators).
//! - [`AdbMdnsAutoConnect`]: Comma-separated list of mdns services to allow auto-connect (default adb-tls-connect).
//! - [`AdbMdnsOpenScreen`]: The default mDNS-SD backend is Bonjour (mdnsResponder).
//!     For machines where Bonjour is not installed, adb can spawn its own, embedded, mDNS-SD back end, openscreen.
//!     If set to `1`, this env variable forces mDNS backend to openscreen.
//! - [`AdbLibusb`]: ADB has its own USB backend implementation but can also employ libusb.
//!     use `adb devices -l` (usb: prefix is omitted for libusb) or `adb host-features` (look for libusb in the output list) to identify which is in use.
//!     To override the default for your OS, set `ADB_LIBUSB` to `1` to enable libusb, or `0` to enable the ADB backend implementation.

use std::env::VarError;
use std::fmt::Display;
use std::ops::Deref;
use std::process::Command;
use std::str::FromStr;

use crate::error::ParseError;
use crate::{AdbError, AdbResult};

/// The values of adb environment variables used when building and executing adb commands.
///
/// The values may be set to [`None`] to remove the corresponding environment variable.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct AdbEnvs {
    /// `$ADB_TRACE`: Comma (or space) separated list of debug info to log.
    pub adb_trace: Option<AdbTrace>,
    /// `$ADB_VENDOR_KEYS`: Colon-separated list of keys (files or directories).
    pub adb_vendor_keys: Option<AdbVendorKeys>,
    /// `$ANDROID_SERIAL`: Serial number to connect to (see -s).
    pub android_serial: Option<AndroidSerial>,
    /// `$ANDROID_LOG_TAGS`: Tags to be used by logcat (see `logcat --help`).
    pub android_log_tags: Option<AndroidLogTags>,
    /// `$ADB_LOCAL_TRANSPORT_MAX_PORT`: Max emulator scan port (default 5585, 16 emulators).
    pub adb_local_transport_max_port: Option<AdbLocalTransportMaxPort>,
    /// `$ADB_MDNS_AUTO_CONNECT`: Comma-separated list of mdns services to allow auto-connect (default adb-tls-connect).
    pub adb_mdns_auto_connect: Option<AdbMdnsAutoConnect>,
    /// `$ADB_MDNS_OPENSCREEN`: The default mDNS-SD backend is Bonjour (mdnsResponder).
    /// For machines where Bonjour is not installed, adb can spawn its own, embedded, mDNS-SD back end, openscreen.
    /// If set to `1`, this env variable forces mDNS backend to openscreen.
    pub adb_mdns_openscreen: Option<AdbMdnsOpenScreen>,
    /// `$ADB_LIBUSB`: ADB has its own USB backend implementation but can also employ libusb.
    /// use `adb devices -l` (usb: prefix is omitted for libusb) or `adb host-features` (look for libusb in the output list) to identify which is in use.
    /// To override the default for your OS, set `ADB_LIBUSB` to `1` to enable libusb, or `0` to enable the ADB backend implementation.
    pub adb_libusb: Option<AdbLibusb>,
}

impl AdbEnvs {
    /// Creates a new [`AdbEnvs`] instance with the values inherited from the environment.
    ///
    /// If you want an empty environment, use [`AdbEnvs::default`] instead.
    pub fn new() -> AdbResult<Self> {
        Ok(Self {
            adb_trace: AdbTrace::get()?,
            adb_vendor_keys: AdbVendorKeys::get()?,
            android_serial: AndroidSerial::get()?,
            android_log_tags: AndroidLogTags::get()?,
            adb_local_transport_max_port: AdbLocalTransportMaxPort::get()?,
            adb_mdns_auto_connect: AdbMdnsAutoConnect::get()?,
            adb_mdns_openscreen: AdbMdnsOpenScreen::get()?,
            adb_libusb: AdbLibusb::get()?,
        })
    }

    /// Apply the current environment variable values to the given [`Command`].
    pub fn apply(&self, cmd: &mut Command) {
        _apply(self.adb_trace.as_ref(), cmd);
        _apply(self.adb_vendor_keys.as_ref(), cmd);
        _apply(self.android_serial.as_ref(), cmd);
        _apply(self.android_log_tags.as_ref(), cmd);
        _apply(self.adb_local_transport_max_port.as_ref(), cmd);
        _apply(self.adb_mdns_auto_connect.as_ref(), cmd);
        _apply(self.adb_mdns_openscreen.as_ref(), cmd);
        _apply(self.adb_libusb.as_ref(), cmd);
    }
}

/// Applies the value of an adb environment variable to a command.
#[inline]
fn _apply<T: AdbEnv>(var: Option<&T>, cmd: &mut Command) {
    match var {
        Some(var) => cmd.env(T::NAME, var.to_string()),
        None => cmd.env_remove(T::NAME),
    };
}

/// Gets and sets the value of an adb environment variable.
pub trait AdbEnv: FromStr<Err = AdbError> + ToString {
    /// The name of the corresponding environment variable.
    const NAME: &'static str;

    /// Gets the value of the corresponding environment variable.
    ///
    /// If the environment variable is not set, returns `Ok(None)`.
    ///
    /// # Errors
    ///
    /// Returns an error if the value is not valid Unicode or contains an unparsable value.
    fn get() -> AdbResult<Option<Self>> {
        match std::env::var(Self::NAME) {
            Ok(var) => Ok(Some(var.parse()?)),
            Err(VarError::NotPresent) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    /// Sets the value of the corresponding environment variable.
    ///
    /// # Safety
    ///
    /// See [`std::env::set_var`].
    fn set(&self) {
        std::env::set_var(Self::NAME, self.to_string());
    }
}

/// The possible values of `$ADB_TRACE`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AdbTraceEnum {
    All,
    Adb,
    Sockets,
    Packets,
    Rwx,
    Usb,
    Sync,
    Sysdeps,
    Transport,
    Jdwp,
    Services,
    Auth,
    Fdevent,
    Shell,
    Incremental,
    Mdns,
}

impl FromStr for AdbTraceEnum {
    type Err = AdbError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "all" => Ok(Self::All),
            "adb" => Ok(Self::Adb),
            "sockets" => Ok(Self::Sockets),
            "packets" => Ok(Self::Packets),
            "rwx" => Ok(Self::Rwx),
            "usb" => Ok(Self::Usb),
            "sync" => Ok(Self::Sync),
            "sysdeps" => Ok(Self::Sysdeps),
            "transport" => Ok(Self::Transport),
            "jdwp" => Ok(Self::Jdwp),
            "services" => Ok(Self::Services),
            "auth" => Ok(Self::Auth),
            "fdevent" => Ok(Self::Fdevent),
            "shell" => Ok(Self::Shell),
            "incremental" => Ok(Self::Incremental),
            "mdns" => Ok(Self::Mdns),
            _ => Err(AdbError::Parse(ParseError::with_description(
                s,
                "AdbTrace",
                "Unknown adb trace",
            ))),
        }
    }
}

impl Display for AdbTraceEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_ref())
    }
}

impl AsRef<str> for AdbTraceEnum {
    fn as_ref(&self) -> &str {
        match self {
            Self::All => "all",
            Self::Adb => "adb",
            Self::Sockets => "sockets",
            Self::Packets => "packets",
            Self::Rwx => "rwx",
            Self::Usb => "usb",
            Self::Sync => "sync",
            Self::Sysdeps => "sysdeps",
            Self::Transport => "transport",
            Self::Jdwp => "jdwp",
            Self::Services => "services",
            Self::Auth => "auth",
            Self::Fdevent => "fdevent",
            Self::Shell => "shell",
            Self::Incremental => "incremental",
            Self::Mdns => "mdns",
        }
    }
}

/// `$ADB_TRACE`: Comma (or space) separated list of debug info to log.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct AdbTrace(pub Vec<AdbTraceEnum>);

impl Deref for AdbTrace {
    type Target = [AdbTraceEnum];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for AdbTrace {
    type Err = AdbError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(AdbTrace(
            s.split(&[' ', ','])
                .map(|s| s.parse())
                .collect::<AdbResult<_>>()?,
        ))
    }
}

impl Display for AdbTrace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            &self
                .0
                .iter()
                .map(|t| t.as_ref())
                .collect::<Vec<_>>()
                .join(","),
        )
    }
}

impl AdbEnv for AdbTrace {
    const NAME: &'static str = "ADB_TRACE";
}

/// `$ADB_VENDOR_KEYS`: Colon-separated list of keys (files or directories).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct AdbVendorKeys(pub Vec<String>);

impl Deref for AdbVendorKeys {
    type Target = [String];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for AdbVendorKeys {
    type Err = AdbError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.split(':').map(|s| s.to_string()).collect()))
    }
}

impl Display for AdbVendorKeys {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.join(":"))
    }
}

impl AdbEnv for AdbVendorKeys {
    const NAME: &'static str = "ADB_VENDOR_KEYS";
}

/// `$ANDROID_SERIAL`: Serial number to connect to (see -s).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct AndroidSerial(pub String);

impl Deref for AndroidSerial {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for AndroidSerial {
    type Err = AdbError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}

impl Display for AndroidSerial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl AdbEnv for AndroidSerial {
    const NAME: &'static str = "ANDROID_SERIAL";
}

/// `$ANDROID_LOG_TAGS`: Tags to be used by logcat (see `logcat --help`).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct AndroidLogTags(pub String);

impl Deref for AndroidLogTags {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for AndroidLogTags {
    type Err = AdbError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}

impl Display for AndroidLogTags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl AdbEnv for AndroidLogTags {
    const NAME: &'static str = "ANDROID_LOG_TAGS";
}

/// `$ADB_LOCAL_TRANSPORT_MAX_PORT`: Max emulator scan port (default 5585, 16 emulators).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AdbLocalTransportMaxPort(pub u16);

impl Deref for AdbLocalTransportMaxPort {
    type Target = u16;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for AdbLocalTransportMaxPort {
    type Err = AdbError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse()
            .map(Self)
            .map_err(|_| AdbError::Parse(ParseError::with_description(s, "u16", "Invalid port")))
    }
}

impl Display for AdbLocalTransportMaxPort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Default for AdbLocalTransportMaxPort {
    fn default() -> Self {
        Self(5585)
    }
}

impl AdbEnv for AdbLocalTransportMaxPort {
    const NAME: &'static str = "ADB_LOCAL_TRANSPORT_MAX_PORT";
}

/// `$ADB_MDNS_AUTO_CONNECT`: Comma-separated list of mdns services to allow auto-connect (default adb-tls-connect).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct AdbMdnsAutoConnect(pub Vec<String>);

impl Deref for AdbMdnsAutoConnect {
    type Target = [String];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for AdbMdnsAutoConnect {
    type Err = AdbError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.split(',').map(|s| s.to_string()).collect()))
    }
}

impl Display for AdbMdnsAutoConnect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.join(","))
    }
}

impl AdbEnv for AdbMdnsAutoConnect {
    const NAME: &'static str = "ADB_MDNS_AUTO_CONNECT";
}

/// `$ADB_MDNS_OPENSCREEN`: The default mDNS-SD backend is Bonjour (mdnsResponder).
/// For machines where Bonjour is not installed, adb can spawn its own, embedded, mDNS-SD back end, openscreen.
/// If set to `1`, this env variable forces mDNS backend to openscreen.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct AdbMdnsOpenScreen(pub bool);

impl Deref for AdbMdnsOpenScreen {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for AdbMdnsOpenScreen {
    type Err = AdbError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s == "1"))
    }
}

impl Display for AdbMdnsOpenScreen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(if self.0 { "1" } else { "0" })
    }
}

impl AdbEnv for AdbMdnsOpenScreen {
    const NAME: &'static str = "ADB_MDNS_OPENSCREEN";
}

/// `$ADB_LIBUSB`: ADB has its own USB backend implementation but can also employ libusb.
/// use `adb devices -l` (usb: prefix is omitted for libusb) or `adb host-features` (look for libusb in the output list) to identify which is in use.
/// To override the default for your OS, set `ADB_LIBUSB` to `1` to enable libusb, or `0` to enable the ADB backend implementation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct AdbLibusb(pub bool);

impl FromStr for AdbLibusb {
    type Err = AdbError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Self(true)),
            "0" => Ok(Self(false)),
            _ => Err(AdbError::Parse(ParseError::with_description(
                s,
                "AdbLibUsb",
                "Invalid value",
            ))),
        }
    }
}

impl Display for AdbLibusb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(if self.0 { "1" } else { "0" })
    }
}

impl AdbEnv for AdbLibusb {
    const NAME: &'static str = "ADB_LIBUSB";
}
