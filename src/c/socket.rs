use core::ffi::c_char;

/// must be padded to at least 16 bytes
#[derive(Default)]
#[repr(C)]
pub struct SockaddrIn {
    pub sin_family: u16,
    pub sin_port: u16,
    pub sin_addr: u32,
    pub sin_zero: [c_char; 8],
}
