use core::ffi::CStr;
use core::slice::{self, Iter};

use crate::syscalls;

pub struct Args<'a> {
    iter: Iter<'a, *const u8>,
}

impl<'a> Args<'a> {
    unsafe fn new(argc: u64, argv: *const *const u8) -> Self {
        let iter = slice::from_raw_parts(argv, argc as usize).iter();
        Self { iter }
    }
}

impl<'a> Iterator for Args<'a> {
    type Item = &'a CStr;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe { self.iter.next().map(|&v| CStr::from_ptr(v as *const i8)) }
    }
}

pub unsafe fn args<'a>(v: *const u8) -> Args<'a> {
    let argc = *(v as *const u64);
    //syscalls::exit(argc as _);

    let argv = v.add(8) as *const *const u8;

    Args::new(argc, argv)
}
