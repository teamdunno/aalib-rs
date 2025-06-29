use super::aastructs::*;

pub fn aa_scrheight(a: *mut aa_context) -> i64 {
    unsafe {
        return (*a).params.height;
    }
}
