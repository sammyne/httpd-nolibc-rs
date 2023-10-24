use crate::syscalls;

pub fn exit(code: i32) -> ! {
    unsafe { syscalls::exit(code) };
}

/// 成功时，父进程得到 Ok(v>0)，子进程得到 Ok(0)
pub fn fork() -> Result<u32, i32> {
    match unsafe { syscalls::fork() } {
        v if v < 0 => Err(v),
        v => Ok(v as u32),
    }
}
