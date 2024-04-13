// TODO: Remove this as soon as `oops` drops it.
#![feature(error_in_core)]

use std::process::ExitCode;

/// TODO
pub fn main() -> Result<ExitCode, SimpleExampleError> {
    Ok(ExitCode::SUCCESS)
}

//---
/// TODO
#[derive(oops::Error)]
pub enum SimpleExampleError {
    /// Something else is wrong, but not sure what.
    #[msg="an unknown error occurred"]
    GenericError,
}
