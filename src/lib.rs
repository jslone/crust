#![feature(asm,core_intrinsics)]
#![crate_type = "staticlib"]
#![no_std]

extern crate libc;

mod crust;
pub use crust::*;

#[inline(never)]
fn test_blink(x: u32) {
    for _ in 0..x {
        crust::gpio::led_on();
        crust::sys::sleep(50000);

        crust::gpio::led_off();
        crust::sys::sleep(50000);
    }
}

#[no_mangle]
pub extern "C" fn kernel_main(_: isize, _: *const *const u8) -> isize {

    loop {
        for i in 1..4 {
            test_blink(i);
            crust::sys::sleep(500000);
        }
    }
}
