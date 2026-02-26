use std::process::ExitStatus;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error(transparent)]
    JSON(#[from] serde_json::Error),

    #[error("`nix eval` did not succeed: exit {status}")]
    ExitFailure { status: ExitStatus, stderr: String },
}
