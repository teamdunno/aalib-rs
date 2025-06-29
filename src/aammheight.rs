use super::aastructs::*;

pub fn aa_mmheight(mut a: *mut aa_context) -> i64 {
    unsafe {
        return (*a).params.mmheight;
    }
}
