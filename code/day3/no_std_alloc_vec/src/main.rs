#![no_std]
#![no_main]

extern crate alloc;

use alloc::vec::Vec;
use libc::{c_char, c_int};
use panic_halt as _;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

unsafe extern "C" {
    fn printf(fmt: *const c_char, ...) -> c_int;
}

#[unsafe(no_mangle)]
pub extern "C" fn main(_argc: c_int, _argv: *const *const c_char) -> c_int {
    let mut v: Vec<i32> = Vec::new();
    v.extend_from_slice(&[10, 20, 30, 40, 50]);

    let sum: i32 = v.iter().sum();
    let fmt = b"Vec len=%d sum=%d\n\0";
    unsafe {
        printf(fmt.as_ptr() as *const c_char, v.len() as c_int, sum as c_int);
    }

    0
}
