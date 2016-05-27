#![feature(asm)]
#![crate_type = "staticlib"]
#![no_std]

extern crate libc;

mod crust;

pub use crust::*;

const GPIO_BASE: u32 = 0x20200000;

fn sleep(value: u32) {
    for _ in 1..value {
        unsafe {
            asm!("");
        }
    }
}

#[no_mangle]
pub extern "C" fn kernel_main(_: isize, _: *const *const u8) -> isize {
    let gpio = GPIO_BASE as *const u32;
    let led_on = unsafe { gpio.offset(10) as *mut u32 };
    let led_off = unsafe { gpio.offset(7) as *mut u32 };

    loop {
        unsafe {
            *(led_on) = 1 << 16;
        }
        sleep(500000);
        unsafe {
            *(led_off) = 1 << 16;
        }
        sleep(500000);
    }
}
