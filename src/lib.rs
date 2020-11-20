#[cfg(target_os = "freebsd")]
mod freebsd;
#[cfg(all(unix, not(target_os = "freebsd")))]
mod unix;
#[cfg(windows)]
mod windows;

mod error;

#[cfg(target_os = "freebsd")]
pub use freebsd::*;
#[cfg(all(unix, not(target_os = "freebsd")))]
pub use unix::*;
#[cfg(windows)]
pub use windows::*;

pub use error::*;

#[derive(Debug, PartialEq)]
pub enum Bitness {
    X86_32,
    X86_64,
    Unknown,
}
