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
pub fn os_bitness() -> BitnessResult<Bitness> {
    use std::mem;
    use winapi::um::sysinfoapi::{GetNativeSystemInfo, SYSTEM_INFO, SYSTEM_INFO_u_s};
    use winapi::um::winnt::{PROCESSOR_ARCHITECTURE_INTEL, PROCESSOR_ARCHITECTURE_AMD64};

    // Allocate zeroed SYSTEM_INFO struct
    let mut system_info: SYSTEM_INFO = unsafe { mem::zeroed() };

    // Retrieve native system info from Windows API
    unsafe { GetNativeSystemInfo(&mut system_info) };

    // Get SYSTEM_INFO_u_s from SYSTEM_INFO
    let s: &SYSTEM_INFO_u_s = unsafe { system_info.u.s() };

    Ok(match s.wProcessorArchitecture {
        PROCESSOR_ARCHITECTURE_INTEL => Bitness::X86_32,
        PROCESSOR_ARCHITECTURE_AMD64 => Bitness::X86_64,
        _ => Bitness::Unknown,
    })
}
