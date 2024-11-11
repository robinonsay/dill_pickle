use core::ffi::{CStr, c_char};
use dill_pickle::{loader::Loader, posix::PosixLoader};

type MyFunc = extern "C" fn() -> *const c_char;

fn main () {
    let res = PosixLoader::new("./libhello.so");
    match res {
        Ok(my_loader) => {
            let hello_world: MyFunc = my_loader.load("HelloWorld").unwrap();
            unsafe {
                let msg = CStr::from_ptr(hello_world()).to_str().unwrap_or("invalid");
                println!("My message: {}", msg);
            }
        },
        Err(e) => {
            println!("Error: {}", e)
        }
    }
}
