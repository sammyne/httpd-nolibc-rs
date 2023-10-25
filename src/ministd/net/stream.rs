use crate::syscalls;

#[allow(dead_code)]
#[repr(i32)]
pub enum Shutdown {
    Read = 0,
    Write = 1,
    Both = 2,
}

pub struct TcpStream(pub(super) i32);

impl TcpStream {
    //pub fn as_raw_fd(&self) -> i32 {
    //    self.0
    //}

    pub fn read(&self, buf: &mut [u8]) -> Result<usize, i32> {
        match unsafe { syscalls::read(self.0, buf.as_mut_ptr(), buf.len()) } {
            v if v >= 0 => Ok(v as usize),
            v => Err(v as i32),
        }
    }

    pub fn shutdown(&self, how: Shutdown) -> Result<(), i32> {
        match unsafe { syscalls::shutdown(self.0, how as i32) } {
            0 => Ok(()),
            v => Err(v),
        }
    }

    pub fn write(&self, buf: &[u8]) -> Result<usize, i32> {
        let b = buf.as_ref();
        match unsafe { syscalls::write(self.0 as u32, b.as_ptr(), b.len()) } {
            v if v >= 0 => Ok(v as usize),
            v => Err(v as i32),
        }
    }
}

impl Drop for TcpStream {
    fn drop(&mut self) {
        unsafe { syscalls::close(self.0) };
    }
}
