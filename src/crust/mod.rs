extern crate core;

pub const KERNEL : &'static str = "crust-kernel-0.0.1";

#[lang = "eh_personality"]
extern fn eh_personality() {}

#[lang = "panic_fmt"]
extern fn panic_fmt() {}

