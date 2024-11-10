use core::ffi::{c_char, c_void};
use std::ffi::{CStr, CString};
use std::mem::transmute_copy;
use super::{errors::DillError, loader::Loader};

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

// const RTLD_LOCAL:i32 =     0x00000;
// const RTLD_LAZY:i32 =      0x00001;
const RTLD_NOW:i32 =       0x00002;
// const RTLD_NO_LOAD:i32 =   0x00004;
// const RTLD_DEEP_BIND:i32 = 0x00008;
// const RTLD_GLOBAL:i32 =    0x00100;
const RTLD_NO_DELETE:i32 = 0x01000;

impl PosixLoader {
    pub fn new(lib: &str) -> Result<Self, super::errors::DillError> {
        unsafe {
            // Convert the &str to a CString
            let lib_name = CString::new(lib)
                .map_err(|_| DillError::NullErr(DillError::create_msg("Null error")))?;
            let handle = dlopen(lib_name.as_ptr(), RTLD_NOW & RTLD_NO_DELETE);
            // Handle error in loading handle
            if handle.is_null() {
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
            Ok(PosixLoader{handle})
        }
    }
    
}

impl Drop for PosixLoader {
    fn drop(&mut self){
        unsafe {
            let res = dlclose(self.handle);
            if res != 0 {
                // Get raw error message pointer
                let raw_err_msg = dlerror();
                // Check if error message is null
                if raw_err_msg.is_null() {
                    // Error message is null
                    return;
                }
                // Parse error messaage
                let err_msg = CStr::from_ptr(raw_err_msg).to_str()
                .unwrap_or("Could not parse error message: UTF-8 Error");
                // Return error message
                println!("DL Close Err: {}", err_msg);
            }
        }
    } 
}

impl Loader for PosixLoader {

    fn load<F>(&self, sym_name: &str) -> Result<F, super::errors::DillError> {
        unsafe {
            let symbol_name = CString::new(sym_name)
                .map_err(|_| DillError::NullErr(DillError::create_msg("Null error")))?;
            let raw_symbol = dlsym(self.handle, symbol_name.as_ptr() as *const i8);
            // Handle error in loading handle
            if raw_symbol.is_null() {
                // Get raw error message pointer
                let raw_err_msg = dlerror();
                // Check if error message is null
                if raw_err_msg.is_null() {
                    // Error message is null
                    return Err(DillError::NullErr(DillError::create_msg("Null error message")));
                }
                // Parse error messaage
                let err_msg = CStr::from_ptr(raw_err_msg).to_str()
                .unwrap_or("Could not parse error message: UTF-8 Error");
                // Return error message
                return Err(DillError::NullErr(DillError::create_msg(err_msg)));
            }
            else {
                let symbol = transmute_copy(&raw_symbol);
                Ok(symbol)
            }
        }
    }

}
