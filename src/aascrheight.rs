use super::aastructs::*;

pub fn aa_scrheight(mut a: *mut aa_context) -> i64 {
    unsafe {
        return (*a).params.height;
    }
}
