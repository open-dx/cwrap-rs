use std::ffi::CStr;
use crate::error::CStringError;

/// Converts a raw pointer to a C-style string into a Rust string slice.
///
/// # Safety
///
/// This function dereferences a raw pointer and requires it to be valid:
/// - The `bytes` pointer must be non-null.
/// - The pointer must point to a valid, null-terminated C string.
/// - The data must remain valid for the duration of the returned `&str`.
///
/// # Errors
///
/// This function returns:
/// - `CStringError::Uninitialized` if the `bytes` pointer is null.
/// - `CStringError::InvalidUtf8` if the string is not valid UTF-8.
///
/// # Examples
///
/// ```rust
/// use crate::try_unwrap_cstr;
/// use std::ffi::CString;
///
/// let c_string = CString::new("Hello, world!").unwrap();
/// let c_str_ptr = c_string.as_ptr();
/// assert_eq!(try_unwrap_cstr(c_str_ptr), Ok("Hello, world!"));
/// ```
pub fn try_unwrap_cstr<'out>(bytes: *const i8) -> Result<&'out str, CStringError> {
    if bytes.is_null() {
        return Err(CStringError::Uninitialized);
    }

    unsafe {
        CStr::from_ptr(bytes).to_str().map_err(|utf8_err| {
            CStringError::Utf8Error(utf8_err)
        })
    }
}
