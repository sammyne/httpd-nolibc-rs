#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use ministd::env::{self, Args};
use ministd::{prelude::*, process};

mod c;
mod ministd;
mod start;
mod syscalls;

use c::SockaddrIn;

const AF_INET: u16 = 2;
//const SOCK_STREAM 1
//const IPPROTO_TCP 6
//const SO_REUSEADDR 2
//const SOL_SOCKET 1
//const SHUT_RDWR 2
//const O_RDONLY 0

fn main(args: Args<'_>) {
    let (port, filename) = must_parse_args(args);
    println(filename);

    let addr = SockaddrIn {
        sin_family: AF_INET,
        sin_port: port,
        sin_addr: 0,
        ..Default::default()
    };

    //loop {
    //
    //}

    //for v in args {
    //    let s = str::from_utf8_unchecked(v);
    //    println(s);
    //}
}

fn must_parse_args(args: Args<'_>) -> (u16, &str) {
    let mut args = args.map(|v| unsafe { ministd::str::from_utf8_unchecked(v) });

    match (
        args.next(),
        args.next().map(|v| v.parse::<u16>()),
        args.next(),
    ) {
        (Some(_), Some(Ok(port)), Some(filename)) => (port, filename),
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
    println(" port file");
}

// 待整理
fn must_tcp_listen(addr: &SockaddrIn, opt: &[u8]) -> i32 {
    todo!()
}
