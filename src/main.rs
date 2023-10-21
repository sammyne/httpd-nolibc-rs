#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use ministd::env::{self, Args};

mod ministd;
mod start;
mod syscalls;

#[no_mangle]
unsafe fn _start_main(rsp: *const u8) -> ! {
    let args = env::args(rsp);

    main(args);

    syscalls::exit(0);
}

#[no_mangle]
unsafe fn main(args: Args<'_>) {
    for v in args {
        let buf = v.to_bytes();
        syscalls::write(syscalls::FILENO_STDOUT, buf.as_ptr(), buf.len());
    }
}
