use super::aastructs::*;

pub fn aa_putpixel(c: *mut aa_context, x: i64, y: i64, color: i64) {
    unsafe {
        *((*c).imagebuffer).offset((x + y * (*c).imgwidth) as isize) = color as std::ffi::c_uchar;
    }
}
