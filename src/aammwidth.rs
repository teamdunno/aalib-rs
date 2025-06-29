use super::aastructs::*;

pub fn aa_mmwidth(a: *mut aa_context) -> i64 {
    unsafe {
        return (*a).params.mmwidth;
    }
}
