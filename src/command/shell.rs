//! Shell commands
//!
//! - `shell [-e ESCAPE] [-n] [-Tt] [-x] [COMMAND...]`:
//!     Run remote shell command (interactive shell if no command given).
//! - `emu COMMAND`: Run emulator console `COMMAND`.
//!
//! See [Shell Commands](https://android.googlesource.com/platform/packages/modules/adb/+/refs/heads/master/docs/user/adb.1.md#shell).

use std::ffi::{OsStr, OsString};
use std::process::Command;

use crate::command::AdbCommandBuilder;
use crate::{Adb, AdbCommand};

/// Whether to allocate a pty.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum AdbPtyAllocation {
    /// `-T`: Disable pty allocation.
    Disable,
    /// `-t`: Allocate a pty if on a tty.
    Enable,
    /// `-tt`: Force pty allocation.
    Force,
}

impl AsRef<OsStr> for AdbPtyAllocation {
    fn as_ref(&self) -> &OsStr {
        match self {
            AdbPtyAllocation::Disable => OsStr::new("-T"),
            AdbPtyAllocation::Enable => OsStr::new("-t"),
            AdbPtyAllocation::Force => OsStr::new("-tt"),
        }
    }
}

/// `shell [-e ESCAPE] [-n] [-Tt] [-x] [COMMAND...]`:
/// Run remote shell command (interactive shell if no command given).
/// - `-e`: Choose escape character, or “none”; default ‘~’.
/// - `-n`: Don't read from stdin.
/// - `-T`: Disable pty allocation.
/// - `-t`: Allocate a pty if on a tty (-tt: force pty allocation).
/// - `-x`: Disable remote exit codes and stdout/stderr separation.
#[derive(Debug, Clone)]
pub struct AdbShell<'a> {
    acb: AdbCommandBuilder<'a>,
    /// `-e`: Choose escape character, or “none”; default ‘~’.
    e: Option<char>,
    /// `-n`: Don't read from stdin.
    n: bool,
    /// `-T`: Disable pty allocation.
    /// `-t`: Allocate a pty if on a tty.
    /// `-tt`: force pty allocation.
    t: Option<AdbPtyAllocation>,
    /// `-x`: Disable remote exit codes and stdout/stderr separation.
    x: bool,
    /// `COMMAND...`: The command to run.
    command: Vec<OsString>,
}

impl<'a> AdbShell<'a> {
    /// Creates a new `AdbShell` command,
    /// `-e` (escape character), `-n` (don't read from stdin), `-Tt` (pty allocation),
    /// `-x` (disable remote exit codes and stdout/stderr separation) are false by default.
    fn new(acb: AdbCommandBuilder<'a>) -> Self {
        Self {
            acb,
            e: None,
            n: false,
            t: None,
            x: false,
            command: Vec::new(),
        }
    }

    /// `-e`: Choose escape character, or `none`; default `~`.
    pub fn e(mut self, e: char) -> Self {
        self.e = Some(e);
        self
    }

    /// `-n`: Don't read from stdin.
    pub fn n(mut self) -> Self {
        self.n = true;
        self
    }

    /// `-T`: Disable pty allocation.
    #[allow(non_snake_case)]
    pub fn T(mut self) -> Self {
        self.t = Some(AdbPtyAllocation::Disable);
        self
    }

    /// `-t`: Allocate a pty if on a tty.
    pub fn t(mut self) -> Self {
        self.t = Some(AdbPtyAllocation::Enable);
        self
    }

    /// `-tt`: Force pty allocation.
    pub fn tt(mut self) -> Self {
        self.t = Some(AdbPtyAllocation::Force);
        self
    }

    /// `-x`: Disable remote exit codes and stdout/stderr separation.
    pub fn x(mut self) -> Self {
        self.x = true;
        self
    }

    /// `COMMAND...`: The command to run.
    pub fn arg<T: AsRef<OsStr>>(mut self, arg: T) -> Self {
        self.command.push(arg.as_ref().to_os_string());
        self
    }

    /// `COMMAND...`: The command to run.
    pub fn args<T, I>(mut self, args: I) -> Self
    where
        T: AsRef<OsStr>,
        I: IntoIterator<Item = T>,
    {
        self.command
            .extend(args.into_iter().map(|arg| arg.as_ref().to_os_string()));
        self
    }
}

impl<'a> AdbCommand for AdbShell<'a> {
    fn build(self) -> Command {
        let mut cmd = self.acb.build();
        cmd.arg("shell");
        if let Some(e) = self.e {
            cmd.arg("-e").arg(e.to_string());
        }
        if self.n {
            cmd.arg("-n");
        }
        if let Some(t) = self.t {
            cmd.arg(t);
        }
        if self.x {
            cmd.arg("-x");
        }
        cmd.args(self.command);
        cmd
    }
}

impl Adb {
    /// `shell [-e ESCAPE] [-n] [-Tt] [-x] [COMMAND...]`:
    /// Run remote shell command (interactive shell if no command given).
    /// - `-e`: Choose escape character, or “none”; default ‘~’.
    /// - `-n`: Don't read from stdin.
    /// - `-T`: Disable pty allocation.
    /// - `-t`: Allocate a pty if on a tty (-tt: force pty allocation).
    /// - `-x`: Disable remote exit codes and stdout/stderr separation.
    ///
    /// # Examples
    ///
    /// `adb shell ls -l`
    ///
    /// ```no_run
    /// # use adbr::{Adb, AdbCommand};
    /// # let adb = Adb::new();
    /// adb.shell()
    ///     .arg("ls")
    ///     .arg("-l")
    ///     .status()
    ///     .expect("`adb shell ls -l` failed");
    /// ```
    pub fn shell(&self) -> AdbShell {
        AdbShell::new(self.command())
    }
}

impl<'a> AdbCommandBuilder<'a> {
    /// `shell [-e ESCAPE] [-n] [-Tt] [-x] [COMMAND...]`:
    /// Run remote shell command (interactive shell if no command given).
    /// - `-e`: Choose escape character, or “none”; default ‘~’.
    /// - `-n`: Don't read from stdin.
    /// - `-T`: Disable pty allocation.
    /// - `-t`: Allocate a pty if on a tty (-tt: force pty allocation).
    /// - `-x`: Disable remote exit codes and stdout/stderr separation.
    ///
    /// See [`Adb::shell`] for more information.
    pub fn shell(self) -> AdbShell<'a> {
        AdbShell::new(self)
    }
}

/// `emu COMMAND`: Run emulator console `COMMAND`.
#[derive(Debug, Clone)]
pub struct AdbEmu<'a, S: AsRef<OsStr>> {
    acb: AdbCommandBuilder<'a>,
    /// `COMMAND`: The command to run.
    command: S,
}

impl<'a, S: AsRef<OsStr>> AdbEmu<'a, S> {
    /// Creates a new `AdbEmu` command.
    fn new(acb: AdbCommandBuilder<'a>, command: S) -> Self {
        Self { acb, command }
    }
}

impl<'a, S: AsRef<OsStr>> AdbCommand for AdbEmu<'a, S> {
    fn build(self) -> Command {
        let mut cmd = self.acb.build();
        cmd.arg("emu").arg(self.command);
        cmd
    }
}

impl Adb {
    /// `emu COMMAND`: Run emulator console `COMMAND`.
    ///
    /// # Examples
    ///
    /// `adb emu kill`
    ///
    /// ```no_run
    /// # use adbr::{Adb, AdbCommand};
    /// # let adb = Adb::new();
    /// adb.emu("kill").status().expect("`adb emu kill` failed");
    /// ```
    pub fn emu<S: AsRef<OsStr>>(&self, command: S) -> AdbEmu<S> {
        AdbEmu::new(self.command(), command)
    }
}

impl<'a> AdbCommandBuilder<'a> {
    /// `emu COMMAND`: Run emulator console `COMMAND`.
    ///
    /// See [`Adb::emu`] for more information.
    pub fn emu<S: AsRef<OsStr>>(self, command: S) -> AdbEmu<'a, S> {
        AdbEmu::new(self, command)
    }
}
