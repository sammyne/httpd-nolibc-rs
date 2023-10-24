#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::ptr;

use ministd::env::{self, Args};
use ministd::net::SocketAddrV4;
use ministd::{prelude::*, process};

mod c;
mod ministd;
mod start;
mod syscalls;

use c::SockaddrIn;

const AF_INET: u16 = 2;
const SOCK_STREAM: i32 = 1;
const IPPROTO_TCP: i32 = 6;
const SO_REUSEADDR: i32 = 2;
const SOL_SOCKET: i32 = 1;
const SHUT_RDWR: i32 = 2;
const O_RDONLY: i32 = 0;

fn main(args: Args<'_>) {
    let (addr, filename) = must_parse_args(args);

    let addr = SockaddrIn {
        sin_family: AF_INET,
        sin_port: addr.port.to_be(),
        sin_addr: addr.ip.0.to_be(),
        ..Default::default()
    };

    let yes: _ = 1i32.to_ne_bytes();

    let sock = unsafe { must_tcp_listen(&addr, &yes) };
    loop {
        let conn = unsafe { syscalls::accept(sock, ptr::null(), 0) };
        if conn < 0 {
            perror("accept");
            continue;
        }

        let pid = unsafe { syscalls::fork() };
        if pid < 0 {
            perror("fork");
            continue;
        } else if pid == 0 {
            // todo: 处理错误码
            unsafe { http_serve(conn, filename).expect("serve") };
            return;
        }
    }
}

fn die(s: &str) -> ! {
    print("FATAL: ");
    println(s);
    process::exit(1);
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

// 待整理
unsafe fn must_tcp_listen(addr: &SockaddrIn, opt: &[u8]) -> i32 {
    let sock = syscalls::socket(AF_INET as i32, SOCK_STREAM, IPPROTO_TCP);
    if sock < 0 {
        die("socket");
    }

    let err = syscalls::setsockopt(sock, SOL_SOCKET, SO_REUSEADDR, opt);
    if err != 0 {
        die("setsockopt");
    }

    let err = syscalls::bind(sock, addr);
    if err != 0 {
        die("bind");
    }

    let err = syscalls::listen(sock, 10);
    if err != 0 {
        die("listen");
    }

    sock
}

fn perror(s: &str) {
    print("ERROR: ");
    println(s);
}
