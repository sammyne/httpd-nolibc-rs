use core::slice::{self, Iter};

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
    /// 为何不用 CStr：CStr 计算长度过程因未知原因失败
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            self.iter
                .next()
                .map(|&v| slice::from_raw_parts(v, crate::c::strlen(v)))
        }
    }
}

pub unsafe fn args<'a>(v: *const u8) -> Args<'a> {
    let argc = *(v as *const u64);

    let argv = v.add(8) as *const *const u8;

    Args::new(argc, argv)
}
