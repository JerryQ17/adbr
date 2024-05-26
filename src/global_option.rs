//! The global options of the `adb` command.

use std::fmt::Display;
use std::net::IpAddr;
use std::str::FromStr;

use crate::command::AdbCommandBuilder;
use crate::error::{AdbError, ParseError};
use crate::socket::Tcp;
use crate::{Adb, AdbResult};

/// The global options of the `adb` command.
///
/// - [`AdbGlobalOption::ListenAll`] `-a` Listen on all network interfaces, not just localhost.
/// - [`AdbGlobalOption::Usb`] `-d` Use USB device (error if multiple devices connected).
/// - [`AdbGlobalOption::TcpIp`] `-e` Use TCP/IP device (error if multiple TCP/IP devices available).
/// - [`AdbGlobalOption::Serial`] `-s SERIAL` Use device with given SERIAL (overrides $ANDROID_SERIAL).
/// - [`AdbGlobalOption::OneDevice`] `-t ID` Use device with given transport ID.
/// - [`AdbGlobalOption::Host`] `-H` Name of adb server host (default=`localhost`).
/// - [`AdbGlobalOption::Port`] `-P *PORT` Smart socket PORT of adb server (default=`5037`).
/// - [`AdbGlobalOption::Listen`] `-L SOCKET` Listen on given socket for adb server (default=`tcp:localhost:5037`).
/// - [`AdbGlobalOption::OneDevice`] `--one-device SERIAL | USB` Server will only connect to one USB device,
///     specified by a SERIAL number or USB device address (only with ‘start-server’ or ‘server nodaemon’).
/// - [`AdbGlobalOption::ExitOnWriteError`] `--exit-on-write-error` Exit if stdout is closed.
///
/// See [GLOBAL OPTIONS](https://android.googlesource.com/platform/packages/modules/adb/+/refs/heads/master/docs/user/adb.1.md#global-options)
///
/// # Examples
///
/// ```
/// # use std::net::{IpAddr, Ipv4Addr};
/// # use adbr::global_option::AdbGlobalOption;
/// # use adbr::socket::Tcp;
/// assert_eq!("-a".parse::<AdbGlobalOption>().unwrap(), AdbGlobalOption::ListenAll);
/// assert_eq!(
///     "-s emulator-123".parse::<AdbGlobalOption>().unwrap(),
///     AdbGlobalOption::Serial("emulator-123".to_string())
/// );
/// assert_eq!(
///     "-L tcp:127.0.0.1:8080".parse::<AdbGlobalOption>().unwrap(),
///     AdbGlobalOption::Listen(Tcp{
///         ip: Some(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))),
///         port: Some(8080)
///     })
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AdbGlobalOption {
    /// `-a`: Listen on all network interfaces, not just localhost.
    ListenAll,
    /// `-d`: Use USB device (error if multiple devices connected).
    Usb,
    /// `-e`: Use TCP/IP device (error if multiple TCP/IP devices available).
    TcpIp,
    /// `-s SERIAL`: Use device with given SERIAL (overrides $ANDROID_SERIAL).
    Serial(String),
    /// `-t ID`: Use device with given transport id.
    TransportId(String),
    /// `-H`: Name of adb server host. Default is `localhost`.
    Host(IpAddr),
    /// `-P *PORT`: Smart socket PORT of adb server. Default is `5037`.
    Port(u16),
    /// `-L SOCKET`: Listen on given socket for adb server. Default is `tcp:localhost:5037`.
    Listen(Tcp),
    /// `--one-device SERIAL | USB`:
    /// Server will only connect to one USB device, specified by a SERIAL number or USB device address
    /// (only with `start-server` or `server nodaemon`).
    OneDevice(String),
    /// `--exit-on-write-error`: Exit if stdout is closed.
    ExitOnWriteError,
}

impl AdbGlobalOption {
    /// Parse a string slice into a [`AdbGlobalOption`], resolving the hostname if needed.
    ///
    /// This only affects variant [`AdbGlobalOption::Host`] and [`AdbGlobalOption::Listen`].
    /// When converting to other variants, this function behaves the same as [`FromStr`].
    ///
    /// # Note
    ///
    /// The resolution may block the current thread while resolution is performed.
    /// If this is not desired, consider using [`FromStr`] which is non-blocking.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::net::{IpAddr, Ipv4Addr};
    /// # use adbr::global_option::AdbGlobalOption;
    /// # use adbr::socket::Tcp;
    /// assert_eq!(
    ///     AdbGlobalOption::from_host("-L tcp:localhost:8080").unwrap(),
    ///     AdbGlobalOption::Listen(Tcp{
    ///         ip: Some(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))),
    ///         port: Some(8080)
    ///     })
    /// );
    /// ```
    pub fn from_host(s: &str) -> AdbResult<Self> {
        Self::from_str_helper::<true>(s)
    }

    fn from_str_helper<const RESOLVE: bool>(s: &str) -> AdbResult<Self> {
        let trimmed = s.trim();
        // 1. Options that don't require a value.
        match trimmed {
            "-a" => return Ok(Self::ListenAll),
            "-d" => return Ok(Self::Usb),
            "-e" => return Ok(Self::TcpIp),
            "--exit-on-write-error" => return Ok(Self::ExitOnWriteError),
            _ => {}
        };
        // 2. Split the value into the option and its value.
        let (opt, val) = trimmed
            .split_once(char::is_whitespace)
            .map(|(opt, val)| (opt, val.trim()))
            .ok_or_else(|| ParseError::with_description("no value", "&str", "missing value"))?;
        // 3. Options that require a value.
        match opt {
            "-s" => Ok(Self::Serial(val.to_string())),
            "-t" => Ok(Self::TransportId(val.to_string())),
            "-H" => {
                if RESOLVE {
                    Ok(Self::Host(
                        Tcp::from_host(&format!("tcp:{val}"))?.ip.ok_or_else(|| {
                            ParseError::with_description(val, "IpAddr", "missing ip")
                        })?,
                    ))
                } else {
                    val.parse()
                        .map(Self::Host)
                        .map_err(|e| ParseError::with_source(val, "IpAddr", e).into())
                }
            }
            "-P" => val
                .parse()
                .map(Self::Port)
                .map_err(|e| ParseError::with_source(val, "port (u16)", e).into()),
            "-L" => {
                if RESOLVE {
                    Ok(Self::Listen(Tcp::from_host(val)?))
                } else {
                    val.parse().map(Self::Listen)
                }
            }
            "--one-device" => Ok(Self::OneDevice(val.to_string())),
            _ => Err(ParseError::with_description(opt, "GlobalOption", "unknown option").into()),
        }
    }
}

impl FromStr for AdbGlobalOption {
    type Err = AdbError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str_helper::<false>(s)
    }
}

impl Display for AdbGlobalOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ListenAll => write!(f, "-a"),
            Self::Usb => write!(f, "-d"),
            Self::TcpIp => write!(f, "-e"),
            Self::Serial(serial) => write!(f, "-s {}", serial),
            Self::TransportId(id) => write!(f, "-t {}", id),
            Self::Host(ip) => write!(f, "-H {}", ip),
            Self::Port(port) => write!(f, "-P {}", port),
            Self::Listen(addr) => write!(f, "-L {}", addr),
            Self::OneDevice(device) => write!(f, "--one-device {}", device),
            Self::ExitOnWriteError => write!(f, "--exit-on-write-error"),
        }
    }
}

impl Adb {
    /// `-a`: Listen on all network interfaces, not just localhost.
    pub fn listen_all(&self) -> AdbCommandBuilder {
        self.command().listen_all()
    }

    /// `-d`: Use USB device (error if multiple devices connected).
    pub fn usb(&self) -> AdbCommandBuilder {
        self.command().usb()
    }

    /// `-e`: Use TCP/IP device (error if multiple TCP/IP devices available).
    pub fn tcp_ip(&self) -> AdbCommandBuilder {
        self.command().tcp_ip()
    }

    /// `-s SERIAL`: Use device with given SERIAL (overrides $ANDROID_SERIAL).
    pub fn serial<S: ToString>(&self, serial: S) -> AdbCommandBuilder {
        self.command().serial(serial)
    }

    /// `-t ID`: Use device with given transport id.
    pub fn transport_id<S: ToString>(&self, id: S) -> AdbCommandBuilder {
        self.command().transport_id(id)
    }

    /// `-H`: Name of adb server host. Default is `localhost`.
    pub fn host<A: Into<IpAddr>>(&self, host: A) -> AdbCommandBuilder {
        self.command().host(host)
    }

    /// `-H`: Name of adb server host. Default is `localhost`.
    ///
    /// This will resolve the hostname to an IP address. See [`Tcp::from_host`] for more information.
    pub fn host_resolved(&self, host: &str) -> AdbResult<AdbCommandBuilder> {
        self.command().host_resolved(host)
    }

    /// `-P *PORT`: Smart socket PORT of adb server. Default is `5037`.
    pub fn port(&self, port: u16) -> AdbCommandBuilder {
        self.command().port(port)
    }

    /// `-L SOCKET`: Listen on given socket for adb server. Default is `tcp:localhost:5037`.
    pub fn listen(&self, addr: Tcp) -> AdbCommandBuilder {
        self.command().listen(addr)
    }

    /// `-L SOCKET`: Listen on given socket for adb server. Default is `tcp:localhost:5037`.
    ///
    /// This will resolve the hostname to an IP address. See [`Tcp::from_host`] for more information.
    pub fn listen_resolved(&self, addr: &str) -> AdbResult<AdbCommandBuilder> {
        self.command().listen_resolved(addr)
    }

    /// `--one-device SERIAL | USB`:
    /// Server will only connect to one USB device, specified by a SERIAL number or USB device address
    /// (only with `start-server` or `server nodaemon`).
    pub fn one_device<S: ToString>(&self, device: S) -> AdbCommandBuilder {
        self.command().one_device(device)
    }

    /// `--exit-on-write-error`: Exit if stdout is closed.
    pub fn exit_on_write_error(&self) -> AdbCommandBuilder {
        self.command().exit_on_write_error()
    }
}

impl<'a> AdbCommandBuilder<'a> {
    /// `-a`: Listen on all network interfaces, not just localhost.
    pub fn listen_all(self) -> Self {
        self.add_global_option_unchecked(AdbGlobalOption::ListenAll)
    }

    /// `-d`: Use USB device (error if multiple devices connected).
    pub fn usb(self) -> Self {
        self.add_global_option_unchecked(AdbGlobalOption::Usb)
    }

    /// `-e`: Use TCP/IP device (error if multiple TCP/IP devices available).
    pub fn tcp_ip(self) -> Self {
        self.add_global_option_unchecked(AdbGlobalOption::TcpIp)
    }

    /// `-s SERIAL`: Use device with given SERIAL (overrides $ANDROID_SERIAL).
    ///
    /// If a `-s SERIAL` option already exists, it will be replaced.
    pub fn serial<S: ToString>(self, serial: S) -> Self {
        self.add_global_option(AdbGlobalOption::Serial(serial.to_string()), |opt| {
            matches!(opt, AdbGlobalOption::Serial(_))
        })
    }

    /// `-t ID`: Use device with given transport id.
    ///
    /// If a `-t ID` option already exists, it will be replaced.
    pub fn transport_id<S: ToString>(self, id: S) -> Self {
        self.add_global_option(AdbGlobalOption::TransportId(id.to_string()), |opt| {
            matches!(opt, AdbGlobalOption::TransportId(_))
        })
    }

    /// `-H`: Name of adb server host. Default is `localhost`.
    ///
    /// If you want to resolve the hostname, use [`Self::host_resolved`] instead.
    pub fn host<A: Into<IpAddr>>(self, host: A) -> Self {
        self.add_global_option(AdbGlobalOption::Host(host.into()), |opt| {
            matches!(opt, AdbGlobalOption::Host(_))
        })
    }

    /// `-H`: Name of adb server host. Default is `localhost`.
    ///
    /// This will resolve the hostname to an IP address. See [`Tcp::from_host`] for more information.
    pub fn host_resolved(self, host: &str) -> AdbResult<Self> {
        Ok(self.add_global_option(
            AdbGlobalOption::Host(
                Tcp::from_host(&format!("tcp:{}", host))?
                    .ip
                    .ok_or_else(|| ParseError::with_description(host, "IpAddr", "missing ip"))?,
            ),
            |opt| matches!(opt, AdbGlobalOption::Host(_)),
        ))
    }

    /// `-P *PORT`: Smart socket PORT of adb server. Default is `5037`.
    pub fn port(self, port: u16) -> Self {
        self.add_global_option(AdbGlobalOption::Port(port), |opt| {
            matches!(opt, AdbGlobalOption::Port(_))
        })
    }

    /// `-L SOCKET`: Listen on given socket for adb server. Default is `tcp:localhost:5037`.
    ///
    /// If you want to resolve the hostname, use [`Self::listen_resolved`] instead.
    pub fn listen(self, addr: Tcp) -> Self {
        self.add_global_option(AdbGlobalOption::Listen(addr), |opt| {
            matches!(opt, AdbGlobalOption::Listen(_))
        })
    }

    /// `-L SOCKET`: Listen on given socket for adb server. Default is `tcp:localhost:5037`.
    ///
    /// This will resolve the hostname to an IP address. See [`Tcp::from_host`] for more information.
    pub fn listen_resolved(self, addr: &str) -> AdbResult<Self> {
        Ok(
            self.add_global_option(AdbGlobalOption::Listen(Tcp::from_host(addr)?), |opt| {
                matches!(opt, AdbGlobalOption::Listen(_))
            }),
        )
    }

    /// `--one-device SERIAL | USB`:
    /// Server will only connect to one USB device, specified by a SERIAL number or USB device address
    /// (only with `start-server` or `server nodaemon`).
    pub fn one_device<S: ToString>(self, device: S) -> Self {
        self.add_global_option(AdbGlobalOption::OneDevice(device.to_string()), |opt| {
            matches!(opt, AdbGlobalOption::OneDevice(_))
        })
    }

    /// `--exit-on-write-error`: Exit if stdout is closed.
    pub fn exit_on_write_error(self) -> Self {
        self.add_global_option_unchecked(AdbGlobalOption::ExitOnWriteError)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

    fn test_loop<T: AsRef<str>>(arr: &[(T, AdbGlobalOption)]) {
        for (s, expected) in arr {
            assert_eq!(
                s.as_ref().parse::<AdbGlobalOption>().unwrap(),
                expected.clone()
            );
            assert_eq!(
                format!("\r\n\t {} \t\r\n", s.as_ref())
                    .parse::<AdbGlobalOption>()
                    .unwrap(),
                expected.clone()
            );
            assert!(AdbGlobalOption::from_str(&format!("-{}", s.as_ref())).is_err());
        }
    }

    #[test]
    fn test_from_str_with_no_value() {
        test_loop(&[
            ("-a", AdbGlobalOption::ListenAll),
            ("-d", AdbGlobalOption::Usb),
            ("-e", AdbGlobalOption::TcpIp),
            ("--exit-on-write-error", AdbGlobalOption::ExitOnWriteError),
        ]);
    }

    #[test]
    fn test_from_str_with_string_value() {
        let values = ["123", "test", "emulator-123", "127.0.0.1:1234"];
        let types = [
            (
                "-s",
                AdbGlobalOption::Serial as fn(String) -> AdbGlobalOption,
            ),
            (
                "-t",
                AdbGlobalOption::TransportId as fn(String) -> AdbGlobalOption,
            ),
            (
                "--one-device",
                AdbGlobalOption::OneDevice as fn(String) -> AdbGlobalOption,
            ),
        ];
        for (opt, f) in types {
            test_loop(
                &values
                    .iter()
                    .map(|s| (format!("{} {}", opt, s), f(s.to_string())))
                    .collect::<Vec<_>>(),
            );
            test_loop(
                &values
                    .iter()
                    .map(|s| (format!("{}  {}", opt, s), f(s.to_string())))
                    .collect::<Vec<_>>(),
            );
            test_loop(
                &values
                    .iter()
                    .map(|s| (format!("{}\t{}", opt, s), f(s.to_string())))
                    .collect::<Vec<_>>(),
            );
        }
    }

    #[test]
    fn test_from_str_net() {
        let values = [
            (
                "-H 127.0.0.1",
                AdbGlobalOption::Host(Ipv4Addr::new(127, 0, 0, 1).into()),
            ),
            (
                "-L tcp:127.0.0.1",
                AdbGlobalOption::Listen(Tcp {
                    ip: Some(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))),
                    port: None,
                }),
            ),
            (
                "-L tcp:127.0.0.1:1234",
                AdbGlobalOption::Listen(Tcp {
                    ip: Some(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))),
                    port: Some(1234),
                }),
            ),
        ];
        for (s, expected) in values {
            assert_eq!(s.parse::<AdbGlobalOption>().unwrap(), expected);
        }
    }

    #[test]
    fn test_from_str_net_host() {
        let values = [
            (
                "-H localhost",
                AdbGlobalOption::Host(Ipv4Addr::new(127, 0, 0, 1).into()),
            ),
            (
                "-L tcp:localhost",
                AdbGlobalOption::Listen(Tcp::with_ipv4(Ipv4Addr::new(127, 0, 0, 1))),
            ),
            (
                "-L tcp:localhost:1234",
                AdbGlobalOption::Listen(Tcp::new(Ipv4Addr::new(127, 0, 0, 1).into(), 1234)),
            ),
        ];
        for (s, expected) in values {
            assert_eq!(AdbGlobalOption::from_host(s).unwrap(), expected);
        }
    }
}
