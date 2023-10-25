//! 此模块实现项目依赖的系统调用接口。
//! linux 系统下的函数调用参数传递顺序约定为：rdi, rsi, rdx, rcx, r8, r9。例如，f(a, b)，则 a、b 依序放入 rdi 和 rsi 寄存器。
//! linux 系统下的系统调用参数传递顺序约定为：rdi, rsi, rdx, r10, r8, r9。例如，syscall(a, b)，则 a、b 依序放入 rdi 和 rsi
//! 寄存器。
//!
use core::mem;
use core::{arch::asm, ffi::c_char};

use crate::c::SockaddrIn;

pub const FILENO_STDOUT: u32 = 1;

/* functions: rdi, rsi, rdx, rcx, r8, r9 */
/*  syscalls: rdi, rsi, rdx, r10, r8, r9 */
/*                           ^^^         */
/* stack grows from a high address to a low address */

pub unsafe fn accept(sockfd: i32, addr: *const SockaddrIn, addr_len: i32) -> i32 {
    let out: i32;
    asm!(
        "syscall",
        in("rax") 43,
        in("rdi") sockfd,
        in("rsi") addr,
        in("rdx") addr_len,
        lateout("rax") out,
        // Linux syscalls don't touch the stack at all, so
        // we don't care about its alignment
        options(nostack)
    );
    out
}

pub unsafe fn bind(sockfd: i32, addr: &SockaddrIn) -> i32 {
    const ADDR_LEN: usize = mem::size_of::<SockaddrIn>();

    let out: i32;
    asm!(
        "syscall",
        in("rax") 49,
        in("rdi") sockfd,
        in("rsi") addr,
        in("rdx") ADDR_LEN,
        lateout("rax") out,
        // Linux syscalls don't touch the stack at all, so
        // we don't care about its alignment
        options(nostack)
    );
    out
}

pub unsafe fn close(fd: i32) -> i32 {
    let out: i32;
    asm!(
        "syscall",
        in("rax") 3,
        in("rdi") fd,
        lateout("rax") out,
        // Linux syscalls don't touch the stack at all, so
        // we don't care about its alignment
        options(nostack)
    );
    out
}

pub unsafe fn exit(code: i32) -> ! {
    const SYSCALL_NUMBER: u64 = 60;
    asm!(
        "syscall",
        in("rax") SYSCALL_NUMBER,
        in("rdi") code,
        options(noreturn)
    )
}

pub unsafe fn fork() -> i32 {
    let out: i32;
    asm!(
        "syscall",
        inout("rax") 57 => out,
        // Linux syscalls don't touch the stack at all, so
        // we don't care about its alignment
        options(nostack)
    );
    out
}

pub unsafe fn listen(socket: i32, backlog: i32) -> i32 {
    let out: i32;
    asm!(
        "syscall",
        inout("rax") 50 => out,
        in("rdi") socket,
        in("rsi") backlog,
        // Linux syscalls don't touch the stack at all, so
        // we don't care about its alignment
        options(nostack)
    );
    out
}

pub unsafe fn open(path: *const c_char, flags: i32) -> i32 {
    let out: i32;
    asm!(
        "syscall",
        in("rax") 2,
        in("rdi") path,
        in("rsi") flags,
        lateout("rax") out,
        // Linux syscalls don't touch the stack at all, so
        // we don't care about its alignment
        options(nostack)
    );
    out
}

pub unsafe fn read(fd: i32, buf: *const u8, n: usize) -> isize {
    let out: isize;
    asm!(
        "syscall",
        inout("rax") 0isize => out,
        in("rdi") fd,
        in("rsi") buf,
        in("rdx") n,
        // Linux syscalls don't touch the stack at all, so
        // we don't care about its alignment
        options(nostack)
    );
    out
}

pub unsafe fn setsockopt(socket: i32, level: i32, option_name: i32, option_value: &[u8]) -> i32 {
    let out: i32;
    asm!(
        "syscall",
        inout("rax") 54 => out,
        in("rdi") socket,
        in("rsi") level,
        in("rdx") option_name,
        in("r10") option_value.as_ptr(),
        in("r8") option_value.len() as i32,
        // Linux syscalls don't touch the stack at all, so
        // we don't care about its alignment
        options(nostack)
    );
    out
}

pub unsafe fn shutdown(socket: i32, how: i32) -> i32 {
    let out: i32;
    asm!(
        "syscall",
        inout("rax") 48 => out,
        in("rdi") socket,
        in("rsi") how,
        // Linux syscalls don't touch the stack at all, so
        // we don't care about its alignment
        options(nostack)
    );
    out
}

pub unsafe fn socket(domain: i32, type_: i32, protocol: i32) -> i32 {
    let out: i32;
    asm!(
        "syscall",
        inout("rax") 41 => out,
        in("rdi") domain,
        in("rsi") type_,
        in("rdx") protocol,
        // Linux syscalls don't touch the stack at all, so
        // we don't care about its alignment
        options(nostack)
    );

    out
}

pub unsafe fn write(fd: u32, buf: *const u8, count: usize) -> isize {
    const SYSCALL_NUMBER: u64 = 1;

    let out: isize;
    asm!(
        "syscall",
        inout("rax") SYSCALL_NUMBER=>out,
        in("rdi") fd,
        in("rsi") buf,
        in("rdx") count,
        // Linux syscalls don't touch the stack at all, so
        // we don't care about its alignment
        options(nostack)
    );

    out
}
