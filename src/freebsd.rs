use sysctl::Sysctl;

use super::*;

/// Get the OS's bitness independently of the executable's bitness.
///
/// This means if the user is running a 64-bit OS, `os_bitness()`
/// will return `Bitness::X86_64` regardless of whether the executable is compiled
/// for x86 or x86-64.
///
/// # Examples
/// ```
/// let bitness = bitness::os_bitness().unwrap();
/// ```
pub fn os_bitness() -> Result<Bitness, BitnessError> {
    /* Use kern.supported_archs, as hw.machine only returns the architecture of the executable. */
    let supported_archs = {
        let ctl = sysctl::Ctl::new("kern.supported_archs").map_err(|err| BitnessError::Sysctl(err.to_string()))?;
        ctl.value().map_err(|err| BitnessError::Sysctl(err.to_string()))?
    };

    Ok(if let sysctl::CtlValue::String(supported_archs) = supported_archs {
        if supported_archs.split(' ').any(|m| m == "amd64") {
            Bitness::X86_64
        }
        else if supported_archs.split(' ').any(|m| m == "i386") {
            Bitness::X86_32
        }
        else {
            Bitness::Unknown
        }
    }
    else {
        Bitness::Unknown
    })
}
