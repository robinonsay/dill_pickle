use std::{ffi::CStr, os::macos::raw, ptr::{null, null_mut}, str::Utf8Error};
use super::{errors::DillError, loader::{Loader, Symbol}};
use std::os::raw::{c_char, c_void};

pub struct PosixLoader {
    handle: *mut c_void
}

#[link(name = "dl")]
extern "C" {
    fn dlopen(filename: *const c_char, flag: i32) -> *mut c_void;
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
    fn dlclose(handle: *mut c_void) -> i32;
    fn dlerror() -> *const c_char;
}

const LOCAL:i32 =     0x00000;
const LAZY:i32 =      0x00001;
const NOW:i32 =       0x00002;
const NO_LOAD:i32 =   0x00004;
const DEEP_BIND:i32 = 0x00008;
const GLOBAL:i32 =    0x00100;
const NO_DELETE:i32 = 0x01000;

impl Loader for PosixLoader {
    fn open(&mut self, lib: &str) -> Result<(), super::errors::DillError> {
        unsafe {
            // Convert the &str to a CString
            let symbol_name = CStr::from_ptr(lib.as_ptr() as *const i8);
            self.handle = dlopen(symbol_name.as_ptr(), NO_DELETE);
            // Handle error in loading handle
            if self.handle.is_null() {
                // Get raw error message pointer
                let raw_err_msg = dlerror();
                // Check if error message is null
                if raw_err_msg.is_null() {
                    // Error message is null
                    return Err(DillError::NullErr(DillError::create_msg("Null error message")));
                }
                // Parse error messaage
                let err_msg = match CStr::from_ptr(raw_err_msg).to_str() {
                    Ok(msg) => msg,
                    // Could not parse
                    Err(_) => "Could not parse error message: UTF-8 Error"
                };
                // Return error message
                return Err(DillError::NullErr(DillError::create_msg(err_msg)));
            }
        }
        Ok(())
    }

    fn load<F>(&self, symbol: Symbol) -> Result<F, super::errors::DillError> {
        todo!()
    }

    fn close(&mut self) -> Result<(), super::errors::DillError> {
        self.handle = null_mut();
        Ok(())
    }
}
