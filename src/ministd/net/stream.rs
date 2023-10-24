pub struct TcpStream(pub(super) i32);

impl TcpStream {
    pub fn as_raw_fd(&self) -> i32 {
        self.0
    }
}
