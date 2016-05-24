#![feature(start,lang_items)]
#![no_std]
#![no_main]

#[start]
#[no_mangle]
pub extern fn main() {
    loop {}
}

#[lang = "eh_personality"]
extern fn eh_personality() {}

#[lang = "panic_fmt"]
extern fn panic_fmt() {}

