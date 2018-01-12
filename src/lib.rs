#[cfg(windows)]
extern crate winapi;
#[cfg(unix)]
extern crate uname;

#[cfg(windows)]
mod windows;
#[cfg(unix)]
mod unix;

#[cfg(windows)]
pub use windows::*;
#[cfg(unix)]
pub use unix::*;

#[derive(Debug, PartialEq)]
pub enum Bitness {
    X86_32,
    X86_64,
    Unknown,
}
