use super::aastructs::*;

pub fn aa_image(a: *mut aa_context) -> *mut std::ffi::c_char {
    unsafe {
        return (*a).imagebuffer as *mut std::ffi::c_char;
    }
}
