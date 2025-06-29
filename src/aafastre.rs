use super::aamktabl::aa_mktable;
use super::aastructs::*;

pub fn aa_fastrender(mut c: *mut aa_context, mut x1: i64, mut y1: i64, mut x2: i64, mut y2: i64) {
    unsafe {
        let mut x = 0;
        let mut y = 0;
        let mut val = 0;
        let mut wi = (*c).imgwidth;
        let mut pos = 0;
        let mut pos1 = 0;
        if x2 < 0 || y2 < 0 || x1 > (*c).params.width || y1 > (*c).params.height {
            return;
        }
        if x2 >= (*c).params.width {
            x2 = (*c).params.width;
        }
        if y2 >= (*c).params.height {
            y2 = (*c).params.height;
        }
        if x1 < 0 {
            x1 = 0;
        }
        if y1 < 0 {
            y1 = 0;
        }
        if ((*c).table).is_null() {
            aa_mktable(c);
        }
        y = y1;
        while y < y2 {
            pos = 2 * y * wi;
            pos1 = y * (*c).params.width;
            x = x1;
            while x < x2 {
                val = *((*c).table).offset(
                    (((u32::from(*((*c).imagebuffer).offset(pos as isize)) >> 4) << 8)
                        + ((u32::from(*((*c).imagebuffer).offset((pos + 1) as isize)) >> 4) << 12)
                        + (u32::from(*((*c).imagebuffer).offset((pos + wi) as isize)) >> 4)
                        + (u32::from((*((*c).imagebuffer).offset((pos + 1 + wi) as isize)) >> 4)
                            << 4)) as isize,
                );
                *((*c).attrbuffer).offset(pos1 as isize) = (val >> 8) as std::ffi::c_uchar;
                *((*c).textbuffer).offset(pos1 as isize) = (val & 0xff) as std::ffi::c_uchar;
                pos += 2;
                pos1 += 1;
                x += 1;
            }
            y += 1;
        }
    }
}
