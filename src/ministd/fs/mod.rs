use crate::syscalls;

const O_RDONLY: i32 = 0;

pub struct File(i32);

impl File {
    pub fn read(&self, buf: &mut [u8]) -> Result<usize, i32> {
        match unsafe { syscalls::read(self.0, buf.as_mut_ptr(), buf.len()) } {
            v if v < 0 => Err(v as i32),
            v => Ok(v as usize),
        }
    }

    /// @warn: filename 底层指针必须指向合法的 C 字符串。
    pub fn open(filename: &str) -> Result<Self, i32> {
        match unsafe { syscalls::open(filename.as_ptr() as *const i8, O_RDONLY) } {
            v if v >= 0 => Ok(Self(v)),
            v => Err(v),
        }
    }
}

impl Drop for File {
    fn drop(&mut self) {
        unsafe { syscalls::close(self.0) };
    }
}
