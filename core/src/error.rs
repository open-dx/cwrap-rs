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
pub enum CStringError {
    /// TODO
    #[msg = "c-string uninitialized"]
    Uninitialized,

    /// TODO
    #[msg = "utf-8 error: {:}"]
    Utf8Error(Utf8Error),

    /// TODO
    #[msg = "null string"]
    NulError,
}
