use super::aastructs::*;

pub fn aa_text(a: *mut aa_context) -> *mut std::ffi::c_char {
    unsafe {
        return (*a).textbuffer as *mut std::ffi::c_char;
    }
}
