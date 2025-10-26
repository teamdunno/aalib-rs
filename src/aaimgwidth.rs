use super::aastructs::*;

pub fn aa_imgwidth(a: *mut aa_context) -> i32 {
    unsafe {
        return (*a).imgwidth as i32;
    }
}
