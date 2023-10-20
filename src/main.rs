#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::arch::asm;
use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/* functions: rdi, rsi, rdx, rcx, r8, r9 */
/*  syscalls: rdi, rsi, rdx, r10, r8, r9 */
/*                           ^^^         */
/* stack grows from a high address to a low address */
pub unsafe fn exit(code: i32) -> ! {
    const SYSCALL_NUMBER: u64 = 60;
    asm!(
        "syscall",
        in("rax") SYSCALL_NUMBER,
        in("rdi") code,
        options(noreturn)
    )
}

#[no_mangle] // don't mangle the name of this function
pub unsafe fn _start() -> ! {
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default
    exit(0);
}

#[no_mangle]
pub fn not_main() {}
