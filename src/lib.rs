#[cfg(windows)]
mod windows;
#[cfg(all(unix, not(target_os = "freebsd")))]
mod unix;
#[cfg(target_os = "freebsd")]
mod freebsd;

mod result;

#[cfg(windows)]
pub use windows::*;
#[cfg(all(unix, not(target_os = "freebsd")))]
pub use unix::*;
#[cfg(target_os = "freebsd")]
pub use freebsd::*;

pub use result::*;

#[derive(Debug, PartialEq)]
pub enum Bitness {
    X86_32,
    X86_64,
    Unknown,
}
