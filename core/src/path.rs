use std::path::Path;
use std::path::PathBuf;
use std::ffi::CString;

use crate::error::CStringError;

/// TODO
pub fn try_path_from_cstring(path_cstring: CString) -> Result<PathBuf, CStringError> {
    let path_str = crate::string::try_unwrap_cstr(path_cstring.as_ptr())?;
    Ok(PathBuf::from(path_str))
}

/// TODO
pub fn try_cstring_from_path(path: &Path) -> Result<CString, CStringError> {
    let path_str = path.as_os_str().to_str().unwrap_or_default();
    CString::new(path_str).map_err(CStringError::from)
}
