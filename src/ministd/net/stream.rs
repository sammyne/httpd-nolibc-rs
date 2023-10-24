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
    pub fn as_raw_fd(&self) -> i32 {
        self.0
    }

    pub fn shutdown(&self, how: Shutdown) -> Result<(), i32> {
        match unsafe { syscalls::shutdown(self.0, how as i32) } {
            0 => Ok(()),
            v => Err(v),
        }
    }
}

impl Drop for TcpStream {
    fn drop(&mut self) {
        unsafe { syscalls::close(self.0) };
    }
}
