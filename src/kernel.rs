#![feature(start,lang_items)]
#![crate_type = "staticlib"]
#![no_std]

mod crust;

#[start]
#[no_mangle]
pub extern fn _start() {
    kernel_main();
}

fn kernel_main() {
    loop {}
}
