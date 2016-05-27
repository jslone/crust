//! Memory operations for libc
//!
//! These operations are all unsafe due to practical
//! limitations of libc. It is unlikely they will be made
//! safe in the future.

#[no_mangle]
pub unsafe extern "C" fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
    for i in 0..n {
        let a = *s1.offset(i as isize) as i32;
        let b = *s2.offset(i as isize) as i32;
        let c = a - b;
        if c != 0 {
            return c;
        }
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    for i in 0..n {
        *dest.offset(i as isize) = *src.offset(i as isize);
    }

    dest
}

#[no_mangle]
pub unsafe extern "C" fn memmove(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    if src < dest as *const u8 {
        for i in (0..n).rev() {
            *dest.offset(i as isize) = *src.offset(i as isize);
        }
    } else {
        for i in 0..n {
            *dest.offset(i as isize) = *src.offset(i as isize);
        }
    }

    dest
}

#[no_mangle]
pub unsafe extern "C" fn memset(s: *mut u8, c: i32, n: usize) -> *mut u8 {
    for i in 0..n {
        *s.offset(i as isize) = c as u8;
    }

    s
}
