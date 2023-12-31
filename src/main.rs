#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use ministd::env::{self, Args};
use ministd::fs::File;
use ministd::net::{SocketAddrV4, TcpListener, TcpStream};
use ministd::{prelude::*, process};

mod c;
mod ministd;
mod start;
mod syscalls;

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
                http_serve(conn, filename).expect("serve");
                return;
            }
            Ok(_) => {}
        }
    }
}

fn http_consume(conn: &TcpStream) {
    let mut buf = [0u8; 8192];
    loop {
        let n = match conn.read(&mut buf) {
            Ok(n) => n,
            Err(_) => {
                perror("read");
                return;
            }
        };

        let s = unsafe { ministd::str::from_utf8_unchecked(&buf[..n]) };
        print(s);

        if n < 3 || s.ends_with("\n\r\n") {
            return;
        }
    }
}

fn http_serve(conn: TcpStream, filename: &str) -> Result<(), i32> {
    http_consume(&conn);

    // 假设 filename 源自 argv[i]，因此底层是合法的 C 字符串
    let f = match File::open(filename) {
        Ok(f) => f,
        Err(_) => {
            perror("open");
            conn.write(b"HTTP/1.1 404 NOT FOUND\r\n\r\n404 NOT FOUND\n")
                .map_err(|err| trace_err("write 'NOT FOUND' http header", err))?;

            return Err(1);
        }
    };
    conn.write(b"HTTP/1.1 200 OK\r\n\r\n")
        .map_err(|err| trace_err("write ok http header", err))?;

    let mut buf = [0u8; 8192];
    loop {
        let n = match f.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => n,
            Err(_) => {
                perror("read");
                break;
            }
        };

        let _ = conn
            .write(&buf[..n])
            .map_err(|err| trace_err("write", err))?;
    }

    conn.shutdown(ministd::net::Shutdown::Both)?;

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

fn perror(s: &str) {
    println!("[ERROR] ", s);
}

fn trace_err(ctx: &str, err: i32) -> i32 {
    perror(ctx);
    err
}

fn usage(argv0: &str) {
    print("usage: ");
    print(argv0);
    println(" [ip]:port file");
}
