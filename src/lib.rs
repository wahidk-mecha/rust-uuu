#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::raw;
    
    #[test]
    fn get_version() {
        let version_ptr: * const raw::c_char = unsafe { uuu_get_version_string() }.into();
        println!("uuu version: {:?}", unsafe { std::ffi::CStr::from_ptr(version_ptr) });
    }
}
