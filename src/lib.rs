#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

extern "C" fn print_usb_device(
    path: *const ::std::os::raw::c_char,
    chip: *const ::std::os::raw::c_char,
    pro: *const ::std::os::raw::c_char,
    vid: u16,
    pid: u16,
    bcd: u16,
    serial_no: *const ::std::os::raw::c_char,
    _p: *mut ::std::os::raw::c_void,
) -> ::std::os::raw::c_int {
    let path_str = unsafe { std::ffi::CStr::from_ptr(path) }.to_str().unwrap();
    let chip_str = unsafe { std::ffi::CStr::from_ptr(chip) }.to_str().unwrap();
    let pro_str = unsafe { std::ffi::CStr::from_ptr(pro) }.to_str().unwrap();
    let serial_str = unsafe { std::ffi::CStr::from_ptr(serial_no) }
        .to_str()
        .unwrap();

    println!(
        "\t{}\t {}\t {}\t 0x{:04X}\t0x{:04X}\t 0x{:04X}\t {}",
        path_str, chip_str, pro_str, vid, pid, bcd, serial_str
    );

    0
}

pub fn print_lsusb() {
    println!("Connected Known USB Devices");
    println!("\tPath\t Chip\t Pro\t Vid\t Pid\t BcdVersion\t Serial_no");
    println!("\t====================================================================");
    unsafe {
        uuu_for_each_devices(Some(print_usb_device), std::ptr::null_mut());
    }
}
