#![no_main]
#![no_std]

#[macro_use]
extern crate cortest;

#[no_mangle]
pub fn main() -> i32 {
    println!("Hello, world!");
    0
}
