#![no_std]

pub extern crate libc;

mod x86;
mod arm;

// Types

#[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
pub use arm::{types::c_char, types::c_int};

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub use x86::{types::c_char, types::c_int};

// Library functions
pub fn _print(arg: &str) {
    let text = arg.as_ptr();
    unsafe { libc::printf(text as *const c_char); }
}

pub fn input(input: &str) -> i32 {
    let read_input = unsafe { libc::scanf(input.as_ptr() as *const c_char) };
    return read_input;
}

// Library macros
#[macro_export]
macro_rules! printfk {
    ($($arg:tt)*) => {$crate::_print($($arg)*)};
}
