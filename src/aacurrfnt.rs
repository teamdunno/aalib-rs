use super::aastructs::*;

pub unsafe extern "C" fn aa_currentfont(mut a: *mut aa_context) -> *const aa_font {
    return (*a).params.font;
}
