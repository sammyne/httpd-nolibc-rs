#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle] // don't mangle the name of this function
pub unsafe fn _start() -> ! {
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default
    syscalls::exit(0);
}

#[no_mangle]
pub fn not_main() {}

mod syscalls;
