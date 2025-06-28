use super::aastructs::*;

pub unsafe extern "C" fn aa_mmwidth(mut a: *mut aa_context) -> std::ffi::c_int {
    return (*a).params.mmwidth;
}
