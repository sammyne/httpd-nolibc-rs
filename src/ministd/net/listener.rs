pub struct TcpListener(i32);

impl TcpListener {
    pub fn as_raw_fd(&self) -> i32 {
        self.0
    }

    pub fn bind(addr: &str) -> Result<Self, i32> {
        todo!()
    }
}
