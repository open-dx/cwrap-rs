// TODO: Remove this as soon as `oops` drops it.
#![feature(error_in_core)]

use std::process::ExitCode;

/// TODO
fn main() -> Result<ExitCode, SimpleExampleError> {
    Ok(ExitCode::SUCCESS)
}

//---
/// Dependency error uses the Error derive macro to implement 
#[derive(oops::Error)]
pub enum SimpleExampleError {
    /// Something else is wrong, but not sure what.
    #[msg="a generic error occurred"]
    GenericError,
}
