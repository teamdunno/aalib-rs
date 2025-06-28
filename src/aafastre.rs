use super::aamktabl::aa_mktable;
use super::aastructs::*;

pub unsafe extern "C" fn aa_fastrender(
    mut c: *mut aa_context,
    mut x1: std::ffi::c_int,
    mut y1: std::ffi::c_int,
    mut x2: std::ffi::c_int,
    mut y2: std::ffi::c_int,
) {
    let mut x: std::ffi::c_int = 0;
    let mut y: std::ffi::c_int = 0;
    let mut val: std::ffi::c_int = 0;
    let mut wi: std::ffi::c_int = (*c).imgwidth;
    let mut pos: std::ffi::c_int = 0;
    let mut pos1: std::ffi::c_int = 0;
    if x2 < 0 as std::ffi::c_int
        || y2 < 0 as std::ffi::c_int
        || x1 > (*c).params.width
        || y1 > (*c).params.height
    {
        return;
    }
    if x2 >= (*c).params.width {
        x2 = (*c).params.width;
    }
    if y2 >= (*c).params.height {
        y2 = (*c).params.height;
    }
    if x1 < 0 as std::ffi::c_int {
        x1 = 0 as std::ffi::c_int;
    }
    if y1 < 0 as std::ffi::c_int {
        y1 = 0 as std::ffi::c_int;
    }
    if ((*c).table).is_null() {
        aa_mktable(c);
    }
    y = y1;
    while y < y2 {
        pos = 2 as std::ffi::c_int * y * wi;
        pos1 = y * (*c).params.width;
        x = x1;
        while x < x2 {
            val = *((*c).table).offset(
                (((*((*c).imagebuffer).offset(pos as isize) as std::ffi::c_int
                    >> 4 as std::ffi::c_int)
                    << 8 as std::ffi::c_int)
                    + ((*((*c).imagebuffer).offset((pos + 1 as std::ffi::c_int) as isize)
                        as std::ffi::c_int
                        >> 4 as std::ffi::c_int)
                        << 12 as std::ffi::c_int)
                    + (*((*c).imagebuffer).offset((pos + wi) as isize) as std::ffi::c_int
                        >> 4 as std::ffi::c_int)
                    + ((*((*c).imagebuffer).offset((pos + 1 as std::ffi::c_int + wi) as isize)
                        as std::ffi::c_int
                        >> 4 as std::ffi::c_int)
                        << 4 as std::ffi::c_int)) as isize,
            ) as std::ffi::c_int;
            *((*c).attrbuffer).offset(pos1 as isize) =
                (val >> 8 as std::ffi::c_int) as std::ffi::c_uchar;
            *((*c).textbuffer).offset(pos1 as isize) =
                (val & 0xff as std::ffi::c_int) as std::ffi::c_uchar;
            pos += 2 as std::ffi::c_int;
            pos1 += 1;
            pos1;
            x += 1;
            x;
        }
        y += 1;
        y;
    }
}
