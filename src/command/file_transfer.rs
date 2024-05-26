//! File transfer commands.
//!
//! - `push [--sync] [-z ALGORITHM] [-Z] LOCAL... REMOTE`: Copy local files/directories to device.
//! - `pull [-a] [-z ALGORITHM] [-Z] REMOTE... LOCAL`: Copy files/dirs from device
//! - `sync [-l] [-z ALGORITHM] [-Z] [all|data|odm|oem|product|system|system_ext|vendor]`:
//!     Sync a local build from `$ANDROID_PRODUCT_OUT` to the device (default `all`)
//!
//! See [File Transfer Commands](https://android.googlesource.com/platform/packages/modules/adb/+/refs/heads/master/docs/user/adb.1.md#file-transfer)

use std::ffi::OsStr;
use std::fmt::Display;
use std::process::Command;
use std::str::FromStr;

use crate::command::AdbCommandBuilder;
use crate::error::ParseError;
use crate::{Adb, AdbCommand, AdbError};

/// Compression algorithm for file transfer commands.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum AdbCompressionAlgorithm {
    Any,
    None,
    Brotli,
    Lz4,
    Zstd,
}

impl AsRef<OsStr> for AdbCompressionAlgorithm {
    fn as_ref(&self) -> &OsStr {
        match self {
            AdbCompressionAlgorithm::Any => OsStr::new("any"),
            AdbCompressionAlgorithm::None => OsStr::new("none"),
            AdbCompressionAlgorithm::Brotli => OsStr::new("brotli"),
            AdbCompressionAlgorithm::Lz4 => OsStr::new("lz4"),
            AdbCompressionAlgorithm::Zstd => OsStr::new("zstd"),
        }
    }
}

impl Display for AdbCompressionAlgorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            AdbCompressionAlgorithm::Any => "any",
            AdbCompressionAlgorithm::None => "none",
            AdbCompressionAlgorithm::Brotli => "brotli",
            AdbCompressionAlgorithm::Lz4 => "lz4",
            AdbCompressionAlgorithm::Zstd => "zstd",
        })
    }
}

impl FromStr for AdbCompressionAlgorithm {
    type Err = AdbError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "any" => Ok(AdbCompressionAlgorithm::Any),
            "none" => Ok(AdbCompressionAlgorithm::None),
            "brotli" => Ok(AdbCompressionAlgorithm::Brotli),
            "lz4" => Ok(AdbCompressionAlgorithm::Lz4),
            "zstd" => Ok(AdbCompressionAlgorithm::Zstd),
            _ => Err(AdbError::Parse(ParseError::with_description(
                s,
                "AdbCompressionAlgorithm",
                "Unknown compression algorithm",
            ))),
        }
    }
}

/// `push [--sync] [-z ALGORITHM] [-Z] LOCAL... REMOTE`: Copy local files/directories to device.
/// - `--sync`: Only push files that are newer on the host than the device.
/// - `-n`: Dry run, push files to device without storing to the filesystem.
/// - `-z`: enable compression with a specified algorithm (any/none/brotli/lz4/zstd).
/// - `-Z`: Disable compression.
pub struct AdbPush<'a, S1, S2, I>
where
    S1: AsRef<OsStr>,
    S2: AsRef<OsStr>,
    I: IntoIterator<Item = S1>,
{
    acb: AdbCommandBuilder<'a>,
    /// `--sync`: Only push files that are newer on the host than the device.
    sync: bool,
    /// `-n`: Dry run, push files to device without storing to the filesystem.
    n: bool,
    /// - `-z ALGORITHM`: Enable compression with a specified algorithm. (if [`Some`])
    /// - `-Z`: Disable compression. (if [`None`])
    z: Option<AdbCompressionAlgorithm>,
    /// Local files/directories to copy.
    local: I,
    /// Remote destination.
    remote: S2,
}

impl<'a, S1, S2, I> AdbPush<'a, S1, S2, I>
where
    S1: AsRef<OsStr>,
    S2: AsRef<OsStr>,
    I: IntoIterator<Item = S1>,
{
    /// Creates a new `AdbPush` instance,
    /// `sync`, `n` (dry run), `z` (compression) is disabled.
    fn new(acb: AdbCommandBuilder<'a>, local: I, remote: S2) -> Self {
        Self {
            acb,
            sync: false,
            n: false,
            z: None,
            local,
            remote,
        }
    }

    /// `--sync`: Only push files that are newer on the host than the device.
    pub fn sync(mut self) -> Self {
        self.sync = true;
        self
    }

    /// `-n`: Dry run, push files to device without storing to the filesystem.
    pub fn n(mut self) -> Self {
        self.n = true;
        self
    }

    /// `-z ALGORITHM`: Enable compression with a specified algorithm.
    ///
    /// The previous compression algorithm will be overwritten.
    pub fn z(mut self, algorithm: AdbCompressionAlgorithm) -> Self {
        self.z = Some(algorithm);
        self
    }

    /// `-Z`: Disable compression.
    ///
    /// The previous compression algorithm will be overwritten.
    #[allow(non_snake_case)]
    pub fn Z(mut self) -> Self {
        self.z = None;
        self
    }

    /// `LOCAL...`: Local files/directories to copy.
    ///
    /// The previous local files/directories will be overwritten.
    pub fn local<S, I1>(self, local: I1) -> AdbPush<'a, S, S2, I1>
    where
        S: AsRef<OsStr>,
        I1: IntoIterator<Item = S>,
    {
        AdbPush {
            acb: self.acb,
            sync: self.sync,
            n: self.n,
            z: self.z,
            local,
            remote: self.remote,
        }
    }

    /// `REMOTE`: Remote destination.
    ///
    /// The previous remote destination will be overwritten.
    pub fn remote<S: AsRef<OsStr>>(self, remote: S) -> AdbPush<'a, S1, S, I> {
        AdbPush {
            acb: self.acb,
            sync: self.sync,
            n: self.n,
            z: self.z,
            local: self.local,
            remote,
        }
    }
}

impl<'a, S1, S2, I> AdbCommand for AdbPush<'a, S1, S2, I>
where
    S1: AsRef<OsStr>,
    S2: AsRef<OsStr>,
    I: IntoIterator<Item = S1>,
{
    fn build(self) -> Command {
        let mut cmd = self.acb.build();
        cmd.arg("push");
        if self.sync {
            cmd.arg("--sync");
        }
        if let Some(algorithm) = self.z {
            cmd.arg("-z").arg(algorithm);
        } else {
            cmd.arg("-Z");
        }
        cmd.args(self.local).arg(self.remote);
        cmd
    }
}

impl Adb {
    /// `push [--sync] [-z ALGORITHM] [-Z] LOCAL... REMOTE`: Copy local files/directories to device.
    /// - `--sync`: Only push files that are newer on the host than the device.
    /// - `-n`: Dry run, push files to device without storing to the filesystem.
    /// - `-z`: enable compression with a specified algorithm (any/none/brotli/lz4/zstd).
    /// - `-Z`: Disable compression.
    ///
    /// # Examples
    ///
    /// `adb push -z zstd /path/to/local /path/to/remote`
    ///
    /// ```no_run
    /// # use adbr::{Adb, AdbCommand};
    /// # use adbr::command::file_transfer::AdbCompressionAlgorithm;
    /// # let adb = Adb::new();
    /// adb.push(&["/path/to/local"], "/path/to/remote")
    ///     .z(AdbCompressionAlgorithm::Zstd)   // optional
    ///     .status()
    ///     .expect("`adb push -z zstd /path/to/local /path/to/remote` failed");
    /// ```
    pub fn push<S1, S2, I>(&self, local: I, remote: S2) -> AdbPush<S1, S2, I>
    where
        S1: AsRef<OsStr>,
        S2: AsRef<OsStr>,
        I: IntoIterator<Item = S1>,
    {
        AdbPush::new(self.command(), local, remote)
    }
}

impl<'a> AdbCommandBuilder<'a> {
    /// `push [--sync] [-z ALGORITHM] [-Z] LOCAL... REMOTE`: Copy local files/directories to device.
    ///
    /// See [`Adb::push`] for more information.
    pub fn push<S1, S2, I>(self, local: I, remote: S2) -> AdbPush<'a, S1, S2, I>
    where
        S1: AsRef<OsStr>,
        S2: AsRef<OsStr>,
        I: IntoIterator<Item = S1>,
    {
        AdbPush::new(self, local, remote)
    }
}

/// `pull [-a] [-z ALGORITHM] [-Z] REMOTE... LOCAL`: Copy files/dirs from device
/// - `-a`: preserve file timestamp and mode.
/// - `-z`: enable compression with a specified algorithm (any/none/brotli/lz4/zstd).
/// - `-Z`: disable compression.
pub struct AdbPull<'a, S1, S2, I>
where
    S1: AsRef<OsStr>,
    S2: AsRef<OsStr>,
    I: IntoIterator<Item = S1>,
{
    acb: AdbCommandBuilder<'a>,
    /// `-a`: Preserve file timestamps and permissions.
    a: bool,
    /// - `-z ALGORITHM`: Enable compression with a specified algorithm. (if [`Some`])
    /// - `-Z`: Disable compression. (if [`None`])
    z: Option<AdbCompressionAlgorithm>,
    /// Remote files/directories to copy.
    remote: I,
    /// Local destination.
    local: S2,
}

impl<'a, S1, S2, I> AdbPull<'a, S1, S2, I>
where
    S1: AsRef<OsStr>,
    S2: AsRef<OsStr>,
    I: IntoIterator<Item = S1>,
{
    /// Creates a new `AdbPull` instance,
    /// `a` (preserve file timestamp and mode), `z` (compression) is disabled.
    fn new(acb: AdbCommandBuilder<'a>, remote: I, local: S2) -> Self {
        Self {
            acb,
            a: false,
            z: None,
            remote,
            local,
        }
    }

    /// `-a`: Preserve file timestamps and permissions.
    pub fn a(mut self) -> Self {
        self.a = true;
        self
    }

    /// `-z ALGORITHM`: Enable compression with a specified algorithm.
    ///
    /// The previous compression algorithm will be overwritten.
    pub fn z(mut self, algorithm: AdbCompressionAlgorithm) -> Self {
        self.z = Some(algorithm);
        self
    }

    /// `-Z`: Disable compression.
    ///
    /// The previous compression algorithm will be overwritten.
    #[allow(non_snake_case)]
    pub fn Z(mut self) -> Self {
        self.z = None;
        self
    }

    /// `REMOTE...`: Remote files/directories to copy.
    ///
    /// The previous remote files/directories will be overwritten.
    pub fn remote<S, I1>(self, remote: I1) -> AdbPull<'a, S, S2, I1>
    where
        S: AsRef<OsStr>,
        I1: IntoIterator<Item = S>,
    {
        AdbPull {
            acb: self.acb,
            a: self.a,
            z: self.z,
            remote,
            local: self.local,
        }
    }

    /// `LOCAL`: Local destination.
    ///
    /// The previous local destination will be overwritten.
    pub fn local<S: AsRef<OsStr>>(self, local: S) -> AdbPull<'a, S1, S, I> {
        AdbPull {
            acb: self.acb,
            a: self.a,
            z: self.z,
            remote: self.remote,
            local,
        }
    }
}

impl<'a, S1, S2, I> AdbCommand for AdbPull<'a, S1, S2, I>
where
    S1: AsRef<OsStr>,
    S2: AsRef<OsStr>,
    I: IntoIterator<Item = S1>,
{
    fn build(self) -> Command {
        let mut cmd = self.acb.build();
        cmd.arg("pull");
        if self.a {
            cmd.arg("-a");
        }
        if let Some(algorithm) = self.z {
            cmd.arg("-z").arg(algorithm);
        } else {
            cmd.arg("-Z");
        }
        cmd.args(self.remote).arg(self.local);
        cmd
    }
}

impl Adb {
    /// `pull [-a] [-z ALGORITHM] [-Z] REMOTE... LOCAL`: Copy files/dirs from device
    /// - `-a`: preserve file timestamp and mode.
    /// - `-z`: enable compression with a specified algorithm (any/none/brotli/lz4/zstd).
    /// - `-Z`: disable compression.
    ///
    /// # Examples
    ///
    /// `adb pull -z zstd /path/to/remote /path/to/local`
    ///
    /// ```no_run
    /// # use adbr::{Adb, AdbCommand};
    /// # use adbr::command::file_transfer::AdbCompressionAlgorithm;
    /// # let adb = Adb::new();
    /// adb.pull("/path/to/remote", "/path/to/local")
    ///     .z(AdbCompressionAlgorithm::Zstd)   // optional
    ///     .status()
    ///     .expect("`adb pull -z zstd /path/to/remote /path/to/local` failed");
    /// ```
    pub fn pull<S1, S2, I>(&self, remote: I, local: S2) -> AdbPull<S1, S2, I>
    where
        S1: AsRef<OsStr>,
        S2: AsRef<OsStr>,
        I: IntoIterator<Item = S1>,
    {
        AdbPull::new(self.command(), remote, local)
    }
}

impl<'a> AdbCommandBuilder<'a> {
    /// `pull [-a] [-z ALGORITHM] [-Z] REMOTE... LOCAL`: Copy files/dirs from device
    ///
    /// See [`Adb::pull`] for more information.
    pub fn pull<S1, S2, I>(self, remote: I, local: S2) -> AdbPull<'a, S1, S2, I>
    where
        S1: AsRef<OsStr>,
        S2: AsRef<OsStr>,
        I: IntoIterator<Item = S1>,
    {
        AdbPull::new(self, remote, local)
    }
}

/// Sync target for `sync` command.
pub enum AdbSyncTarget {
    All,
    Data,
    Odm,
    Oem,
    Product,
    System,
    SystemExt,
    Vendor,
}

impl AsRef<OsStr> for AdbSyncTarget {
    fn as_ref(&self) -> &OsStr {
        match self {
            AdbSyncTarget::All => OsStr::new("all"),
            AdbSyncTarget::Data => OsStr::new("data"),
            AdbSyncTarget::Odm => OsStr::new("odm"),
            AdbSyncTarget::Oem => OsStr::new("oem"),
            AdbSyncTarget::Product => OsStr::new("product"),
            AdbSyncTarget::System => OsStr::new("system"),
            AdbSyncTarget::SystemExt => OsStr::new("system_ext"),
            AdbSyncTarget::Vendor => OsStr::new("vendor"),
        }
    }
}

impl Display for AdbSyncTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            AdbSyncTarget::All => "all",
            AdbSyncTarget::Data => "data",
            AdbSyncTarget::Odm => "odm",
            AdbSyncTarget::Oem => "oem",
            AdbSyncTarget::Product => "product",
            AdbSyncTarget::System => "system",
            AdbSyncTarget::SystemExt => "system_ext",
            AdbSyncTarget::Vendor => "vendor",
        })
    }
}

impl FromStr for AdbSyncTarget {
    type Err = AdbError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "all" => Ok(AdbSyncTarget::All),
            "data" => Ok(AdbSyncTarget::Data),
            "odm" => Ok(AdbSyncTarget::Odm),
            "oem" => Ok(AdbSyncTarget::Oem),
            "product" => Ok(AdbSyncTarget::Product),
            "system" => Ok(AdbSyncTarget::System),
            "system_ext" => Ok(AdbSyncTarget::SystemExt),
            "vendor" => Ok(AdbSyncTarget::Vendor),
            _ => Err(AdbError::Parse(ParseError::with_description(
                s,
                "SyncTarget",
                "Unknown sync target",
            ))),
        }
    }
}

/// `sync [-l] [-z ALGORITHM] [-Z] [all|data|odm|oem|product|system|system_ext|vendor]`:
/// Sync a local build from `$ANDROID_PRODUCT_OUT` to the device (default `all`)
/// `-n`: Dry run. Push files to device without storing to the filesystem.
/// `-l`: List files that would be copied, but don't copy them.
/// `-z`: Enable compression with a specified algorithm (any/none/brotli/lz4/zstd)
/// `-Z`: Disable compression.
pub struct AdbSync<'a> {
    acb: AdbCommandBuilder<'a>,
    /// `-n`: Dry run. Push files to device without storing to the filesystem.
    n: bool,
    /// `-l`: List files that would be copied, but don't copy them.
    l: bool,
    /// - `-z ALGORITHM`: Enable compression with a specified algorithm. (if [`Some`])
    /// - `-Z`: Disable compression. (if [`None`])
    z: Option<AdbCompressionAlgorithm>,
    /// Sync target.
    target: Option<AdbSyncTarget>,
}

impl<'a> AdbSync<'a> {
    /// Creates a new `AdbSync` instance,
    /// `n` (dry run), `l` (list files), `z` (compression) is disabled.
    fn new(acb: AdbCommandBuilder<'a>) -> Self {
        Self {
            acb,
            n: false,
            l: false,
            z: None,
            target: None,
        }
    }

    /// `-n`: Dry run. Push files to device without storing to the filesystem.
    pub fn n(mut self) -> Self {
        self.n = true;
        self
    }

    /// `-l`: List files that would be copied, but don't copy them.
    pub fn l(mut self) -> Self {
        self.l = true;
        self
    }

    /// `-z ALGORITHM`: Enable compression with a specified algorithm.
    ///
    /// The previous compression algorithm will be overwritten.
    pub fn z(mut self, algorithm: AdbCompressionAlgorithm) -> Self {
        self.z = Some(algorithm);
        self
    }

    /// `-Z`: Disable compression.
    ///
    /// The previous compression algorithm will be overwritten.
    #[allow(non_snake_case)]
    pub fn Z(mut self) -> Self {
        self.z = None;
        self
    }

    /// `TARGET`: Sync target.
    ///
    /// The previous sync target will be overwritten.
    pub fn arg(mut self, target: AdbSyncTarget) -> Self {
        self.target = Some(target);
        self
    }
}

impl<'a> AdbCommand for AdbSync<'a> {
    fn build(self) -> Command {
        let mut cmd = self.acb.build();
        cmd.arg("sync");
        if self.n {
            cmd.arg("-n");
        }
        if self.l {
            cmd.arg("-l");
        }
        if let Some(algorithm) = self.z {
            cmd.arg("-z").arg(algorithm);
        } else {
            cmd.arg("-Z");
        }
        if let Some(target) = self.target {
            cmd.arg(target);
        }
        cmd
    }
}

impl Adb {
    /// `sync [-l] [-z ALGORITHM] [-Z] [all|data|odm|oem|product|system|system_ext|vendor]`:
    /// Sync a local build from `$ANDROID_PRODUCT_OUT` to the device (default `all`)
    /// `-n`: Dry run. Push files to device without storing to the filesystem.
    /// `-l`: List files that would be copied, but don't copy them.
    /// `-z`: Enable compression with a specified algorithm (any/none/brotli/lz4/zstd)
    /// `-Z`: Disable compression.
    ///
    /// # Examples
    ///
    /// `adb sync -z zstd all`
    ///
    /// ```no_run
    /// # use adbr::{Adb, AdbCommand};
    /// # use adbr::command::file_transfer::{AdbCompressionAlgorithm, AdbSyncTarget};
    /// # let adb = Adb::new();
    /// adb.sync()
    ///     .z(AdbCompressionAlgorithm::Zstd)   // optional
    ///     .arg(AdbSyncTarget::All)            // optional
    ///     .status()
    ///     .expect("`adb sync -z zstd` failed");
    /// ```
    pub fn sync(&self) -> AdbSync {
        AdbSync::new(self.command())
    }
}

impl<'a> AdbCommandBuilder<'a> {
    /// `sync [-l] [-z ALGORITHM] [-Z] [all|data|odm|oem|product|system|system_ext|vendor]`:
    /// Sync a local build from `$ANDROID_PRODUCT_OUT` to the device (default `all`)
    ///
    /// See [`Adb::sync`] for more information.
    pub fn sync(self) -> AdbSync<'a> {
        AdbSync::new(self)
    }
}
