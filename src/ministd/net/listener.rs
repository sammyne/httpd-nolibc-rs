use core::ptr;

use crate::c::SockaddrIn;
use crate::ministd::net::{SocketAddrV4, TcpStream};
use crate::syscalls;

const AF_INET: u16 = 2;
const IPPROTO_TCP: i32 = 6;
const SOCK_STREAM: i32 = 1;
const SOL_SOCKET: i32 = 1;
const SO_REUSEADDR: i32 = 2;

pub struct TcpListener(i32);

impl TcpListener {
    //pub fn as_raw_fd(&self) -> i32 {
    //    self.0
    //}

    pub fn accept(&self) -> Result<TcpStream, &'static str> {
        let conn = unsafe { syscalls::accept(self.0, ptr::null(), 0) };
        if conn < 0 {
            return Err("fail");
        }

        Ok(TcpStream(conn))
    }

    pub fn bind(addr: SocketAddrV4) -> Result<Self, &'static str> {
        let addr = SockaddrIn {
            sin_family: AF_INET,
            sin_port: addr.port.to_be(),
            sin_addr: addr.ip.0.to_be(),
            ..Default::default()
        };

        let sock = unsafe { syscalls::socket(AF_INET as i32, SOCK_STREAM, IPPROTO_TCP) };
        if sock < 0 {
            return Err("socket");
        }

        let opt: _ = 1i32.to_ne_bytes();
        if unsafe { syscalls::setsockopt(sock, SOL_SOCKET, SO_REUSEADDR, &opt) } != 0 {
            return Err("setsockopt");
        }

        if unsafe { syscalls::bind(sock, &addr) } != 0 {
            return Err("bind");
        }

        if unsafe { syscalls::listen(sock, 10) } != 0 {
            return Err("listen");
        }

        Ok(Self(sock))
    }
}
