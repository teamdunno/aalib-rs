use super::aastructs::*;

pub unsafe extern "C" fn aa_attrs(mut a: *mut aa_context) -> *mut std::ffi::c_char {
    return (*a).attrbuffer as *mut std::ffi::c_char;
}
