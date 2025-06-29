use super::aastructs::*;

pub fn aa_putpixel(mut c: *mut aa_context, mut x: i64, mut y: i64, mut color: i64) {
    unsafe {
        *((*c).imagebuffer).offset((x + y * (*c).imgwidth) as isize) = color as std::ffi::c_uchar;
    }
}
