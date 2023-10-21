use core::arch::asm;

pub const FILENO_STDOUT: u32 = 1;

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

pub unsafe fn write(fd: u32, buf: *const u8, count: usize) {
    const SYSCALL_NUMBER: u64 = 1;
    asm!(
        "syscall",
        in("rax") SYSCALL_NUMBER,
        in("rdi") fd,
        in("rsi") buf,
        in("rdx") count,
        // Linux syscalls don't touch the stack at all, so
        // we don't care about its alignment
        options(nostack)
    );
}
