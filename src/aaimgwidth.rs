use super::aastructs::*;

pub fn aa_imgwidth(a: *mut aa_context) -> i64 {
    unsafe {
        return (*a).imgwidth;
    }
}
