use super::aastructs::*;

pub unsafe extern "C" fn aa_text(mut a: *mut aa_context) -> *mut std::ffi::c_char {
    return (*a).textbuffer as *mut std::ffi::c_char;
}
