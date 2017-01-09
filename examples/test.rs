#![no_main]
#![no_std]

#[macro_use]
extern crate cortest;

test! {
    fn equal() -> bool {
        42 == 42
    }

    fn different() -> bool {
        42 != 24
    }

    fn failed() -> bool {
        false
    }
}
