use core::intrinsics;

pub unsafe fn exit(code: i32) -> ! {
    asm!("swi 0"
         :: "{r7}"(1), "{r0}"(code)
         :: "volatile");
    intrinsics::unreachable()
}

pub unsafe fn write(fd: u32, buffer: *const u8, size: usize) -> i32 {
    let r;
    asm!("swi 0"
         : "={r0}"(r)
         : "{r7}"(4), "{r0}"(fd), "{r1}"(buffer), "{r2}"(size)
         :: "volatile");
    r
}
