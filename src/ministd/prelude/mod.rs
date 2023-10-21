use crate::syscalls::{self, FILENO_STDOUT};

pub fn print(s: &str) {
    let b = s.as_bytes();
    unsafe {
        let _ = syscalls::write(FILENO_STDOUT, b.as_ptr(), b.len());
    }
}

pub fn println(s: &str) {
    print(s);
    print("\n");
}

pub fn write<T>(fd: i32, data: T) -> isize
where
    T: AsRef<[u8]>,
{
    let b = data.as_ref();
    unsafe { syscalls::write(fd as u32, b.as_ptr(), b.len()) }
}

pub fn writeln<T>(fd: i32, data: T)
where
    T: AsRef<[u8]>,
{
    let _ = write(fd, data);
    let _ = write(fd, &[b'\n']);
}
