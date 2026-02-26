use std::ffi::OsStr;
use std::process::{Command, Stdio};

mod error;
mod types;

#[cfg(test)]
mod tests;

pub use error::*;
pub use types::*;

const INSPECT_FLAKE_REF: &str =
    "https://flakehub.com/f/DeterminateSystems/inspect/*#contents.includingOutputPaths";

/// The primary entry point to this crate.
/// It returns the parsed output of running `nix eval` on the [`inspect` flake][1] for a given flake reference.
///
/// [1]: https://github.com/DeterminateSystems/inspect
#[inline]
pub fn inspect(flake_ref: impl AsRef<OsStr>) -> Result<InspectOutput> {
    let mut command = Command::new("nix");
    let pipe = command
        .arg("eval")
        .arg("--json")
        .arg("--no-write-lock-file")
        .arg("--override-input")
        .arg("flake")
        .arg(flake_ref)
        .arg(INSPECT_FLAKE_REF)
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
