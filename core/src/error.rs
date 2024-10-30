use core::str::Utf8Error;

#[macro_export]
macro_rules! rec {
    () => {
        //..
    };
}

/// TODO
pub fn report_err<E: std::fmt::Display>(error: E) {
    tracing::error!("Error (ffi): {:}", error);
}

/// TODO
#[derive(oops::Error)]
pub enum CBindingError {
    /// TODO
    #[msg("c-binding error: {0:}")]
    CStringError(CStringError),
}

impl From<std::ffi::NulError> for CBindingError {
    /// TODO
    fn from(error: std::ffi::NulError) -> Self {
        CBindingError::CStringError(CStringError::NulError(error))
    }
}

/// TODO
#[derive(oops::Error)]
pub enum CStringError {
    /// TODO
    #[msg("c-string uninitialized")]
    Uninitialized,

    /// TODO
    #[msg("utf-8 error: {0:}")]
    Utf8Error(Utf8Error),

    /// TODO: See how/if we can do this without the std lib.
    #[msg("null string: {0:}")]
    NulError(std::ffi::NulError),
}

impl From<std::ffi::NulError> for CStringError {
    /// TODO
    fn from(error: std::ffi::NulError) -> Self {
        CStringError::NulError(error)
    }
}
