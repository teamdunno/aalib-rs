use super::aastructs::*;

pub fn aa_text(mut a: *mut aa_context) -> *mut std::ffi::c_char {
    unsafe {
        return (*a).textbuffer as *mut std::ffi::c_char;
    }
}
