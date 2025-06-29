use super::aastructs::*;

pub fn aa_imgheight(a: *mut aa_context) -> i64 {
    unsafe {
        return (*a).imgheight;
    }
}
