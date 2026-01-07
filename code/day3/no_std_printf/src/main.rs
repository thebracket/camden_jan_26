#![no_std]
#![no_main]

use core::panic::PanicInfo;

use libc::{c_char, c_int};

unsafe extern "C" {
    fn printf(fmt: *const c_char, ...) -> c_int;
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn main(_argc: c_int, _argv: *const *const c_char) -> c_int {
    let msg = b"Hello from no_std via printf\n\0";
    unsafe {
        printf(msg.as_ptr() as *const c_char);
    }
    0
}
