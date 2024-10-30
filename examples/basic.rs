// TODO: Remove this as soon as `oops` drops it.
use std::process::ExitCode;

use anyhow::Result;

use cwrap::drop::DropExtern;

/// TODO
pub fn main() -> Result<ExitCode> {
    let some_ext = Box::new(SomeExt {});
    
    some_ext.drop();
    
    Ok(ExitCode::SUCCESS)
}

#[repr(C)]
pub struct SomeExt {
    // TODO: Some fields that need to be dropped ..
}

impl DropExtern for SomeExt {
    fn drop(self: Box<Self>) {
        tracing::debug!("Externally dropping SomeExt");
        let _ = Box::into_raw(self);
    }
}
