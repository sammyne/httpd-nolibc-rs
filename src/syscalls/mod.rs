use core::arch::asm;

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
