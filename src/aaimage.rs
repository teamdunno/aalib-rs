use super::aastructs::*;

pub unsafe extern "C" fn aa_image(mut a: *mut aa_context) -> *mut std::ffi::c_char {
    return (*a).imagebuffer as *mut std::ffi::c_char;
}
