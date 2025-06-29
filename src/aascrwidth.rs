use super::aastructs::*;

pub fn aa_scrwidth(mut a: *mut aa_context) -> i64 {
    unsafe {
        return (*a).params.width;
    }
}
