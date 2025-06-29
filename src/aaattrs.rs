use super::aastructs::*;

pub fn aa_attrs(mut a: *mut aa_context) -> *mut std::ffi::c_char {
    unsafe {
        return (*a).attrbuffer as *mut std::ffi::c_char;
    }
}
