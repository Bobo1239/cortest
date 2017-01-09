# `cortest`

> Minimal re-implementation of `std` for testing (q)emulated Cortex-M processors

This library gives you access to some of `std` functionality, like standard
I/O, on the following targets

- `thumbv6m-none-eabi`
- `thumbv7em-none-eabi`
- `thumbv7em-none-eabihf`
- `thumbv7m-none-eabi`

Programs linked to this library are meant to be executed using QEMU (in
*user* emulation mode) and won't work when flashed on a microcontroller.

## Examples

### Hello, world!

Here's the classic "Hello, world!" program

``` rust
#![no_main]
#![no_std]

#[macro_use]
extern crate cortest;

#[no_mangle]
pub fn main() -> i32 {
    println!("Hello, world!");
    0
}
```

Note that `main` returns an `i32` value. That's the exit code of the program.

```
$ xargo rustc --target thumbv7m-none-eabi -- -C link-arg=-nostartfiles

$ qemu-arm -cpu cortex-m3 target/thumbv7m-none-eabi/debug/examples/hello
Hello, world!
```

### Panicking

`panic!` works and gives you information about where the `panic!` originated.

``` rust
#![no_main]
#![no_std]

extern crate cortest;

#[no_mangle]
pub fn main() -> i32 {
    panic!()
}
```

```
$ qemu-arm -cpu cortex-m3 target/thumbv7m-none-eabi/debug/examples/panic
panicked at 'explicit panic', examples/panic.rs:8
```

### Testing

This crate provides a `test!` macro that you can use to define unit tests.

``` rust
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
```

Each unit test must return a `bool` value that indicates whether the test
succeeded (`true`) or failed (`false`).

```
$ qemu-arm -cpu cortex-m3 target/thumbv7m-none-eabi/debug/examples/test
running 3 tests
test equal ... ok
test different ... ok
test failed ... FAILED

test result: FAILED. 2 passed; 1 failed

$ echo $?
101
```

Note that there is no unwinding support in this crate so an failed `assert!`
will "take down" the test runner and the rest of tests won't run.

## Known limitations

- QEMU user emulation mode doesn't emulate "hardware" so programs linked to this
  library can't use Cortex-M peripherals like `SysTick`.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
