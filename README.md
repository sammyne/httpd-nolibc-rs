# httpd-nolibc-rs

## 快速开始

```bash
cargo run -- 8080 static/index.html
```

## 温馨提示
- 降低编译所得可执行文件大小的方式参见 [Minimizing Rust Binary Size]，最后可执行文件大小约为 6.8KiB。

## 参考文献
- [A no_std Rust binary](https://fasterthanli.me/series/making-our-own-executable-packer/part-12)
- [Francesco149/nolibc-httpd](https://github.com/Francesco149/nolibc-httpd)
- [LINUX SYSTEM CALL TABLE FOR X86 64](https://blog.rchapman.org/posts/Linux_System_Call_Table_for_x86_64/)
- [writing a 5kb http server without libc](https://www.youtube.com/watch?v=K2Re0pG_1g4)

[Minimizing Rust Binary Size]: https://github.com/johnthagen/min-sized-rust
