//! Runtime functions for libc and rust
//!
//! These have no meaningful implementations at the moment,
//! the implementation will evolve once other libc features
//! are defined.

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[lang = "panic_fmt"]
extern "C" fn panic_fmt() {}

#[no_mangle]
pub unsafe fn __aeabi_unwind_cpp_pr0() -> () {
    loop {}
}

#[no_mangle]
pub unsafe fn __aeabi_unwind_cpp_pr1() -> () {
    loop {}
}
