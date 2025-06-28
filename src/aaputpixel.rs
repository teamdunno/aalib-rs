use super::aastructs::*;

pub unsafe extern "C" fn aa_putpixel(
    mut c: *mut aa_context,
    mut x: std::ffi::c_int,
    mut y: std::ffi::c_int,
    mut color: std::ffi::c_int,
) {
    *((*c).imagebuffer).offset((x + y * (*c).imgwidth) as isize) = color as std::ffi::c_uchar;
}
