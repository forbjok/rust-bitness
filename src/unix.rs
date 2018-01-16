use uname;

use super::*;

/// Get the OS's bitness independently of the executable's bitness.
///
/// This means if the user is running a 64-bit OS, os_bitness()
/// will return Bitness::X86_64 regardless of whether the executable is compiled
/// for x86 or x86-64.
///
/// # Examples
/// ```
/// let bitness = bitness::os_bitness().unwrap();
/// ```
pub fn os_bitness() -> BitnessResult<Bitness> {
    let info = uname::uname()?;

    Ok(match info.machine.as_ref() {
        "i686" => Bitness::X86_32,
        "x86_64" => Bitness::X86_64,
        _ => Bitness::Unknown,
    })
}
