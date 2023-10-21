use crate::syscalls::{self, FILENO_STDOUT};

pub fn print(s: &str) {
    let b = s.as_bytes();
    unsafe {
        syscalls::write(FILENO_STDOUT, b.as_ptr(), b.len());
    }
}

pub fn println(s: &str) {
    print(s);
    print("\n");
}
