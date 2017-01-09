//! Processes

use syscall;

/// Exit this process
pub fn exit(code: i32) -> ! {
    unsafe { syscall::exit(code) }
}
