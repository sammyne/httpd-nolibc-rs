#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use ministd::env::{self, Args};
use ministd::net::{SocketAddrV4, TcpListener};
use ministd::{prelude::*, process};

mod c;
mod ministd;
mod start;
mod syscalls;

const SHUT_RDWR: i32 = 2;
const O_RDONLY: i32 = 0;

fn main(args: Args<'_>) {
    let (addr, filename) = must_parse_args(args);

    let listener = TcpListener::bind(addr).expect("tcp bind");

    loop {
        let conn = match listener.accept() {
            Ok(v) => v,
            Err(err) => {
                println!("[ERROR] accept ", err);
                continue;
            }
        };

        match process::fork() {
            Err(_) => {
                perror("fork");
                continue;
            }
            Ok(0) => {
                // todo: 处理错误码
                unsafe { http_serve(conn.as_raw_fd(), filename).expect("serve") };
                return;
            }
            Ok(_) => {}
        }
    }
}

unsafe fn http_consume(fd: i32) {
    let mut buf = [0u8; 8192];
    loop {
        let n = syscalls::read(fd, buf.as_mut_ptr(), buf.len());
        if n < 0 {
            perror("read");
            return;
        }
        let n = n as usize;

        let s = ministd::str::from_utf8_unchecked(&buf[..n]);
        print(s);

        if n < 3 || s.ends_with("\n\r\n") {
            return;
        }
    }
}

unsafe fn http_drop(conn: i32) {
    let _ = syscalls::shutdown(conn, SHUT_RDWR);
    let _ = syscalls::close(conn);
}

unsafe fn http_serve(fd: i32, filename: &str) -> Result<(), i32> {
    http_consume(fd);

    // 假设 filename 源自 argv[i]，因此底层是合法的 C 字符串
    let f = syscalls::open(filename.as_ptr() as *const i8, O_RDONLY);
    if f < 0 {
        perror("open");
        writeln(fd, concat!("HTTP/1.1 404 NOT FOUND\r\n\r\n404 NOT FOUND"));
        return Err(1);
    }
    writeln(fd, "HTTP/1.1 200 OK\r\n\r\n");

    let mut buf = [0u8; 8192];
    loop {
        let n = match syscalls::read(f, buf.as_mut_ptr(), buf.len()) {
            v if v < 0 => {
                perror("read");
                break;
            }
            0 => break,
            n => n as usize,
        };

        let n = write(fd, &buf[..n]);
        if n < 0 {
            perror("write");
            return Err(1);
        }
    }

    http_drop(fd);

    Ok(())
}

fn must_parse_args(args: Args<'_>) -> (SocketAddrV4, &str) {
    let mut args = args.map(|v| unsafe { ministd::str::from_utf8_unchecked(v) });

    match (args.next(), args.next().map(|v| v.parse()), args.next()) {
        (Some(_), Some(Ok(addr)), Some(filename)) => (addr, filename),
        (Some(_), Some(Err(err)), _) => {
            println!("failed to parse addr: ", err);
            process::exit(1);
        }
        (Some(argv0), _, _) => {
            usage(argv0);
            process::exit(1);
        }
        _ => unreachable!(),
    }
}

fn usage(argv0: &str) {
    print("usage: ");
    print(argv0);
    println(" [ip:]port file");
}

fn perror(s: &str) {
    print("ERROR: ");
    println(s);
}
