//! Runtime functions for libc and rust
//!
//! These have no meaningful implementations at the moment,
//! the implementation will evolve once other libc features
//! are defined.

extern "C" {
    fn halt();
}

#[no_mangle]
pub extern "C" fn abort() {
    unsafe {
        halt();
    }
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[lang = "panic_fmt"]
extern "C" fn panic_fmt() {}
