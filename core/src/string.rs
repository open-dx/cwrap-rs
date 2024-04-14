use std::ffi::CStr;

use crate::error::CStringError;


/// TODO
pub fn try_unwrap_cstr<'out>(bytes: *const i8) -> Result<&'out str, CStringError> {
    if bytes.is_null() {
        return Err(CStringError::Uninitialized);
    }
    
    unsafe {
        match CStr::from_ptr(bytes).to_str() {
            Ok(c_str) => Ok(c_str),
            Err(error) => Err(CStringError::Utf8Error(error)),
        }
    }
}
