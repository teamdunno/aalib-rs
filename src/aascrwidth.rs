use super::aastructs::*;

pub unsafe extern "C" fn aa_scrwidth(mut a: *mut aa_context) -> std::ffi::c_int {
    return (*a).params.width;
}
