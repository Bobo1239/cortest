#![allow(private_no_mangle_fns)]

use process;

#[no_mangle]
pub extern "C" fn _start() {
    extern "Rust" {
        fn main() -> i32;
    }

    process::exit(unsafe { main() })
}
