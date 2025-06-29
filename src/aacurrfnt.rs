use super::aastructs::*;

pub fn aa_currentfont(a: *mut aa_context) -> *const aa_font {
    unsafe {
        return (*a).params.font;
    }
}
