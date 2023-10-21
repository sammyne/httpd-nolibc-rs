#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use ministd::env::{self, Args};

mod c;
mod ministd;
mod start;
mod syscalls;

//#[no_mangle]
unsafe fn main(args: Args<'_>) {
    for v in args {
        syscalls::write(syscalls::FILENO_STDOUT, v.as_ptr(), v.len());
    }
}
