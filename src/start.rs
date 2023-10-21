use core::arch::asm;
use core::panic::PanicInfo;

use crate::{env, ministd};

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle] // don't mangle the name of this function
unsafe extern "C" fn _start() -> ! {
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default
    // option 'RUSTFLAGS="-C force-frame-pointers=yes"' helps storing rsp+8 in rbp.
    // 另一种方式是 nake_function，具体参考
    //    https://fasterthanli.me/series/making-our-own-executable-packer/part-12
    asm!(
        "mov rdi, rbp",
        "sub rdi, -8",
        "call _start_main",
        options(noreturn)
    )
    //asm!("mov rdi, rsp", "call main", options(noreturn))

    // todo: figure out why following is broken
    /*
    let rsp = {
        let v:u64;
        asm!("mov {}, rbp", out(reg) v);
        v as *const u8
    };

    let _args = env::args(rsp);

    //main(args);

    syscalls::exit(0);
    */
}

#[no_mangle]
unsafe fn _start_main(rsp: *const u8) -> ! {
    let args = env::args(rsp);

    let _ = crate::main(args);

    ministd::process::exit(0);
}
