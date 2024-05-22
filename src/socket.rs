//! This module provides some structs representing the adb socket families.

use std::fmt::{Display, Formatter};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6, ToSocketAddrs};
use std::str::FromStr;

use crate::error::{AdbError, ParseError};
use crate::AdbResult;

/// The address family of the `adb` command.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum AdbSocketFamily {
    Tcp(Tcp),
    LocalAbstract(LocalAbstract),
    LocalReserved(LocalReserved),
    LocalFileSystem(LocalFileSystem),
    Dev(Dev),
    DevRaw(DevRaw),
    Jdwp(Jdwp),
    Vsock(Vsock),
    AcceptFd(AcceptFd),
}

impl Display for AdbSocketFamily {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AdbSocketFamily::Tcp(inner) => write!(f, "{}", inner),
            AdbSocketFamily::LocalAbstract(inner) => write!(f, "{}", inner),
            AdbSocketFamily::LocalReserved(inner) => write!(f, "{}", inner),
            AdbSocketFamily::LocalFileSystem(inner) => write!(f, "{}", inner),
            AdbSocketFamily::Dev(inner) => write!(f, "{}", inner),
            AdbSocketFamily::DevRaw(inner) => write!(f, "{}", inner),
            AdbSocketFamily::Jdwp(inner) => write!(f, "{}", inner),
            AdbSocketFamily::Vsock(inner) => write!(f, "{}", inner),
            AdbSocketFamily::AcceptFd(inner) => write!(f, "{}", inner),
        }
    }
}

impl FromStr for AdbSocketFamily {
    type Err = AdbError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(tcp) = s.parse() {
            Ok(AdbSocketFamily::Tcp(tcp))
        } else if let Ok(local_abstract) = s.parse() {
            Ok(AdbSocketFamily::LocalAbstract(local_abstract))
        } else if let Ok(local_reserved) = s.parse() {
            Ok(AdbSocketFamily::LocalReserved(local_reserved))
        } else if let Ok(local_file_system) = s.parse() {
            Ok(AdbSocketFamily::LocalFileSystem(local_file_system))
        } else if let Ok(dev) = s.parse() {
            Ok(AdbSocketFamily::Dev(dev))
        } else if let Ok(dev_raw) = s.parse() {
            Ok(AdbSocketFamily::DevRaw(dev_raw))
        } else if let Ok(jdwp) = s.parse() {
            Ok(AdbSocketFamily::Jdwp(jdwp))
        } else if let Ok(vsock) = s.parse() {
            Ok(AdbSocketFamily::Vsock(vsock))
        } else if let Ok(accept_fd) = s.parse() {
            Ok(AdbSocketFamily::AcceptFd(accept_fd))
        } else {
            Err(AdbError::Parse(ParseError::with_description(
                s,
                "AdbSocketFamily",
                "invalid syntax",
            )))
        }
    }
}

impl From<Tcp> for AdbSocketFamily {
    fn from(tcp: Tcp) -> Self {
        AdbSocketFamily::Tcp(tcp)
    }
}

impl From<LocalAbstract> for AdbSocketFamily {
    fn from(local_abstract: LocalAbstract) -> Self {
        AdbSocketFamily::LocalAbstract(local_abstract)
    }
}

impl From<LocalReserved> for AdbSocketFamily {
    fn from(local_reserved: LocalReserved) -> Self {
        AdbSocketFamily::LocalReserved(local_reserved)
    }
}

impl From<LocalFileSystem> for AdbSocketFamily {
    fn from(local_file_system: LocalFileSystem) -> Self {
        AdbSocketFamily::LocalFileSystem(local_file_system)
    }
}

impl From<Dev> for AdbSocketFamily {
    fn from(dev: Dev) -> Self {
        AdbSocketFamily::Dev(dev)
    }
}

impl From<DevRaw> for AdbSocketFamily {
    fn from(dev_raw: DevRaw) -> Self {
        AdbSocketFamily::DevRaw(dev_raw)
    }
}

impl From<Jdwp> for AdbSocketFamily {
    fn from(jdwp: Jdwp) -> Self {
        AdbSocketFamily::Jdwp(jdwp)
    }
}

impl From<Vsock> for AdbSocketFamily {
    fn from(vsock: Vsock) -> Self {
        AdbSocketFamily::Vsock(vsock)
    }
}

impl From<AcceptFd> for AdbSocketFamily {
    fn from(accept_fd: AcceptFd) -> Self {
        AdbSocketFamily::AcceptFd(accept_fd)
    }
}

/// A TCP socket. Both IPv4 and IPv6 addresses are supported.
///
/// # Syntax
///
/// `tcp:[host:[port]]`
///
/// - `host`: Optional hostname or IP address.
///     If an IPv6 address is provided, it should be enclosed in square brackets.
/// - `port`: Optional port number.
///
/// # Note
///
/// Semantically, `host` and `port` should not be [`None`] at the same time.
///
/// In this case, the `Tcp` socket is considered invalid and behaves as follows:
/// - The [`Display`] implementation will return an empty string.
/// - The [`FromStr`] implementation will return an error.
///
/// ```
/// # use adbr::socket::Tcp;
/// assert!("tcp:".parse::<Tcp>().is_err());
/// assert_eq!(Tcp { ip: None, port: None }.to_string(), "");
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Tcp {
    // The IP address of the host.
    pub ip: Option<IpAddr>,
    // The port number.
    pub port: Option<u16>,
}

impl Tcp {
    /// Creates a new `Tcp` socket with the given IP address and port number.
    pub const fn new(host: IpAddr, port: u16) -> Self {
        Self {
            ip: Some(host),
            port: Some(port),
        }
    }

    /// Creates a new `Tcp` socket with the given IP address.
    pub const fn with_ip(host: IpAddr) -> Self {
        Self {
            ip: Some(host),
            port: None,
        }
    }

    /// Creates a new `Tcp` socket with the given IPv4 address.
    pub const fn with_ipv4(host: Ipv4Addr) -> Self {
        Self {
            ip: Some(IpAddr::V4(host)),
            port: None,
        }
    }

    /// Creates a new `Tcp` socket with the given IPv6 address.
    pub const fn with_ipv6(host: Ipv6Addr) -> Self {
        Self {
            ip: Some(IpAddr::V6(host)),
            port: None,
        }
    }

    /// Creates a new `Tcp` socket with the given port number.
    pub const fn with_port(port: u16) -> Self {
        Self {
            ip: None,
            port: Some(port),
        }
    }

    /// Resolves the given hostname into an IP address. If the resolution results
    /// in multiple IP addresses, the first IPv4 address is preferred.
    ///
    /// # Note
    ///
    /// The resolution may block the current thread while resolution is performed.
    /// If this is not desired, consider using [`FromStr`] which is non-blocking.
    ///
    /// # Examples
    ///
    /// ```
    /// use adbr::socket::Tcp;
    /// use std::net::Ipv4Addr;
    ///
    /// let tcp = Tcp::from_host("localhost").unwrap();
    /// assert_eq!(tcp, Tcp::with_ipv4(Ipv4Addr::new(127, 0, 0, 1)));
    /// ```
    pub fn from_host(host: &str) -> AdbResult<Self> {
        host.parse().or_else(|_| {
            Self::resolve(host).or_else(|e| {
                // ToSocketAddrs requires a hostname with a port number.
                // Retry if the input hostname does not contain a port number,
                match Self::resolve(&format!("{host}:0")) {
                    Ok(tcp) => Ok(Self::with_ip(tcp.ip.unwrap())),
                    _ => Err(e),
                }
            })
        })
    }

    fn resolve(host: &str) -> AdbResult<Self> {
        let mut addrs = host
            .to_socket_addrs()
            .map_err(|e| ParseError::with_source(host, "std::vec::IntoIter<SocketAddr>", e))?;
        let first = addrs.next();
        match first {
            None => Err(AdbError::Parse(ParseError::with_description(
                host,
                "SocketAddr",
                "no socket addresses found",
            ))),
            Some(SocketAddr::V4(v4)) => Ok(v4.into()),
            _ => Ok(addrs.find(SocketAddr::is_ipv4).or(first).unwrap().into()),
        }
    }
}

impl Display for Tcp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match (self.ip, self.port) {
            (Some(IpAddr::V4(v4)), Some(port)) => write!(f, "tcp:{}:{}", v4, port),
            (Some(IpAddr::V6(v6)), Some(port)) => write!(f, "tcp:[{}]:{}", v6, port),
            (Some(IpAddr::V4(v4)), None) => write!(f, "tcp:{}", v4),
            (Some(IpAddr::V6(v6)), None) => write!(f, "tcp:[{}]", v6),
            (None, Some(port)) => write!(f, "tcp:{}", port),
            (None, None) => write!(f, ""),
        }
    }
}

impl FromStr for Tcp {
    type Err = AdbError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.strip_prefix("tcp:") {
            None | Some("") => Err(AdbError::Parse(ParseError::with_description(
                s,
                "Tcp",
                "incomplete or invalid tcp syntax, expected `tcp:[host:[port]]`",
            ))),
            Some(value) => {
                if let Ok(port) = value.parse::<u16>() {
                    Ok(port.into())
                } else if let Ok(socket) = value.parse::<SocketAddr>() {
                    Ok(socket.into())
                } else if let Ok(v4) = value.parse::<Ipv4Addr>() {
                    Ok(v4.into())
                } else {
                    match value.strip_prefix('[').and_then(|v| v.strip_suffix(']')) {
                        None => Err(AdbError::Parse(ParseError::with_description(
                            value,
                            "Tcp",
                            "ipv6 address must be enclosed in square brackets",
                        ))),
                        Some(v) => match v.parse::<Ipv6Addr>() {
                            Ok(v6) => Ok(v6.into()),
                            Err(e) => Err(ParseError::with_source(value, "Ipv6Addr", e).into()),
                        },
                    }
                }
            }
        }
    }
}

impl From<SocketAddr> for Tcp {
    fn from(addr: SocketAddr) -> Self {
        Self::new(addr.ip(), addr.port())
    }
}

impl From<SocketAddrV4> for Tcp {
    fn from(addr: SocketAddrV4) -> Self {
        Self::new(IpAddr::V4(*addr.ip()), addr.port())
    }
}

impl From<SocketAddrV6> for Tcp {
    fn from(addr: SocketAddrV6) -> Self {
        Self::new(IpAddr::V6(*addr.ip()), addr.port())
    }
}

impl From<IpAddr> for Tcp {
    fn from(ip: IpAddr) -> Self {
        Self::with_ip(ip)
    }
}

impl From<Ipv4Addr> for Tcp {
    fn from(ipv4: Ipv4Addr) -> Self {
        Self::with_ipv4(ipv4)
    }
}

impl From<Ipv6Addr> for Tcp {
    fn from(ipv6: Ipv6Addr) -> Self {
        Self::with_ipv6(ipv6)
    }
}

impl From<u16> for Tcp {
    fn from(port: u16) -> Self {
        Self::with_port(port)
    }
}

/// A Unix domain socket in the abstract namespace.
///
/// # Syntax
///
/// `localabstract:<unix domain socket name>`
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct LocalAbstract(pub String);

/// A Unix domain socket in the reserved namespace.
///
/// # Syntax
///
///`localreserved:<unix domain socket name>`
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct LocalReserved(pub String);

/// A Unix domain socket in the file system.
///
/// # Syntax
///
/// `localfilesystem:<unix domain socket name>`
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct LocalFileSystem(pub String);

/// A character device.
///
/// # Syntax
///
/// `dev:<character device name>`
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Dev(pub String);

/// Open device in raw mode.
///
/// # Syntax
///
/// `dev-raw:<character device name>`
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct DevRaw(pub String);

/// A Java Debug Wire Protocol process.
///
/// # Syntax
///
/// `jdwp:<process pid>`
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Jdwp(pub u32);

/// A VSOCK address.
///
/// # Syntax
///
/// `vsock:<cid>:<port>`
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Vsock {
    pub cid: u32,
    pub port: u32,
}

impl Display for Vsock {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "vsock:{}:{}", self.cid, self.port)
    }
}

impl FromStr for Vsock {
    type Err = AdbError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once(':') {
            Some(("vsock", "")) => Err(AdbError::Parse(ParseError::with_description(
                s,
                "Vsock",
                "missing cid and port",
            ))),
            Some(("vsock", value)) => match value.split_once(':') {
                Some((cid, port)) => Ok(Self {
                    cid: cid
                        .parse()
                        .map_err(|e| ParseError::with_source(cid, "cid (u32)", e))?,
                    port: port
                        .parse()
                        .map_err(|e| ParseError::with_source(port, "port (u32)", e))?,
                }),
                None => Err(AdbError::Parse(ParseError::with_description(
                    value,
                    "Vsock",
                    "missing port",
                ))),
            },
            _ => Err(AdbError::Parse(ParseError::with_description(
                s,
                "Vsock",
                "invalid syntax, expected `vsock:<cid>:<port>`",
            ))),
        }
    }
}

/// A file descriptor for a socket.
///
/// # Syntax
///
/// `acceptfd:<fd>`
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct AcceptFd(pub u32);

/// implement [`Display`] for general adb socket families
macro_rules! display {
    ($(($ty:ty, $fmt:literal)),*) => {
        $(
            impl Display for $ty {
                fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                    write!(f, $fmt, self.0)
                }
            }
        )*
    };
}

display!(
    (LocalAbstract, "localabstract:{}"),
    (LocalReserved, "localreserved:{}"),
    (LocalFileSystem, "localfilesystem:{}"),
    (Dev, "dev:{}"),
    (DevRaw, "dev-raw:{}"),
    (Jdwp, "jdwp:{}"),
    (AcceptFd, "acceptfd:{}")
);

/// implement [`FromStr`] for general adb socket families
macro_rules! from_str {
    ($(($ty:ty, $key: literal, $value: literal)),*) => {
        $(
            impl FromStr for $ty {
                type Err = AdbError;

                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    match s.split_once(':') {
                        Some(($key, "")) => Err(AdbError::Parse(ParseError::with_description(
                            s,
                            stringify!($ty),
                            concat!("missing ", $value),
                        ))),
                        Some(($key, value)) => Ok(Self(value.parse().map_err(|e| ParseError::with_source(
                            value,
                            $value,
                            e,
                        ))?)),
                        _ => Err(AdbError::Parse(ParseError::with_description(
                            s,
                            stringify!($ty),
                            concat!("invalid syntax, expected `", $key, ":<", $value, ">`"),
                        ))),
                    }
                }
            }
        )*
    };
}

from_str!(
    (
        LocalAbstract,
        "localabstract",
        "unix domain socket name (string)"
    ),
    (
        LocalReserved,
        "localreserved",
        "unix domain socket name (string)"
    ),
    (
        LocalFileSystem,
        "localfilesystem",
        "unix domain socket name (string)"
    ),
    (Dev, "dev", "character device name (string)"),
    (DevRaw, "dev-raw", "character device name (string)"),
    (Jdwp, "jdwp", "process pid (u32)"),
    (AcceptFd, "acceptfd", "fd (u32)")
);

#[cfg(test)]
mod tests {
    use super::*;

    const TCP_COMMON: [(&str, Tcp); 5] = [
        ("tcp:5555", Tcp::with_port(5555)),
        ("tcp:127.0.0.1", Tcp::with_ipv4(Ipv4Addr::new(127, 0, 0, 1))),
        (
            "tcp:[::1]",
            Tcp::with_ipv6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)),
        ),
        (
            "tcp:127.0.0.1:5555",
            Tcp::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 5555),
        ),
        (
            "tcp:[::1]:5555",
            Tcp::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)), 5555),
        ),
    ];

    const TCP_PARSE_ERR: [&str; 30] = [
        "",
        "tcp:",
        // incomplete address
        "tcp:127.0",
        "tcp:127.0:5555",
        "tcp:[]",
        "tcp:[]:5555",
        "tcp:[:]",
        "tcp:[:5555]",
        "tcp:5555:",
        // Ipv6 address without square brackets
        "tcp:::",
        "tcp:::1",
        "tcp:::1:5555",
        "tcp:ffff::1:5555",
        "tcp:1111:2222:3333:4444:5555:6666:7777:8888",
        "tcp:1111:2222:3333:4444:5555:6666:7777:8888:5555",
        // IpAddr out of range
        "tcp:256.0.0.0",
        "tcp:256.-1.0.0",
        "tcp:[gggg::]",
        "tcp:[::gggg]",
        // port out of range
        "tcp:-1",
        "tcp:65536",
        // SocketAddr out of range
        "tcp:256.0.0.0:-1",
        "tcp:256.0.0.0:5555",
        "tcp:256.0.0.0:65536",
        "tcp:256.-1.0.0:5555",
        "tcp:[gggg::]:5555",
        "tcp:[::gggg]:5555",
        // invalid characters
        "tcp:abcd",
        "tcp:a.b.c.d",
        "tcp:a.b.c.d:p",
    ];

    #[test]
    fn test_tcp_display() {
        for (s, tcp) in TCP_COMMON {
            assert_eq!(s, tcp.to_string());
        }
    }

    #[test]
    fn test_tcp_parse() {
        for (s, tcp) in TCP_COMMON {
            assert_eq!(tcp, s.parse().unwrap());
        }
        for s in TCP_PARSE_ERR {
            assert!(s.parse::<Tcp>().is_err(), "{}", s);
        }
    }

    const TCP_RESOLVE_OK: [(&str, Tcp); 2] = [
        (
            "localhost:5555",
            Tcp::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 5555),
        ),
        ("localhost", Tcp::with_ipv4(Ipv4Addr::new(127, 0, 0, 1))),
    ];

    const TCP_RESOLVE_ERR: [&str; 6] = [
        "local-host",
        "local-host:5555",
        "localhost:",
        "abcd",
        "a.b.c.d",
        "a.b.c.d:p",
    ];

    #[test]
    fn test_tcp_resolve() {
        for (s, tcp) in TCP_RESOLVE_OK {
            assert_eq!(tcp, Tcp::from_host(s).unwrap());
        }
        for s in TCP_RESOLVE_ERR {
            assert!(Tcp::from_host(s).is_err(), "{}", s);
        }
    }

    #[test]
    fn test_local_abstract_display() {
        let local_abstract = LocalAbstract("socket".to_string());
        assert_eq!("localabstract:socket", local_abstract.to_string());
    }

    #[test]
    fn test_local_abstract_parse() {
        let local_abstract = LocalAbstract("socket".to_string());
        assert_eq!(local_abstract, "localabstract:socket".parse().unwrap());
        let err = ["localabstract", "localabstract:"];
        for s in err {
            assert!(s.parse::<LocalAbstract>().is_err(), "{}", s);
        }
    }

    #[test]
    fn test_local_reserved_display() {
        let local_reserved = LocalReserved("socket".to_string());
        assert_eq!("localreserved:socket", local_reserved.to_string());
    }

    #[test]
    fn test_local_reserved_parse() {
        let local_reserved = LocalReserved("socket".to_string());
        assert_eq!(local_reserved, "localreserved:socket".parse().unwrap());
        let err = ["localreserved", "localreserved:"];
        for s in err {
            assert!(s.parse::<LocalReserved>().is_err(), "{}", s);
        }
    }

    #[test]
    fn test_local_file_system_display() {
        let local_file_system = LocalFileSystem(String::from("/path/to/socket"));
        assert_eq!(
            "localfilesystem:/path/to/socket",
            local_file_system.to_string()
        );
    }

    #[test]
    fn test_local_file_system_parse() {
        let local_file_system = LocalFileSystem(String::from("/path/to/socket"));
        assert_eq!(
            local_file_system,
            "localfilesystem:/path/to/socket".parse().unwrap()
        );
        let err = ["localfilesystem", "localfilesystem:"];
        for s in &err {
            assert!(s.parse::<LocalFileSystem>().is_err(), "{}", s);
        }
    }

    #[test]
    fn test_dev_display() {
        let dev = Dev(String::from("/dev/tty"));
        assert_eq!("dev:/dev/tty", dev.to_string());
    }

    #[test]
    fn test_dev_parse() {
        let dev = Dev(String::from("/dev/tty"));
        assert_eq!(dev, "dev:/dev/tty".parse().unwrap());
        let err = ["dev", "dev:"];
        for s in &err {
            assert!(s.parse::<Dev>().is_err(), "{}", s);
        }
    }

    #[test]
    fn test_dev_raw_display() {
        let dev_raw = DevRaw(String::from("/dev/tty"));
        assert_eq!("dev-raw:/dev/tty", dev_raw.to_string());
    }

    #[test]
    fn test_dev_raw_parse() {
        let dev_raw = DevRaw(String::from("/dev/tty"));
        assert_eq!(dev_raw, "dev-raw:/dev/tty".parse().unwrap());
        let err = ["dev-raw", "dev-raw:"];
        for s in &err {
            assert!(s.parse::<DevRaw>().is_err(), "{}", s);
        }
    }

    const OVERFLOW: u64 = u32::MAX as u64 + 1;

    #[test]
    fn test_jdwp_display() {
        let jdwp = Jdwp(1234);
        assert_eq!("jdwp:1234", jdwp.to_string());
    }

    #[test]
    fn test_jdwp_parse() {
        let jdwp = Jdwp(1234);
        assert_eq!(jdwp, "jdwp:1234".parse().unwrap());
        let err = ["jdwp", "jdwp:", "jdwp:-1", &format!("jdwp:{}", OVERFLOW)];
        for s in &err {
            assert!(s.parse::<Jdwp>().is_err(), "{}", s);
        }
    }

    #[test]
    fn test_vsock_display() {
        let vsock = Vsock { cid: 1, port: 2 };
        assert_eq!("vsock:1:2", vsock.to_string());
    }

    #[test]
    fn test_vsock_parse() {
        let vsock = Vsock { cid: 1, port: 2 };
        assert_eq!(vsock, "vsock:1:2".parse().unwrap());
        let err = [
            "vsock",
            "vsock:",
            "vsock:1",
            "vsock::1",
            "vsock:1:",
            "vsock:-1",
            "vsock:-1:-1",
            &format!("vsock:1:{}", OVERFLOW),
            &format!("vsock:{}:2", OVERFLOW),
            &format!("vsock:{}:{}", OVERFLOW, OVERFLOW),
        ];
        for s in &err {
            assert!(s.parse::<Vsock>().is_err(), "{}", s);
        }
    }

    #[test]
    fn test_accept_fd_display() {
        let accept_fd = AcceptFd(1);
        assert_eq!("acceptfd:1", accept_fd.to_string());
    }

    #[test]
    fn test_accept_fd_parse() {
        let accept_fd = AcceptFd(1);
        assert_eq!(accept_fd, "acceptfd:1".parse().unwrap());
        let err = [
            "acceptfd",
            "acceptfd:",
            "acceptfd:-1",
            &format!("acceptfd:{}", OVERFLOW),
        ];
        for s in &err {
            assert!(s.parse::<AcceptFd>().is_err(), "{}", s);
        }
    }
}
