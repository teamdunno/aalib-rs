use super::aastructs::*;

pub fn aa_imgheight(a: *mut aa_context) -> i32 {
    unsafe {
        return (*a).imgheight as i32;
    }
}
