use std::ffi::OsStr;
use std::process::{Command, Stdio};

mod types;

pub use types::*;
use crate::error::*;

const INSPECT_INCLUDING_OUTPUTS: &str =
    "https://flakehub.com/f/DeterminateSystems/inspect/*#contents.includingOutputPaths";
const INSPECT_EXCLUDING_OUTPUTS: &str =
    "https://flakehub.com/f/DeterminateSystems/inspect/*#contents.excludingOutputPaths";

/// The primary entry point to this crate.
/// It returns the parsed output of running `nix eval` on the [`inspect` flake][1] for a given flake reference.
/// If output paths are not required, see [`InspectOptions`] and [`inspect_with_options`].
///
/// [1]: https://github.com/DeterminateSystems/inspect
#[inline]
pub fn inspect(flake_ref: impl AsRef<OsStr>) -> Result<Output> {
    inspect_with_options(flake_ref, &Default::default())
}

/// Options controlling the `inspect` flake's behavior.
/// By default, output paths are requested.
#[derive(Debug, Clone)]
pub struct Options {
    with_output: bool,
}

impl Options {
    pub fn new() -> Options {
        Self { with_output: true }
    }

    /// Determine whether or not this should include output paths in the inventory.
    pub fn with_output(mut self, with_output: bool) -> Self {
        self.with_output = with_output;
        self
    }
}

impl Default for Options {
    fn default() -> Self {
        Self::new()
    }
}

/// Alternative entry point for inventorying a flake.
/// If customization options are not needed, prefer using [`inspect`].
pub fn inspect_with_options(
    flake_ref: impl AsRef<OsStr>,
    options: &Options,
) -> Result<Output> {
    let inspect_flake_ref = if options.with_output {
        INSPECT_INCLUDING_OUTPUTS
    } else {
        INSPECT_EXCLUDING_OUTPUTS
    };

    let mut command = Command::new("nix");
    let pipe = command
        .arg("eval")
        .arg("--json")
        .arg("--no-write-lock-file")
        .arg("--override-input")
        .arg("flake")
        .arg(flake_ref)
        .arg(inspect_flake_ref)
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

    let output = serde_json::from_slice(&result.stdout)?;
    Ok(output)
}
