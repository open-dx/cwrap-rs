use core::ffi::CStr;
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
    #[msg="c-string uninitialized"]
    Uninitialized,
    
    /// TODO
    #[msg="utf-8 error: {:}"]
    Utf8Error(Utf8Error),
}

pub unsafe fn try_unwrap_cstr<'out>(bytes: *const i8) -> Result<&'out str, CStringError> {
    if bytes.is_null() {
        return Err(CStringError::Uninitialized);
    }
    
    match CStr::from_ptr(bytes).to_str() {
        Ok(c_str) => Ok(c_str),
        Err(error) => Err(CStringError::Utf8Error(error)),
    }
}
