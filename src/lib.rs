//! Minimal re-implementation of `std` for testing (q)emulated Cortex-M
//! processors

#![deny(missing_docs)]
#![deny(warnings)]
#![feature(asm)]
#![feature(compiler_builtins_lib)]
#![feature(core_intrinsics)]
#![feature(lang_items)]
#![no_std]

extern crate compiler_builtins;

pub mod io;
pub mod macros;
pub mod process;

mod panicking;
#[doc(hidden)]
pub mod rt;
mod syscall;

#[doc(hidden)]
#[no_mangle]
pub extern "C" fn __aeabi_unwind_cpp_pr0() {
    unreachable!()
}

#[doc(hidden)]
#[no_mangle]
pub extern "C" fn __aeabi_unwind_cpp_pr1() {
    unreachable!()
}
