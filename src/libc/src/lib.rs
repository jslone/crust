//! Rust implementation of libc for crust kernel
//!
//! The implementation will be safe where practical based
//! on the interface, though C level performance is the
//! primary concern given it will primarily be ingested
//! by unsafe code being cross compiled.

#![feature(lang_items)]
#![no_std]
#![no_builtins]

pub mod runtime;
pub mod mem;
