use super::aastructs::*;

unsafe extern "C" {
    fn __ctype_b_loc() -> *mut *const std::ffi::c_ushort;
}
pub type aa_attribute = std::ffi::c_uint;
pub const AA_SPECIAL: aa_attribute = 5;
pub const AA_REVERSE: aa_attribute = 4;
pub const AA_BOLDFONT: aa_attribute = 3;
pub const AA_BOLD: aa_attribute = 2;
pub const AA_DIM: aa_attribute = 1;
pub const AA_NORMAL: aa_attribute = 0;
static mut DIMC: std::ffi::c_double = 0.;
static mut CONSTANT: std::ffi::c_double = 0.;
static mut currfont: *const aa_font = 0 as *const aa_font;
unsafe fn values(
    mut c: i64,
    v1: &mut i64,
    v2: &mut i64,
    v3: &mut i64,
    v4: &mut i64,
) { unsafe {
    let mut i = 0;
    let attr = c / 256;
    let mut font: *const std::ffi::c_uchar = 0 as *const std::ffi::c_uchar;
    font = (*currfont).data;
    font = (*currfont).data;
    c = c % 256;
    c = c * (*currfont).height;
    *v1 = 0;
    *v2 = 0;
    *v3 = 0;
    *v4 = 0;
    i = 0;
    while i < (*currfont).height / 2 {
        *v1 += ((*font.offset((c + i) as isize) & (1) << 0 != 0) as i32
            + (*font.offset((c + i) as isize) & (1) << 1 != 0) as i32
            + (*font.offset((c + i) as isize) & (1) << 2 != 0) as i32
            + (*font.offset((c + i) as isize) & (1) << 3 != 0) as i32) as i64;
        *v2 += ((*font.offset((c + i) as isize) & (1) << 4 != 0) as i32
            + (*font.offset((c + i) as isize) & (1) << 5 != 0) as i32
            + (*font.offset((c + i) as isize) & (1) << 6 != 0) as i32
            + (*font.offset((c + i) as isize) & (1) << 7 != 0) as i32) as i64;
        i += 1;
    }
    while i < (*currfont).height {
        *v3 += ((*font.offset((c + i) as isize) & (1) << 0 != 0) as i32
            + (*font.offset((c + i) as isize) & (1) << 1 != 0) as i32
            + (*font.offset((c + i) as isize) & (1) << 2 != 0) as i32
            + (*font.offset((c + i) as isize) & (1) << 3 != 0) as i32) as i64;
        *v4 += ((*font.offset((c + i) as isize) & (1) << 4 != 0) as i32
            + (*font.offset((c + i) as isize) & (1) << 5 != 0) as i32
            + (*font.offset((c + i) as isize) & (1) << 6 != 0) as i32
            + (*font.offset((c + i) as isize) & (1) << 7 != 0) as i32) as i64;
        i += 1;
    }
    *v1 *= 8;
    *v2 *= 8;
    *v3 *= 8;
    *v4 *= 8;
    match attr {
        4 => {
            *v1 = (*currfont).height * 2 * 8 - *v1;
            *v2 = (*currfont).height * 2 * 8 - *v2;
            *v3 = (*currfont).height * 2 * 8 - *v3;
            *v4 = (*currfont).height * 2 * 8 - *v4;
        }
        1 => {
            *v1 = ((*v1 + 1) as f64 / DIMC) as i64;
            *v2 = ((*v2 + 1) as f64 / DIMC) as i64;
            *v3 = ((*v3 + 1) as f64 / DIMC) as i64;
            *v4 = ((*v4 + 1) as f64 / DIMC) as i64;
        }
        2 => {
            *v1 = (*v1 as f64 * CONSTANT) as i64;
            *v2 = (*v2 as f64 * CONSTANT) as i64;
            *v3 = (*v3 as f64 * CONSTANT) as i64;
            *v4 = (*v4 as f64 * CONSTANT) as i64;
        }
        3 => {
            i = 0;
            while i < (*currfont).height / 2 {
                *v1 += (((*font.offset((c + i) as isize) & (1) << 0 != 0) as i32
                    + (!(*font.offset((c + i) as isize) & (1) << 1 != 0)
                        && *font.offset((c + i) as isize) & (1) << 1 - 1 != 0)
                        as i32
                    + (!(*font.offset((c + i) as isize) & (1) << 2 != 0)
                        && *font.offset((c + i) as isize) & (1) << 2 - 1 != 0)
                        as i32
                    + (!(*font.offset((c + i) as isize) & (1) << 3 != 0)
                        && *font.offset((c + i) as isize) & (1) << 3 - 1 != 0)
                        as i32)
                    * 8 as i32) as i64;
                *v2 += (((*font.offset((c + i) as isize) & (1) << 4 != 0) as i32
                    + (!(*font.offset((c + i) as isize) & (1) << 5 != 0)
                        && *font.offset((c + i) as isize) & (1) << 5 - 1 != 0)
                        as i32
                    + (!(*font.offset((c + i) as isize) & (1) << 6 != 0)
                        && *font.offset((c + i) as isize) & (1) << 6 - 1 != 0)
                        as i32
                    + (!(*font.offset((c + i) as isize) & (1) << 7 != 0)
                        && *font.offset((c + i) as isize) & (1) << 7 - 1 != 0)
                        as i32)
                    * 8) as i64;
                i += 1;
            }
            while i < (*currfont).height {
                *v3 += (((*font.offset((c + i) as isize) & (1) << 0 != 0) as i32
                    + (!(*font.offset((c + i) as isize) & (1) << 1 != 0)
                        && *font.offset((c + i) as isize) & (1) << 1 - 1 != 0)
                        as i32
                    + (!(*font.offset((c + i) as isize) & (1) << 2 != 0)
                        && *font.offset((c + i) as isize) & (1) << 2 - 1 != 0)
                        as i32
                    + (!(*font.offset((c + i) as isize) & (1) << 3 != 0)
                        && *font.offset((c + i) as isize) & (1) << 3 - 1 != 0)
                        as i32)
                    * 8) as i64;
                *v4 += (((*font.offset((c + i) as isize) & (1) << 4 != 0) as i32
                    + (!(*font.offset((c + i) as isize) & (1) << 5 != 0)
                        && *font.offset((c + i) as isize) & (1) << 5 - 1 != 0)
                        as i32
                    + (!(*font.offset((c + i) as isize) & (1) << 6 != 0)
                        && *font.offset((c + i) as isize) & (1) << 6 - 1 != 0)
                        as i32
                    + (!(*font.offset((c + i) as isize) & (1) << 7 != 0)
                        && *font.offset((c + i) as isize) & (1) << 7 - 1 != 0)
                        as i32) as i32
                    * 8) as i64;
                i += 1;
            }
        }
        _ => {}
    };
}}

pub fn __aa_calcparams(
    font: *const aa_font,
    parameters: *mut parameters,
    supported: i64,
    dimmul: f64,
    boldmul: f64,
) {
    unsafe {
        let mut i = 0;
        let mut ma1 = 0;
        let mut ma2 = 0;
        let mut ma3 = 0;
        let mut ma4 = 0;
        let mut msum = 0;
        let mut mi1 = 50000;
        let mut mi2 = 50000;
        let mut mi3 = 50000;
        let mut mi4 = 50000;
        let mut misum = 50000;
        let mut v1 = 0;
        let mut v2 = 0;
        let mut v3 = 0;
        let mut v4 = 0;
        let mut sum = 0;
        DIMC = dimmul;
        CONSTANT = boldmul;
        currfont = font;
        i = 0;
        while i < 256 * 5 {
            if (*(*__ctype_b_loc()).offset((i & 0xff) as isize) as u32 & 32768 != 0
                || i & 0xff == ' ' as i64
                || i & 0xff > 160 && supported & 256 != 0
                || supported & 128 != 0 && i & 0xff != 0)
                && supported & 1 << (i >> 8) != 0
            {
                values(i, &mut v1, &mut v2, &mut v3, &mut v4);
                if v1 > ma1 {
                    ma1 = v1;
                }
                if v2 > ma2 {
                    ma2 = v2;
                }
                if v3 > ma3 {
                    ma3 = v3;
                }
                if v4 > ma4 {
                    ma4 = v4;
                }
                if v1 + v2 + v3 + v4 > msum {
                    msum = v1 + v2 + v3 + v4;
                }
                if v1 < mi1 {
                    mi1 = v1;
                }
                if v2 < mi2 {
                    mi2 = v2;
                }
                if v3 < mi3 {
                    mi3 = v3;
                }
                if v4 < mi4 {
                    mi4 = v4;
                }
                if v1 + v2 + v3 + v4 < misum {
                    misum = v1 + v2 + v3 + v4;
                }
            }
            i += 1;
            i;
        }
        msum -= misum;
        mi1 = misum / 4;
        mi2 = misum / 4;
        mi3 = misum / 4;
        mi4 = misum / 4;
        ma1 = msum / 4;
        ma2 = msum / 4;
        ma3 = msum / 4;
        ma4 = msum / 4;
        i = 0;
        while i < 256 * 5 {
            if !(1 as std::ffi::c_int == 0) {
                values(i, &mut v1, &mut v2, &mut v3, &mut v4);
                sum = ((v1 + v2 + v3 + v4 - misum) as f64 * (1020.0 / msum as f64) + 0.5f64) as i64;
                v1 = ((v1 - mi1) as f64 * (255 as f64 / ma1 as f64) + 0.5f64) as i64;
                v2 = ((v2 - mi2) as f64 * (255 as f64 / ma2 as f64) + 0.5f64) as i64;
                v3 = ((v3 - mi3) as f64 * (255 as f64 / ma3 as f64) + 0.5f64) as i64;
                v4 = ((v4 - mi4) as f64 * (255 as f64 / ma4 as f64) + 0.5f64) as i64;
                if v1 > 255 {
                    v1 = 255;
                }
                if v2 > 255 {
                    v2 = 255;
                }
                if v3 > 255 {
                    v3 = 255;
                }
                if v4 > 255 {
                    v4 = 255;
                }
                if v1 < 0 {
                    v1 = 0;
                }
                if v2 < 0 {
                    v2 = 0;
                }
                if v3 < 0 {
                    v3 = 0;
                }
                if v4 < 0 {
                    v4 = 0;
                }
                (*parameters.offset(i as isize)).p[0] = v1;
                (*parameters.offset(i as isize)).p[1] = v2;
                (*parameters.offset(i as isize)).p[2] = v3;
                (*parameters.offset(i as isize)).p[3] = v4;
                (*parameters.offset(i as isize)).p[4] = sum;
            }
            i += 1;
            i;
        }
    }
}
