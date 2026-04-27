use std::{
    ffi::OsStr,
    process::{Command, Stdio},
};

mod types;

use crate::error::*;
pub use types::*;

#[inline]
pub fn show(flake_ref: impl AsRef<OsStr>) -> Result<()> {
    show_with_options(flake_ref, &Default::default())
}

#[derive(Debug, Clone)]
pub struct Options {
    all_systems: bool,
}

impl Options {
    #[must_use]
    pub const fn new() -> Options {
        Self { all_systems: true }
    }

    #[must_use]
    pub const fn all_systems(mut self, all_systems: bool) -> Self {
        self.all_systems = all_systems;
        self
    }
}

impl Default for Options {
    fn default() -> Self {
        Self::new()
    }
}

pub fn show_with_options(flake_ref: impl AsRef<OsStr>, options: &Options) -> Result<()> {
    let pipe = Command::new("nix")
        .arg("flake")
        .arg("show")
        .arg(&flake_ref)
        .arg("--no-write-lock-file")
        .arg("--json")
        .args(options.all_systems.then_some("--all-systems"))
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let result = pipe.wait_with_output()?;
    if !result.status.success() {
        return Err(Error::ExitFailure {
            status: result.status,
            stderr: String::from_utf8_lossy(&result.stderr).into(),
        });
    }

    Ok(serde_json::from_slice(&result.stdout)?)
}
