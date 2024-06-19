//! App Installation commands (see also `adb shell cmd package help`)
//!
//! - `install [-lrtsdg] [--instant] PACKAGE`: Push a single package to the device and install it
//! - `install-multiple [-lrtsdpg] [--instant] PACKAGE...`: Push multiple APKs to the device for a single package and install them
//! - `install-multi-package [-lrtsdg] [--instant] PACKAGE...`: Push one or more packages to the device and install them atomically
//! - `uninstall [-k] APPLICATION_ID`: Remove this APPLICATION_ID from the device.
//!
//! See [App Installation Commands](https://android.googlesource.com/platform/packages/modules/adb/+/refs/heads/master/docs/user/adb.1.md#app-installation)

use std::ffi::{OsStr, OsString};
use std::process::Command;

use crate::command::AdbCommandBuilder;
use crate::{Adb, AdbCommand};

/// `install [-lrtsdg] [--instant] PACKAGE`: Push a single package to the device and install it
/// - `-r`: Replace existing application.
/// - `-t`: Allow test packages.
/// - `-d`: Allow version code downgrade (debuggable packages only).
/// - `-g`: Grant all runtime permissions.
/// - `--abi ABI`: Override platform's default ABI.
/// - `--instant`: Cause the app to be installed as an ephemeral install app.
/// - `--no-streaming`: Always push APK to device and invoke Package Manager as separate steps.
/// - `--streaming`: Force streaming APK directly into Package Manager.
/// - `--fastdeploy`: Use fast deploy.
/// - `-no-fastdeploy`: Prevent use of fast deploy.
/// - `-force-agent`: Force update of deployment agent when using fast deploy.
/// - `-date-check-agent`: Update deployment agent when local version is newer and using fast deploy.
/// - `--version-check-agent`: Update deployment agent when local version has different version code and using fast deploy.
/// - `--local-agent`: Locate agent files from local source build (instead of SDK location).
///
/// See also `adb shell pm help` for more options.
#[derive(Debug, Clone)]
pub struct AdbInstall<'a, S1: AsRef<OsStr>, S2: AsRef<OsStr>> {
    acb: AdbCommandBuilder<'a>,
    l: bool,
    /// `-r`: Replace existing application.
    r: bool,
    /// `-t`: Allow test packages.
    t: bool,
    s: bool,
    /// `-d`: Allow version code downgrade (debuggable packages only).
    d: bool,
    /// `-g`: Grant all runtime permissions.
    g: bool,
    /// `--abi ABI`: Override platform's default ABI.
    abi: Option<S1>,
    /// `--instant`: Cause the app to be installed as an ephemeral install app.
    instant: bool,
    /// `--no-streaming`: Always push APK to device and invoke Package Manager as separate steps.
    no_streaming: bool,
    /// `--streaming`: Force streaming APK directly into Package Manager.
    streaming: bool,
    /// `--fastdeploy`: Use fast deploy.
    fastdeploy: bool,
    /// `-no-fastdeploy`: Prevent use of fast deploy.
    no_fastdeploy: bool,
    /// `-force-agent`: Force update of deployment agent when using fast deploy.
    force_agent: bool,
    /// `-date-check-agent`: Update deployment agent when local version is newer and using fast deploy.
    date_check_agent: bool,
    /// `--version-check-agent`: Update deployment agent when local version has different version code and using fast deploy.
    version_check_agent: bool,
    /// `--local-agent`: Locate agent files from local source build (instead of SDK location).
    local_agent: bool,
    /// `PACKAGE`: The package to install.
    package: S2,
}

impl<'a, S: AsRef<OsStr>> AdbInstall<'a, S, S> {
    fn new(acb: AdbCommandBuilder<'a>, package: S) -> AdbInstall<'a, S, S> {
        AdbInstall {
            acb,
            l: false,
            r: false,
            t: false,
            s: false,
            d: false,
            g: false,
            abi: None,
            instant: false,
            no_streaming: false,
            streaming: false,
            fastdeploy: false,
            no_fastdeploy: false,
            force_agent: false,
            date_check_agent: false,
            version_check_agent: false,
            local_agent: false,
            package,
        }
    }
}

impl<'a, S1, S2> AdbInstall<'a, S1, S2>
where
    S1: AsRef<OsStr>,
    S2: AsRef<OsStr>,
{
    pub fn l(mut self) -> Self {
        self.l = true;
        self
    }

    /// `-r`: Replace existing application.
    pub fn r(mut self) -> Self {
        self.r = true;
        self
    }

    /// `-t`: Allow test packages.
    pub fn t(mut self) -> Self {
        self.t = true;
        self
    }

    pub fn s(mut self) -> Self {
        self.s = true;
        self
    }

    /// `-d`: Allow version code downgrade (debuggable packages only).
    pub fn d(mut self) -> Self {
        self.d = true;
        self
    }

    /// `-g`: Grant all runtime permissions.
    pub fn g(mut self) -> Self {
        self.g = true;
        self
    }

    /// `--abi ABI`: Override platform's default ABI.
    ///
    /// The previous ABI will be overwritten.
    pub fn abi<S: AsRef<OsStr>>(self, abi: S) -> AdbInstall<'a, S, S2> {
        AdbInstall {
            acb: self.acb,
            l: self.l,
            r: self.r,
            t: self.t,
            s: self.s,
            d: self.d,
            g: self.g,
            abi: Some(abi),
            instant: self.instant,
            no_streaming: self.no_streaming,
            streaming: self.streaming,
            fastdeploy: self.fastdeploy,
            no_fastdeploy: self.no_fastdeploy,
            force_agent: self.force_agent,
            date_check_agent: self.date_check_agent,
            version_check_agent: self.version_check_agent,
            local_agent: self.local_agent,
            package: self.package,
        }
    }

    /// `--instant`: Cause the app to be installed as an ephemeral install app.
    pub fn instant(mut self) -> Self {
        self.instant = true;
        self
    }

    /// `--no-streaming`: Always push APK to device and invoke Package Manager as separate steps.
    pub fn no_streaming(mut self) -> Self {
        self.no_streaming = true;
        self
    }

    /// `--streaming`: Force streaming APK directly into Package Manager.
    pub fn streaming(mut self) -> Self {
        self.streaming = true;
        self
    }

    /// `--fastdeploy`: Use fast deploy.
    pub fn fastdeploy(mut self) -> Self {
        self.fastdeploy = true;
        self
    }

    /// `-no-fastdeploy`: Prevent use of fast deploy.
    pub fn no_fastdeploy(mut self) -> Self {
        self.no_fastdeploy = true;
        self
    }

    /// `-force-agent`: Force update of deployment agent when using fast deploy.
    pub fn force_agent(mut self) -> Self {
        self.force_agent = true;
        self
    }

    /// `-date-check-agent`: Update deployment agent when local version is newer and using fast deploy.
    pub fn date_check_agent(mut self) -> Self {
        self.date_check_agent = true;
        self
    }

    /// `--version-check-agent`: Update deployment agent when local version has different version code and using fast deploy.
    pub fn version_check_agent(mut self) -> Self {
        self.version_check_agent = true;
        self
    }

    /// `--local-agent`: Locate agent files from local source build (instead of SDK location).
    pub fn local_agent(mut self) -> Self {
        self.local_agent = true;
        self
    }

    /// `PACKAGE`: The package to install.
    ///
    /// The previous package will be overwritten.
    pub fn package<S: AsRef<OsStr>>(self, package: S) -> AdbInstall<'a, S1, S> {
        AdbInstall {
            acb: self.acb,
            l: self.l,
            r: self.r,
            t: self.t,
            s: self.s,
            d: self.d,
            g: self.g,
            abi: self.abi,
            instant: self.instant,
            no_streaming: self.no_streaming,
            streaming: self.streaming,
            fastdeploy: self.fastdeploy,
            no_fastdeploy: self.no_fastdeploy,
            force_agent: self.force_agent,
            date_check_agent: self.date_check_agent,
            version_check_agent: self.version_check_agent,
            local_agent: self.local_agent,
            package,
        }
    }
}

impl<'a, S1, S2> AdbCommand for AdbInstall<'a, S1, S2>
where
    S1: AsRef<OsStr>,
    S2: AsRef<OsStr>,
{
    fn build(self) -> Command {
        let mut cmd = self.acb.build();
        cmd.arg("install");
        if self.l {
            cmd.arg("-l");
        }
        if self.r {
            cmd.arg("-r");
        }
        if self.t {
            cmd.arg("-t");
        }
        if self.s {
            cmd.arg("-s");
        }
        if self.d {
            cmd.arg("-d");
        }
        if self.g {
            cmd.arg("-g");
        }
        if let Some(abi) = self.abi {
            cmd.arg("--abi").arg(abi);
        }
        if self.instant {
            cmd.arg("--instant");
        }
        if self.no_streaming {
            cmd.arg("--no-streaming");
        }
        if self.streaming {
            cmd.arg("--streaming");
        }
        if self.fastdeploy {
            cmd.arg("--fastdeploy");
        }
        if self.no_fastdeploy {
            cmd.arg("-no-fastdeploy");
        }
        if self.force_agent {
            cmd.arg("-force-agent");
        }
        if self.date_check_agent {
            cmd.arg("-date-check-agent");
        }
        if self.version_check_agent {
            cmd.arg("--version-check-agent");
        }
        if self.local_agent {
            cmd.arg("--local-agent");
        }
        cmd.arg(self.package);
        cmd
    }
}

impl Adb {
    /// `install [-lrtsdg] [--instant] PACKAGE`: Push a single package to the device and install it
    /// - `-r`: Replace existing application.
    /// - `-t`: Allow test packages.
    /// - `-d`: Allow version code downgrade (debuggable packages only).
    /// - `-g`: Grant all runtime permissions.
    /// - `--abi ABI`: Override platform's default ABI.
    /// - `--instant`: Cause the app to be installed as an ephemeral install app.
    /// - `--no-streaming`: Always push APK to device and invoke Package Manager as separate steps.
    /// - `--streaming`: Force streaming APK directly into Package Manager.
    /// - `--fastdeploy`: Use fast deploy.
    /// - `-no-fastdeploy`: Prevent use of fast deploy.
    /// - `-force-agent`: Force update of deployment agent when using fast deploy.
    /// - `-date-check-agent`: Update deployment agent when local version is newer and using fast deploy.
    /// - `--version-check-agent`: Update deployment agent when local version has different version code and using fast deploy.
    /// - `--local-agent`: Locate agent files from local source build (instead of SDK location).
    ///
    /// See also `adb shell pm help` for more options.
    ///
    /// # Examples
    ///
    /// `adb install -r /path/to/app.apk`
    ///
    /// ```no_run
    /// # use adbr::{Adb, AdbCommand};
    /// # let adb = Adb::new();
    /// adb.install("/path/to/app.apk")
    ///     .r()
    ///     .status()
    ///     .expect("`adb install -r /path/to/app.apk` failed");
    /// ```
    pub fn install<S: AsRef<OsStr>>(&self, package: S) -> AdbInstall<S, S> {
        AdbInstall::new(self.command(), package)
    }
}

impl<'a> AdbCommandBuilder<'a> {
    /// `install [-lrtsdg] [--instant] PACKAGE`: Push a single package to the device and install it
    /// - `-r`: Replace existing application.
    /// - `-t`: Allow test packages.
    /// - `-d`: Allow version code downgrade (debuggable packages only).
    /// - `-g`: Grant all runtime permissions.
    /// - `--abi ABI`: Override platform's default ABI.
    /// - `--instant`: Cause the app to be installed as an ephemeral install app.
    /// - `--no-streaming`: Always push APK to device and invoke Package Manager as separate steps.
    /// - `--streaming`: Force streaming APK directly into Package Manager.
    /// - `--fastdeploy`: Use fast deploy.
    /// - `-no-fastdeploy`: Prevent use of fast deploy.
    /// - `-force-agent`: Force update of deployment agent when using fast deploy.
    /// - `-date-check-agent`: Update deployment agent when local version is newer and using fast deploy.
    /// - `--version-check-agent`: Update deployment agent when local version has different version code and using fast deploy.
    /// - `--local-agent`: Locate agent files from local source build (instead of SDK location).
    ///
    /// See also `adb shell pm help` for more options.
    ///
    /// See [`Adb::install`] for more information.
    pub fn install<S: AsRef<OsStr>>(self, package: S) -> AdbInstall<'a, S, S> {
        AdbInstall::new(self, package)
    }
}

/// `install-multiple [-lrtsdpg] [--instant] PACKAGE...`: Push multiple APKs to the device for a single package and install them
/// - `-r`: Replace existing application.
/// - `-t`: Allow test packages.
/// - `-d`: Allow version code downgrade (debuggable packages only).
/// - `-p`: Partial application install.
/// - `-g`: Grant all runtime permissions.
/// - `--abi ABI`: Override platform's default ABI.
/// - `--instant`: Cause the app to be installed as an ephemeral install app.
/// - `--no-streaming`: Always push APK to device and invoke Package Manager as separate steps.
/// - `--streaming`: Force streaming APK directly into Package Manager.
/// - `--fastdeploy`: Use fast deploy.
/// - `-no-fastdeploy`: Prevent use of fast deploy.
/// - `-force-agent`: Force update of deployment agent when using fast deploy.
/// - `-date-check-agent`: Update deployment agent when local version is newer and using fast deploy.
/// - `--version-check-agent`: Update deployment agent when local version has different version code and using fast deploy.
/// - `--local-agent`: Locate agent files from local source build (instead of SDK location).
///
/// See also `adb shell pm help` for more options.
#[derive(Debug, Clone)]
pub struct AdbInstallMultiple<'a, S: AsRef<OsStr>> {
    acb: AdbCommandBuilder<'a>,
    l: bool,
    /// `-r`: Replace existing application.
    r: bool,
    /// `-t`: Allow test packages.
    t: bool,
    s: bool,
    /// `-d`: Allow version code downgrade (debuggable packages only).
    d: bool,
    /// `-p`: Partial application install.
    p: bool,
    /// `-g`: Grant all runtime permissions.
    g: bool,
    /// `--abi ABI`: Override platform's default ABI.
    abi: Option<S>,
    /// `--instant`: Cause the app to be installed as an ephemeral install app.
    instant: bool,
    /// `--no-streaming`: Always push APK to device and invoke Package Manager as separate steps.
    no_streaming: bool,
    /// `--streaming`: Force streaming APK directly into Package Manager.
    streaming: bool,
    /// `--fastdeploy`: Use fast deploy.
    fastdeploy: bool,
    /// `-no-fastdeploy`: Prevent use of fast deploy.
    no_fastdeploy: bool,
    /// `-force-agent`: Force update of deployment agent when using fast deploy.
    force_agent: bool,
    /// `-date-check-agent`: Update deployment agent when local version is newer and using fast deploy.
    date_check_agent: bool,
    /// `--version-check-agent`: Update deployment agent when local version has different version code and using fast deploy.
    version_check_agent: bool,
    /// `--local-agent`: Locate agent files from local source build (instead of SDK location).
    local_agent: bool,
    /// `PACKAGE...`: The packages to install.
    packages: Vec<OsString>,
}

impl<'a, S: AsRef<OsStr>> AdbInstallMultiple<'a, S> {
    fn new(acb: AdbCommandBuilder<'a>, packages: Vec<OsString>) -> AdbInstallMultiple<'a, S> {
        AdbInstallMultiple {
            acb,
            l: false,
            r: false,
            t: false,
            s: false,
            d: false,
            p: false,
            g: false,
            abi: None,
            instant: false,
            no_streaming: false,
            streaming: false,
            fastdeploy: false,
            no_fastdeploy: false,
            force_agent: false,
            date_check_agent: false,
            version_check_agent: false,
            local_agent: false,
            packages,
        }
    }

    pub fn l(mut self) -> Self {
        self.l = true;
        self
    }

    /// `-r`: Replace existing application.
    pub fn r(mut self) -> Self {
        self.r = true;
        self
    }

    /// `-t`: Allow test packages.
    pub fn t(mut self) -> Self {
        self.t = true;
        self
    }

    pub fn s(mut self) -> Self {
        self.s = true;
        self
    }

    /// `-d`: Allow version code downgrade (debuggable packages only).
    pub fn d(mut self) -> Self {
        self.d = true;
        self
    }

    /// `-g`: Grant all runtime permissions.
    pub fn g(mut self) -> Self {
        self.g = true;
        self
    }

    /// `--abi ABI`: Override platform's default ABI.
    ///
    /// The previous ABI will be overwritten.
    pub fn abi<S1: AsRef<OsStr>>(self, abi: S1) -> AdbInstallMultiple<'a, S1> {
        AdbInstallMultiple {
            acb: self.acb,
            l: self.l,
            r: self.r,
            t: self.t,
            s: self.s,
            d: self.d,
            p: self.p,
            g: self.g,
            abi: Some(abi),
            instant: self.instant,
            no_streaming: self.no_streaming,
            streaming: self.streaming,
            fastdeploy: self.fastdeploy,
            no_fastdeploy: self.no_fastdeploy,
            force_agent: self.force_agent,
            date_check_agent: self.date_check_agent,
            version_check_agent: self.version_check_agent,
            local_agent: self.local_agent,
            packages: self.packages,
        }
    }

    /// `--instant`: Cause the app to be installed as an ephemeral install app.
    pub fn instant(mut self) -> Self {
        self.instant = true;
        self
    }

    /// `--no-streaming`: Always push APK to device and invoke Package Manager as separate steps.
    pub fn no_streaming(mut self) -> Self {
        self.no_streaming = true;
        self
    }

    /// `--streaming`: Force streaming APK directly into Package Manager.
    pub fn streaming(mut self) -> Self {
        self.streaming = true;
        self
    }

    /// `--fastdeploy`: Use fast deploy.
    pub fn fastdeploy(mut self) -> Self {
        self.fastdeploy = true;
        self
    }

    /// `-no-fastdeploy`: Prevent use of fast deploy.
    pub fn no_fastdeploy(mut self) -> Self {
        self.no_fastdeploy = true;
        self
    }

    /// `-force-agent`: Force update of deployment agent when using fast deploy.
    pub fn force_agent(mut self) -> Self {
        self.force_agent = true;
        self
    }

    /// `-date-check-agent`: Update deployment agent when local version is newer and using fast deploy.
    pub fn date_check_agent(mut self) -> Self {
        self.date_check_agent = true;
        self
    }

    /// `--version-check-agent`: Update deployment agent when local version has different version code and using fast deploy.
    pub fn version_check_agent(mut self) -> Self {
        self.version_check_agent = true;
        self
    }

    /// `--local-agent`: Locate agent files from local source build (instead of SDK location).
    pub fn local_agent(mut self) -> Self {
        self.local_agent = true;
        self
    }

    /// `PACKAGE...`: The packages to install.
    ///
    /// The previous packages will be overwritten.
    pub fn packages<S1, I>(mut self, packages: I) -> Self
    where
        S1: AsRef<OsStr>,
        I: IntoIterator<Item = S1>,
    {
        self.packages = packages
            .into_iter()
            .map(|s| s.as_ref().to_os_string())
            .collect();
        self
    }
}

impl<'a, S> AdbCommand for AdbInstallMultiple<'a, S>
where
    S: AsRef<OsStr>,
{
    fn build(self) -> Command {
        let mut cmd = self.acb.build();
        cmd.arg("install-multiple");
        if self.l {
            cmd.arg("-l");
        }
        if self.r {
            cmd.arg("-r");
        }
        if self.t {
            cmd.arg("-t");
        }
        if self.s {
            cmd.arg("-s");
        }
        if self.d {
            cmd.arg("-d");
        }
        if self.p {
            cmd.arg("-p");
        }
        if self.g {
            cmd.arg("-g");
        }
        if let Some(abi) = self.abi {
            cmd.arg("--abi").arg(abi);
        }
        if self.instant {
            cmd.arg("--instant");
        }
        if self.no_streaming {
            cmd.arg("--no-streaming");
        }
        if self.streaming {
            cmd.arg("--streaming");
        }
        if self.fastdeploy {
            cmd.arg("--fastdeploy");
        }
        if self.no_fastdeploy {
            cmd.arg("-no-fastdeploy");
        }
        if self.force_agent {
            cmd.arg("-force-agent");
        }
        if self.date_check_agent {
            cmd.arg("-date-check-agent");
        }
        if self.version_check_agent {
            cmd.arg("--version-check-agent");
        }
        if self.local_agent {
            cmd.arg("--local-agent");
        }
        cmd.args(self.packages);
        cmd
    }
}

impl Adb {
    /// `install-multiple [-lrtsdpg] [--instant] PACKAGE...`: Push multiple APKs to the device for a single package and install them
    /// - `-r`: Replace existing application.
    /// - `-t`: Allow test packages.
    /// - `-d`: Allow version code downgrade (debuggable packages only).
    /// - `-p`: Partial application install.
    /// - `-g`: Grant all runtime permissions.
    /// - `--abi ABI`: Override platform's default ABI.
    /// - `--instant`: Cause the app to be installed as an ephemeral install app.
    /// - `--no-streaming`: Always push APK to device and invoke Package Manager as separate steps.
    /// - `--streaming`: Force streaming APK directly into Package Manager.
    /// - `--fastdeploy`: Use fast deploy.
    /// - `-no-fastdeploy`: Prevent use of fast deploy.
    /// - `-force-agent`: Force update of deployment agent when using fast deploy.
    /// - `-date-check-agent`: Update deployment agent when local version is newer and using fast deploy.
    /// - `--version-check-agent`: Update deployment agent when local version has different version code and using fast deploy.
    /// - `--local-agent`: Locate agent files from local source build (instead of SDK location).
    ///
    /// See also `adb shell pm help` for more options.
    ///
    /// # Examples
    ///
    /// `adb install-multiple -r /path/to/app1.apk /path/to/app2.apk`
    ///
    /// ```no_run
    /// # use adbr::{Adb, AdbCommand};
    /// # let adb = Adb::new();
    /// adb.install_multiple(&["/path/to/app1.apk", "/path/to/app2.apk"])
    ///     .r()
    ///     .status()
    ///     .expect("`adb install-multiple -r /path/to/app1.apk /path/to/app2.apk` failed");
    /// ```
    pub fn install_multiple<S, I>(&self, packages: I) -> AdbInstallMultiple<S>
    where
        S: AsRef<OsStr>,
        I: IntoIterator<Item = S>,
    {
        AdbInstallMultiple::new(
            self.command(),
            packages
                .into_iter()
                .map(|s| s.as_ref().to_os_string())
                .collect(),
        )
    }
}

impl<'a> AdbCommandBuilder<'a> {
    /// `install-multiple [-lrtsdpg] [--instant] PACKAGE...`: Push multiple APKs to the device for a single package and install them
    /// - `-r`: Replace existing application.
    /// - `-t`: Allow test packages.
    /// - `-d`: Allow version code downgrade (debuggable packages only).
    /// - `-p`: Partial application install.
    /// - `-g`: Grant all runtime permissions.
    /// - `--abi ABI`: Override platform's default ABI.
    /// - `--instant`: Cause the app to be installed as an ephemeral install app.
    /// - `--no-streaming`: Always push APK to device and invoke Package Manager as separate steps.
    /// - `--streaming`: Force streaming APK directly into Package Manager.
    /// - `--fastdeploy`: Use fast deploy.
    /// - `-no-fastdeploy`: Prevent use of fast deploy.
    /// - `-force-agent`: Force update of deployment agent when using fast deploy.
    /// - `-date-check-agent`: Update deployment agent when local version is newer and using fast deploy.
    /// - `--version-check-agent`: Update deployment agent when local version has different version code and using fast deploy.
    /// - `--local-agent`: Locate agent files from local source build (instead of SDK location).
    ///
    /// See also `adb shell pm help` for more options.
    ///
    /// See [`Adb::install_multiple`] for more information.
    pub fn install_multiple<S, I>(self, packages: I) -> AdbInstallMultiple<'a, S>
    where
        S: AsRef<OsStr>,
        I: IntoIterator<Item = S>,
    {
        AdbInstallMultiple::new(
            self,
            packages
                .into_iter()
                .map(|s| s.as_ref().to_os_string())
                .collect(),
        )
    }
}

/// `install-multi-package [-lrtsdg] [--instant] PACKAGE...`: Push one or more packages to the device and install them atomically
/// - `-r`: Replace existing application.
/// - `-t`: Allow test packages.
/// - `-d`: Allow version code downgrade (debuggable packages only).
/// - `-g`: Grant all runtime permissions.
/// - `--abi ABI`: Override platform's default ABI.
/// - `--instant`: Cause the app to be installed as an ephemeral install app.
/// - `--no-streaming`: Always push APK to device and invoke Package Manager as separate steps.
/// - `--streaming`: Force streaming APK directly into Package Manager.
/// - `--fastdeploy`: Use fast deploy.
/// - `-no-fastdeploy`: Prevent use of fast deploy.
/// - `-force-agent`: Force update of deployment agent when using fast deploy.
/// - `-date-check-agent`: Update deployment agent when local version is newer and using fast deploy.
/// - `--version-check-agent`: Update deployment agent when local version has different version code and using fast deploy.
/// - `--local-agent`: Locate agent files from local source build (instead of SDK location).
///
/// See also `adb shell pm help` for more options.
#[derive(Debug, Clone)]
pub struct AdbInstallMultiPackage<'a, S: AsRef<OsStr>> {
    acb: AdbCommandBuilder<'a>,
    l: bool,
    /// `-r`: Replace existing application.
    r: bool,
    /// `-t`: Allow test packages.
    t: bool,
    s: bool,
    /// `-d`: Allow version code downgrade (debuggable packages only).
    d: bool,
    /// `-g`: Grant all runtime permissions.
    g: bool,
    /// `--abi ABI`: Override platform's default ABI.
    abi: Option<S>,
    /// `--instant`: Cause the app to be installed as an ephemeral install app.
    instant: bool,
    /// `--no-streaming`: Always push APK to device and invoke Package Manager as separate steps.
    no_streaming: bool,
    /// `--streaming`: Force streaming APK directly into Package Manager.
    streaming: bool,
    /// `--fastdeploy`: Use fast deploy.
    fastdeploy: bool,
    /// `-no-fastdeploy`: Prevent use of fast deploy.
    no_fastdeploy: bool,
    /// `-force-agent`: Force update of deployment agent when using fast deploy.
    force_agent: bool,
    /// `-date-check-agent`: Update deployment agent when local version is newer and using fast deploy.
    date_check_agent: bool,
    /// `--version-check-agent`: Update deployment agent when local version has different version code and using fast deploy.
    version_check_agent: bool,
    /// `--local-agent`: Locate agent files from local source build (instead of SDK location).
    local_agent: bool,
    /// `PACKAGE...`: The packages to install.
    packages: Vec<OsString>,
}

impl<'a, S: AsRef<OsStr>> AdbInstallMultiPackage<'a, S> {
    fn new(acb: AdbCommandBuilder<'a>, packages: Vec<OsString>) -> AdbInstallMultiPackage<'a, S> {
        AdbInstallMultiPackage {
            acb,
            l: false,
            r: false,
            t: false,
            s: false,
            d: false,
            g: false,
            abi: None,
            instant: false,
            no_streaming: false,
            streaming: false,
            fastdeploy: false,
            no_fastdeploy: false,
            force_agent: false,
            date_check_agent: false,
            version_check_agent: false,
            local_agent: false,
            packages,
        }
    }

    pub fn l(mut self) -> Self {
        self.l = true;
        self
    }

    /// `-r`: Replace existing application.
    pub fn r(mut self) -> Self {
        self.r = true;
        self
    }

    /// `-t`: Allow test packages.
    pub fn t(mut self) -> Self {
        self.t = true;
        self
    }

    pub fn s(mut self) -> Self {
        self.s = true;
        self
    }

    /// `-d`: Allow version code downgrade (debuggable packages only).
    pub fn d(mut self) -> Self {
        self.d = true;
        self
    }

    /// `-g`: Grant all runtime permissions.
    pub fn g(mut self) -> Self {
        self.g = true;
        self
    }

    /// `--abi ABI`: Override platform's default ABI.
    ///
    /// The previous ABI will be overwritten.
    pub fn abi<S1: AsRef<OsStr>>(self, abi: S1) -> AdbInstallMultiPackage<'a, S1> {
        AdbInstallMultiPackage {
            acb: self.acb,
            l: self.l,
            r: self.r,
            t: self.t,
            s: self.s,
            d: self.d,
            g: self.g,
            abi: Some(abi),
            instant: self.instant,
            no_streaming: self.no_streaming,
            streaming: self.streaming,
            fastdeploy: self.fastdeploy,
            no_fastdeploy: self.no_fastdeploy,
            force_agent: self.force_agent,
            date_check_agent: self.date_check_agent,
            version_check_agent: self.version_check_agent,
            local_agent: self.local_agent,
            packages: self.packages,
        }
    }

    /// `--instant`: Cause the app to be installed as an ephemeral install app.
    pub fn instant(mut self) -> Self {
        self.instant = true;
        self
    }

    /// `--no-streaming`: Always push APK to device and invoke Package Manager as separate steps.
    pub fn no_streaming(mut self) -> Self {
        self.no_streaming = true;
        self
    }

    /// `--streaming`: Force streaming APK directly into Package Manager.
    pub fn streaming(mut self) -> Self {
        self.streaming = true;
        self
    }

    /// `--fastdeploy`: Use fast deploy.
    pub fn fastdeploy(mut self) -> Self {
        self.fastdeploy = true;
        self
    }

    /// `-no-fastdeploy`: Prevent use of fast deploy.
    pub fn no_fastdeploy(mut self) -> Self {
        self.no_fastdeploy = true;
        self
    }

    /// `-force-agent`: Force update of deployment agent when using fast deploy.
    pub fn force_agent(mut self) -> Self {
        self.force_agent = true;
        self
    }

    /// `-date-check-agent`: Update deployment agent when local version is newer and using fast deploy.
    pub fn date_check_agent(mut self) -> Self {
        self.date_check_agent = true;
        self
    }

    /// `--version-check-agent`: Update deployment agent when local version has different version code and using fast deploy.
    pub fn version_check_agent(mut self) -> Self {
        self.version_check_agent = true;
        self
    }

    /// `--local-agent`: Locate agent files from local source build (instead of SDK location).
    pub fn local_agent(mut self) -> Self {
        self.local_agent = true;
        self
    }

    /// `PACKAGE...`: The packages to install.
    ///
    /// The previous packages will be overwritten.
    pub fn packages<S1, I>(mut self, packages: I) -> Self
    where
        S1: AsRef<OsStr>,
        I: IntoIterator<Item = S1>,
    {
        self.packages = packages
            .into_iter()
            .map(|s| s.as_ref().to_os_string())
            .collect();
        self
    }
}

impl<'a, S> AdbCommand for AdbInstallMultiPackage<'a, S>
where
    S: AsRef<OsStr>,
{
    fn build(self) -> Command {
        let mut cmd = self.acb.build();
        cmd.arg("install-multi-package");
        if self.l {
            cmd.arg("-l");
        }
        if self.r {
            cmd.arg("-r");
        }
        if self.t {
            cmd.arg("-t");
        }
        if self.s {
            cmd.arg("-s");
        }
        if self.d {
            cmd.arg("-d");
        }
        if self.g {
            cmd.arg("-g");
        }
        if let Some(abi) = self.abi {
            cmd.arg("--abi").arg(abi);
        }
        if self.instant {
            cmd.arg("--instant");
        }
        if self.no_streaming {
            cmd.arg("--no-streaming");
        }
        if self.streaming {
            cmd.arg("--streaming");
        }
        if self.fastdeploy {
            cmd.arg("--fastdeploy");
        }
        if self.no_fastdeploy {
            cmd.arg("-no-fastdeploy");
        }
        if self.force_agent {
            cmd.arg("-force-agent");
        }
        if self.date_check_agent {
            cmd.arg("-date-check-agent");
        }
        if self.version_check_agent {
            cmd.arg("--version-check-agent");
        }
        if self.local_agent {
            cmd.arg("--local-agent");
        }
        cmd.args(self.packages);
        cmd
    }
}

impl Adb {
    /// `install-multi-package [-lrtsdg] [--instant] PACKAGE...`: Push one or more packages to the device and install them atomically
    /// - `-r`: Replace existing application.
    /// - `-t`: Allow test packages.
    /// - `-d`: Allow version code downgrade (debuggable packages only).
    /// - `-g`: Grant all runtime permissions.
    /// - `--abi ABI`: Override platform's default ABI.
    /// - `--instant`: Cause the app to be installed as an ephemeral install app.
    /// - `--no-streaming`: Always push APK to device and invoke Package Manager as separate steps.
    /// - `--streaming`: Force streaming APK directly into Package Manager.
    /// - `--fastdeploy`: Use fast deploy.
    /// - `-no-fastdeploy`: Prevent use of fast deploy.
    /// - `-force-agent`: Force update of deployment agent when using fast deploy.
    /// - `-date-check-agent`: Update deployment agent when local version is newer and using fast deploy.
    /// - `--version-check-agent`: Update deployment agent when local version has different version code and using fast deploy.
    /// - `--local-agent`: Locate agent files from local source build (instead of SDK location).
    ///
    /// See also `adb shell pm help` for more options.
    ///
    /// # Examples
    ///
    /// `adb install-multi-package -r /path/to/app1.apk /path/to/app2.apk`
    ///
    /// ```no_run
    /// # use adbr::{Adb, AdbCommand};
    /// # let adb = Adb::new();
    /// adb.install_multi_package(&["/path/to/app1.apk", "/path/to/app2.apk"])
    ///     .r()
    ///     .status()
    ///     .expect("`adb install-multi-package -r /path/to/app1.apk /path/to/app2.apk` failed");
    /// ```
    pub fn install_multi_package<S, I>(&self, packages: I) -> AdbInstallMultiPackage<S>
    where
        S: AsRef<OsStr>,
        I: IntoIterator<Item = S>,
    {
        AdbInstallMultiPackage::new(
            self.command(),
            packages
                .into_iter()
                .map(|s| s.as_ref().to_os_string())
                .collect(),
        )
    }
}

impl<'a> AdbCommandBuilder<'a> {
    /// `install-multi-package [-lrtsdg] [--instant] PACKAGE...`: Push one or more packages to the device and install them atomically
    /// - `-r`: Replace existing application.
    /// - `-t`: Allow test packages.
    /// - `-d`: Allow version code downgrade (debuggable packages only).
    /// - `-g`: Grant all runtime permissions.
    /// - `--abi ABI`: Override platform's default ABI.
    /// - `--instant`: Cause the app to be installed as an ephemeral install app.
    /// - `--no-streaming`: Always push APK to device and invoke Package Manager as separate steps.
    /// - `--streaming`: Force streaming APK directly into Package Manager.
    /// - `--fastdeploy`: Use fast deploy.
    /// - `-no-fastdeploy`: Prevent use of fast deploy.
    /// - `-force-agent`: Force update of deployment agent when using fast deploy.
    /// - `-date-check-agent`: Update deployment agent when local version is newer and using fast deploy.
    /// - `--version-check-agent`: Update deployment agent when local version has different version code and using fast deploy.
    /// - `--local-agent`: Locate agent files from local source build (instead of SDK location).
    ///
    /// See also `adb shell pm help` for more options.
    ///
    /// See [`Adb::install_multi_package`] for more information.
    pub fn install_multi_package<S, I>(self, packages: I) -> AdbInstallMultiPackage<'a, S>
    where
        S: AsRef<OsStr>,
        I: IntoIterator<Item = S>,
    {
        AdbInstallMultiPackage::new(
            self,
            packages
                .into_iter()
                .map(|s| s.as_ref().to_os_string())
                .collect(),
        )
    }
}

/// `uninstall [-k] APPLICATION_ID`: Remove this `APPLICATION_ID` from the device.
///
/// - `-k`: Keep the data and cache directories.
#[derive(Debug, Clone)]
pub struct AdbUninstall<'a, S: AsRef<OsStr>> {
    acb: AdbCommandBuilder<'a>,
    /// `-k`: Keep the data and cache directories.
    k: bool,
    /// `APPLICATION_ID`: The package to uninstall.
    application_id: S,
}

impl<'a, S: AsRef<OsStr>> AdbUninstall<'a, S> {
    fn new(acb: AdbCommandBuilder<'a>, application_id: S) -> AdbUninstall<'a, S> {
        AdbUninstall {
            acb,
            k: false,
            application_id,
        }
    }

    /// `-k`: Keep the data and cache directories.
    pub fn k(mut self) -> Self {
        self.k = true;
        self
    }

    /// `APPLICATION_ID`: The package to uninstall.
    ///
    /// The previous application ID will be overwritten.
    pub fn application_id<S1: AsRef<OsStr>>(self, application_id: S1) -> AdbUninstall<'a, S1> {
        AdbUninstall {
            acb: self.acb,
            k: self.k,
            application_id,
        }
    }
}

impl<'a, S: AsRef<OsStr>> AdbCommand for AdbUninstall<'a, S> {
    fn build(self) -> Command {
        let mut cmd = self.acb.build();
        cmd.arg("uninstall");
        if self.k {
            cmd.arg("-k");
        }
        cmd.arg(self.application_id);
        cmd
    }
}

impl Adb {
    /// `uninstall [-k] APPLICATION_ID`: Remove this `APPLICATION_ID` from the device.
    ///
    /// - `-k`: Keep the data and cache directories.
    ///
    /// # Examples
    ///
    /// `adb uninstall com.example.app`
    ///
    /// ```no_run
    /// # use adbr::{Adb, AdbCommand};
    /// # let adb = Adb::new();
    /// adb.uninstall("com.example.app")
    ///     .status()
    ///     .expect("`adb uninstall com.example.app` failed");
    /// ```
    pub fn uninstall<S: AsRef<OsStr>>(&self, application_id: S) -> AdbUninstall<S> {
        AdbUninstall::new(self.command(), application_id)
    }
}

impl<'a> AdbCommandBuilder<'a> {
    /// `uninstall [-k] APPLICATION_ID`: Remove this `APPLICATION_ID` from the device.
    ///
    /// - `-k`: Keep the data and cache directories.
    ///
    /// See [`Adb::uninstall`] for more information.
    pub fn uninstall<S: AsRef<OsStr>>(self, application_id: S) -> AdbUninstall<'a, S> {
        AdbUninstall::new(self, application_id)
    }
}
