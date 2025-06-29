use super::aastructs::*;

pub fn aa_scrwidth(a: *mut aa_context) -> i64 {
    unsafe {
        return (*a).params.width;
    }
}
