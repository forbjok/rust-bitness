# Bitness [![Build Status](https://travis-ci.org/forbjok/rust-bitness.svg?branch=master)](https://travis-ci.org/forbjok/rust-bitness) [![Crates.io](https://img.shields.io/crates/v/bitness.svg)]()

Rust library for detecting OS bitness independently of the executable's bitness. Windows, GNU/Linux and FreeBSD currently supported.

## How to use

```rust
let bitness = bitness::os_bitness().unwrap();

match bitness {
  Bitness::X86_32 => println!("We're 32-bit!"),
  Bitness::X86_64 => println!("We're 64-bit!"),
  _ => { }
}
```
