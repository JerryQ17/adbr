//! The module for adb commands and command builders.

pub mod app_installation;
pub mod debugging;
pub mod features;
pub mod file_transfer;
pub mod general;
pub mod global_option;
pub mod internal_debugging;
pub mod networking;
pub mod scripting;
pub mod security;
pub mod shell;
pub mod usb;

use std::collections::HashSet;
use std::process::{Child, Command, ExitStatus, Output};

use crate::{Adb, AdbResult};
use global_option::AdbGlobalOption;

pub use file_transfer::{AdbCompressionAlgorithm, AdbSyncTarget};
pub use scripting::{AdbRebootTarget, AdbWaitForState, AdbWaitForTransport};

/// A trait that builds and executes adb commands.
pub trait AdbCommand: Sized {
    /// Builds the adb command with working directory, global options and command-specific options.
    ///
    /// It is recommended to use [`Self::spawn`], [`Self::output`] or [`Self::status`] shortcuts
    /// instead of this method, unless you need to modify the command further.
    fn build(self) -> Command;

    /// Executes the command as a child process, returning a handle to it.
    ///
    /// By default, stdin, stdout and stderr are inherited from the parent.
    ///
    /// Shortcut for [`Command::spawn`].
    fn spawn(self) -> AdbResult<Child> {
        self.build().spawn().map_err(Into::into)
    }

    /// Executes the command as a child process,
    /// waiting for it to finish and collecting all of its output.
    ///
    /// By default, stdout and stderr are captured (and used to provide the resulting output).
    /// Stdin is not inherited from the parent and any attempt by the child process
    /// to read from the stdin stream will result in the stream immediately closing.
    ///
    /// Shortcut for [`Command::output`].
    fn output(self) -> AdbResult<Output> {
        self.build().output().map_err(Into::into)
    }

    /// Executes a command as a child process, waiting for it to finish and collecting its status.
    ///
    /// By default, stdin, stdout and stderr are inherited from the parent.
    ///
    /// Shortcut for [`Command::status`].
    fn status(self) -> AdbResult<ExitStatus> {
        self.build().status().map_err(Into::into)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
/// A builder for adb commands.
///
/// It contains the environment and global options of a command,
/// but command-specific options are not provided.
pub struct AdbCommandBuilder<'a> {
    /// The environment used in this command.
    adb: &'a Adb,
    /// The global options of the command.
    global_options: HashSet<AdbGlobalOption>,
}

impl<'a> AdbCommandBuilder<'a> {
    /// Creates an [`AdbCommandBuilder`] with empty global options.
    pub(crate) fn new(adb: &'a Adb) -> Self {
        Self {
            adb,
            global_options: HashSet::new(),
        }
    }

    /// Removes all existing global options that match the given predicate (`matches`),
    /// then adds the given global option (`opt`).
    pub(crate) fn add_global_option<F>(mut self, opt: AdbGlobalOption, mut matches: F) -> Self
    where
        F: FnMut(&AdbGlobalOption) -> bool,
    {
        self.global_options.retain(|opt| !matches(opt));
        self.add_global_option_unchecked(opt)
    }

    /// Adds the given global option (`opt`) without checking for duplicates.
    ///
    /// # Safety
    ///
    /// Duplicated global options will not affect memory safety,
    /// but may lead to unsuccessful adb command execution or wrong result.
    pub(crate) fn add_global_option_unchecked(mut self, opt: AdbGlobalOption) -> Self {
        self.global_options.insert(opt);
        self
    }

    /// Builds the adb command with working directory, environment variables and global options.
    fn build(self) -> Command {
        let mut cmd = Command::new("adb");
        if let Some(working_directory) = &self.adb.working_directory {
            cmd.current_dir(working_directory);
        }
        self.adb.envs.apply(&mut cmd);
        cmd.args(self.global_options.iter().map(|opt| opt.to_string()));
        cmd
    }
}
