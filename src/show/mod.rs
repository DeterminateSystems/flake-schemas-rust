use std::ffi::OsStr;

mod types;

pub use types::*;
use crate::error::*;

#[inline]
pub fn show(flake_ref: impl AsRef<OsStr>) -> Result<()> {
    show_with_options(flake_ref, &Default::default())
}

#[derive(Debug, Clone)]
pub struct Options {
    with_output: bool,
}

impl Options {
    pub fn new() -> Options {
        Self { with_output: true }
    }

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

pub fn show_with_options(flake_ref: impl AsRef<OsStr>, options: &Options) -> Result<()> {
    let _ = flake_ref;
    let _ = options;

    todo!();
}
