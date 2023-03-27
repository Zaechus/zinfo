#[cfg(target_os = "linux")]
pub fn hostname() -> String {
    use std::fs;

    if let Ok(s) = fs::read_to_string("/etc/hostname") {
        s.trim().to_owned()
    } else {
        "hostname".to_owned()
    }
}

#[cfg(windows)]
pub fn hostname() -> String {
    use windows::{
        core::PWSTR,
        Win32::System::SystemInformation::{ComputerNamePhysicalDnsHostname, GetComputerNameExW},
    };

    let mut buffer_size: u32 = 0;
    unsafe {
        GetComputerNameExW(
            ComputerNamePhysicalDnsHostname,
            PWSTR::null(),
            &mut buffer_size,
        );
    }

    let mut buffer = vec![0_u16; buffer_size as usize];
    unsafe {
        GetComputerNameExW(
            ComputerNamePhysicalDnsHostname,
            PWSTR::from_raw(buffer.as_mut_ptr()),
            &mut buffer_size,
        );
    }

    String::from_utf16_lossy(&buffer)
}

#[cfg(not(any(target_os = "linux", windows)))]
pub fn hostname() -> String {
    use crate::get_output;

    if let Ok(o) = get_output("hostname", &[]) {
        o
    } else {
        "hostname".to_owned()
    }
}
