use super::aastructs::*;

unsafe extern "C" {
    fn __ctype_b_loc() -> *mut *const std::ffi::c_ushort;
}
pub type C2RustUnnamed = std::ffi::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
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
unsafe extern "C" fn values(
    mut c: std::ffi::c_int,
    mut v1: *mut std::ffi::c_int,
    mut v2: *mut std::ffi::c_int,
    mut v3: *mut std::ffi::c_int,
    mut v4: *mut std::ffi::c_int,
) {
    let mut i: std::ffi::c_int = 0;
    let mut attr: std::ffi::c_int = c / 256 as std::ffi::c_int;
    let mut font: *const std::ffi::c_uchar = 0 as *const std::ffi::c_uchar;
    font = (*currfont).data;
    font = (*currfont).data;
    c = c % 256 as std::ffi::c_int;
    c = c * (*currfont).height;
    *v1 = 0 as std::ffi::c_int;
    *v2 = 0 as std::ffi::c_int;
    *v3 = 0 as std::ffi::c_int;
    *v4 = 0 as std::ffi::c_int;
    i = 0 as std::ffi::c_int;
    while i < (*currfont).height / 2 as std::ffi::c_int {
        *v1 += (*font.offset((c + i) as isize) as std::ffi::c_int
            & (1 as std::ffi::c_int) << 0 as std::ffi::c_int
            != 0 as std::ffi::c_int) as std::ffi::c_int
            + (*font.offset((c + i) as isize) as std::ffi::c_int
                & (1 as std::ffi::c_int) << 1 as std::ffi::c_int
                != 0 as std::ffi::c_int) as std::ffi::c_int
            + (*font.offset((c + i) as isize) as std::ffi::c_int
                & (1 as std::ffi::c_int) << 2 as std::ffi::c_int
                != 0 as std::ffi::c_int) as std::ffi::c_int
            + (*font.offset((c + i) as isize) as std::ffi::c_int
                & (1 as std::ffi::c_int) << 3 as std::ffi::c_int
                != 0 as std::ffi::c_int) as std::ffi::c_int;
        *v2 += (*font.offset((c + i) as isize) as std::ffi::c_int
            & (1 as std::ffi::c_int) << 4 as std::ffi::c_int
            != 0 as std::ffi::c_int) as std::ffi::c_int
            + (*font.offset((c + i) as isize) as std::ffi::c_int
                & (1 as std::ffi::c_int) << 5 as std::ffi::c_int
                != 0 as std::ffi::c_int) as std::ffi::c_int
            + (*font.offset((c + i) as isize) as std::ffi::c_int
                & (1 as std::ffi::c_int) << 6 as std::ffi::c_int
                != 0 as std::ffi::c_int) as std::ffi::c_int
            + (*font.offset((c + i) as isize) as std::ffi::c_int
                & (1 as std::ffi::c_int) << 7 as std::ffi::c_int
                != 0 as std::ffi::c_int) as std::ffi::c_int;
        i += 1;
        i;
    }
    while i < (*currfont).height {
        *v3 += (*font.offset((c + i) as isize) as std::ffi::c_int
            & (1 as std::ffi::c_int) << 0 as std::ffi::c_int
            != 0 as std::ffi::c_int) as std::ffi::c_int
            + (*font.offset((c + i) as isize) as std::ffi::c_int
                & (1 as std::ffi::c_int) << 1 as std::ffi::c_int
                != 0 as std::ffi::c_int) as std::ffi::c_int
            + (*font.offset((c + i) as isize) as std::ffi::c_int
                & (1 as std::ffi::c_int) << 2 as std::ffi::c_int
                != 0 as std::ffi::c_int) as std::ffi::c_int
            + (*font.offset((c + i) as isize) as std::ffi::c_int
                & (1 as std::ffi::c_int) << 3 as std::ffi::c_int
                != 0 as std::ffi::c_int) as std::ffi::c_int;
        *v4 += (*font.offset((c + i) as isize) as std::ffi::c_int
            & (1 as std::ffi::c_int) << 4 as std::ffi::c_int
            != 0 as std::ffi::c_int) as std::ffi::c_int
            + (*font.offset((c + i) as isize) as std::ffi::c_int
                & (1 as std::ffi::c_int) << 5 as std::ffi::c_int
                != 0 as std::ffi::c_int) as std::ffi::c_int
            + (*font.offset((c + i) as isize) as std::ffi::c_int
                & (1 as std::ffi::c_int) << 6 as std::ffi::c_int
                != 0 as std::ffi::c_int) as std::ffi::c_int
            + (*font.offset((c + i) as isize) as std::ffi::c_int
                & (1 as std::ffi::c_int) << 7 as std::ffi::c_int
                != 0 as std::ffi::c_int) as std::ffi::c_int;
        i += 1;
        i;
    }
    *v1 *= 8 as std::ffi::c_int;
    *v2 *= 8 as std::ffi::c_int;
    *v3 *= 8 as std::ffi::c_int;
    *v4 *= 8 as std::ffi::c_int;
    match attr {
        4 => {
            *v1 = (*currfont).height * 2 as std::ffi::c_int * 8 as std::ffi::c_int - *v1;
            *v2 = (*currfont).height * 2 as std::ffi::c_int * 8 as std::ffi::c_int - *v2;
            *v3 = (*currfont).height * 2 as std::ffi::c_int * 8 as std::ffi::c_int - *v3;
            *v4 = (*currfont).height * 2 as std::ffi::c_int * 8 as std::ffi::c_int - *v4;
        }
        1 => {
            *v1 = ((*v1 + 1 as std::ffi::c_int) as std::ffi::c_double / DIMC) as std::ffi::c_int;
            *v2 = ((*v2 + 1 as std::ffi::c_int) as std::ffi::c_double / DIMC) as std::ffi::c_int;
            *v3 = ((*v3 + 1 as std::ffi::c_int) as std::ffi::c_double / DIMC) as std::ffi::c_int;
            *v4 = ((*v4 + 1 as std::ffi::c_int) as std::ffi::c_double / DIMC) as std::ffi::c_int;
        }
        2 => {
            *v1 = (*v1 as std::ffi::c_double * CONSTANT) as std::ffi::c_int;
            *v2 = (*v2 as std::ffi::c_double * CONSTANT) as std::ffi::c_int;
            *v3 = (*v3 as std::ffi::c_double * CONSTANT) as std::ffi::c_int;
            *v4 = (*v4 as std::ffi::c_double * CONSTANT) as std::ffi::c_int;
        }
        3 => {
            i = 0 as std::ffi::c_int;
            while i < (*currfont).height / 2 as std::ffi::c_int {
                *v1 += ((*font.offset((c + i) as isize) as std::ffi::c_int
                    & (1 as std::ffi::c_int) << 0 as std::ffi::c_int
                    != 0 as std::ffi::c_int) as std::ffi::c_int
                    + (!(*font.offset((c + i) as isize) as std::ffi::c_int
                        & (1 as std::ffi::c_int) << 1 as std::ffi::c_int
                        != 0 as std::ffi::c_int)
                        && *font.offset((c + i) as isize) as std::ffi::c_int
                            & (1 as std::ffi::c_int) << 1 as std::ffi::c_int - 1 as std::ffi::c_int
                            != 0 as std::ffi::c_int) as std::ffi::c_int
                    + (!(*font.offset((c + i) as isize) as std::ffi::c_int
                        & (1 as std::ffi::c_int) << 2 as std::ffi::c_int
                        != 0 as std::ffi::c_int)
                        && *font.offset((c + i) as isize) as std::ffi::c_int
                            & (1 as std::ffi::c_int) << 2 as std::ffi::c_int - 1 as std::ffi::c_int
                            != 0 as std::ffi::c_int) as std::ffi::c_int
                    + (!(*font.offset((c + i) as isize) as std::ffi::c_int
                        & (1 as std::ffi::c_int) << 3 as std::ffi::c_int
                        != 0 as std::ffi::c_int)
                        && *font.offset((c + i) as isize) as std::ffi::c_int
                            & (1 as std::ffi::c_int) << 3 as std::ffi::c_int - 1 as std::ffi::c_int
                            != 0 as std::ffi::c_int) as std::ffi::c_int)
                    * 8 as std::ffi::c_int;
                *v2 += ((*font.offset((c + i) as isize) as std::ffi::c_int
                    & (1 as std::ffi::c_int) << 4 as std::ffi::c_int
                    != 0 as std::ffi::c_int) as std::ffi::c_int
                    + (!(*font.offset((c + i) as isize) as std::ffi::c_int
                        & (1 as std::ffi::c_int) << 5 as std::ffi::c_int
                        != 0 as std::ffi::c_int)
                        && *font.offset((c + i) as isize) as std::ffi::c_int
                            & (1 as std::ffi::c_int) << 5 as std::ffi::c_int - 1 as std::ffi::c_int
                            != 0 as std::ffi::c_int) as std::ffi::c_int
                    + (!(*font.offset((c + i) as isize) as std::ffi::c_int
                        & (1 as std::ffi::c_int) << 6 as std::ffi::c_int
                        != 0 as std::ffi::c_int)
                        && *font.offset((c + i) as isize) as std::ffi::c_int
                            & (1 as std::ffi::c_int) << 6 as std::ffi::c_int - 1 as std::ffi::c_int
                            != 0 as std::ffi::c_int) as std::ffi::c_int
                    + (!(*font.offset((c + i) as isize) as std::ffi::c_int
                        & (1 as std::ffi::c_int) << 7 as std::ffi::c_int
                        != 0 as std::ffi::c_int)
                        && *font.offset((c + i) as isize) as std::ffi::c_int
                            & (1 as std::ffi::c_int) << 7 as std::ffi::c_int - 1 as std::ffi::c_int
                            != 0 as std::ffi::c_int) as std::ffi::c_int)
                    * 8 as std::ffi::c_int;
                i += 1;
                i;
            }
            while i < (*currfont).height {
                *v3 += ((*font.offset((c + i) as isize) as std::ffi::c_int
                    & (1 as std::ffi::c_int) << 0 as std::ffi::c_int
                    != 0 as std::ffi::c_int) as std::ffi::c_int
                    + (!(*font.offset((c + i) as isize) as std::ffi::c_int
                        & (1 as std::ffi::c_int) << 1 as std::ffi::c_int
                        != 0 as std::ffi::c_int)
                        && *font.offset((c + i) as isize) as std::ffi::c_int
                            & (1 as std::ffi::c_int) << 1 as std::ffi::c_int - 1 as std::ffi::c_int
                            != 0 as std::ffi::c_int) as std::ffi::c_int
                    + (!(*font.offset((c + i) as isize) as std::ffi::c_int
                        & (1 as std::ffi::c_int) << 2 as std::ffi::c_int
                        != 0 as std::ffi::c_int)
                        && *font.offset((c + i) as isize) as std::ffi::c_int
                            & (1 as std::ffi::c_int) << 2 as std::ffi::c_int - 1 as std::ffi::c_int
                            != 0 as std::ffi::c_int) as std::ffi::c_int
                    + (!(*font.offset((c + i) as isize) as std::ffi::c_int
                        & (1 as std::ffi::c_int) << 3 as std::ffi::c_int
                        != 0 as std::ffi::c_int)
                        && *font.offset((c + i) as isize) as std::ffi::c_int
                            & (1 as std::ffi::c_int) << 3 as std::ffi::c_int - 1 as std::ffi::c_int
                            != 0 as std::ffi::c_int) as std::ffi::c_int)
                    * 8 as std::ffi::c_int;
                *v4 += ((*font.offset((c + i) as isize) as std::ffi::c_int
                    & (1 as std::ffi::c_int) << 4 as std::ffi::c_int
                    != 0 as std::ffi::c_int) as std::ffi::c_int
                    + (!(*font.offset((c + i) as isize) as std::ffi::c_int
                        & (1 as std::ffi::c_int) << 5 as std::ffi::c_int
                        != 0 as std::ffi::c_int)
                        && *font.offset((c + i) as isize) as std::ffi::c_int
                            & (1 as std::ffi::c_int) << 5 as std::ffi::c_int - 1 as std::ffi::c_int
                            != 0 as std::ffi::c_int) as std::ffi::c_int
                    + (!(*font.offset((c + i) as isize) as std::ffi::c_int
                        & (1 as std::ffi::c_int) << 6 as std::ffi::c_int
                        != 0 as std::ffi::c_int)
                        && *font.offset((c + i) as isize) as std::ffi::c_int
                            & (1 as std::ffi::c_int) << 6 as std::ffi::c_int - 1 as std::ffi::c_int
                            != 0 as std::ffi::c_int) as std::ffi::c_int
                    + (!(*font.offset((c + i) as isize) as std::ffi::c_int
                        & (1 as std::ffi::c_int) << 7 as std::ffi::c_int
                        != 0 as std::ffi::c_int)
                        && *font.offset((c + i) as isize) as std::ffi::c_int
                            & (1 as std::ffi::c_int) << 7 as std::ffi::c_int - 1 as std::ffi::c_int
                            != 0 as std::ffi::c_int) as std::ffi::c_int)
                    * 8 as std::ffi::c_int;
                i += 1;
                i;
            }
        }
        _ => {}
    };
}

pub unsafe extern "C" fn __aa_calcparams(
    mut font: *const aa_font,
    mut parameters: *mut parameters,
    mut supported: std::ffi::c_int,
    mut dimmul: std::ffi::c_double,
    mut boldmul: std::ffi::c_double,
) {
    let mut i: std::ffi::c_int = 0;
    let mut ma1: std::ffi::c_int = 0 as std::ffi::c_int;
    let mut ma2: std::ffi::c_int = 0 as std::ffi::c_int;
    let mut ma3: std::ffi::c_int = 0 as std::ffi::c_int;
    let mut ma4: std::ffi::c_int = 0 as std::ffi::c_int;
    let mut msum: std::ffi::c_int = 0 as std::ffi::c_int;
    let mut mi1: std::ffi::c_int = 50000 as std::ffi::c_int;
    let mut mi2: std::ffi::c_int = 50000 as std::ffi::c_int;
    let mut mi3: std::ffi::c_int = 50000 as std::ffi::c_int;
    let mut mi4: std::ffi::c_int = 50000 as std::ffi::c_int;
    let mut misum: std::ffi::c_int = 50000 as std::ffi::c_int;
    let mut v1: std::ffi::c_int = 0;
    let mut v2: std::ffi::c_int = 0;
    let mut v3: std::ffi::c_int = 0;
    let mut v4: std::ffi::c_int = 0;
    let mut sum: std::ffi::c_int = 0;
    DIMC = dimmul;
    CONSTANT = boldmul;
    currfont = font;
    i = 0 as std::ffi::c_int;
    while i < 256 as std::ffi::c_int * 5 as std::ffi::c_int {
        if (*(*__ctype_b_loc()).offset((i & 0xff as std::ffi::c_int) as isize) as std::ffi::c_int
            & _ISgraph as std::ffi::c_int as std::ffi::c_ushort as std::ffi::c_int
            != 0
            || i & 0xff as std::ffi::c_int == ' ' as i32
            || i & 0xff as std::ffi::c_int > 160 as std::ffi::c_int
                && supported & 256 as std::ffi::c_int != 0
            || supported & 128 as std::ffi::c_int != 0 && i & 0xff as std::ffi::c_int != 0)
            && supported & (1 as std::ffi::c_int) << (i >> 8 as std::ffi::c_int) != 0
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
    mi1 = misum / 4 as std::ffi::c_int;
    mi2 = misum / 4 as std::ffi::c_int;
    mi3 = misum / 4 as std::ffi::c_int;
    mi4 = misum / 4 as std::ffi::c_int;
    ma1 = msum / 4 as std::ffi::c_int;
    ma2 = msum / 4 as std::ffi::c_int;
    ma3 = msum / 4 as std::ffi::c_int;
    ma4 = msum / 4 as std::ffi::c_int;
    i = 0 as std::ffi::c_int;
    while i < 256 as std::ffi::c_int * 5 as std::ffi::c_int {
        if !(1 as std::ffi::c_int == 0) {
            values(i, &mut v1, &mut v2, &mut v3, &mut v4);
            sum = ((v1 + v2 + v3 + v4 - misum) as std::ffi::c_double
                * (1020 as std::ffi::c_int as std::ffi::c_double / msum as std::ffi::c_double)
                + 0.5f64) as std::ffi::c_int;
            v1 = ((v1 - mi1) as std::ffi::c_double
                * (255 as std::ffi::c_int as std::ffi::c_double / ma1 as std::ffi::c_double)
                + 0.5f64) as std::ffi::c_int;
            v2 = ((v2 - mi2) as std::ffi::c_double
                * (255 as std::ffi::c_int as std::ffi::c_double / ma2 as std::ffi::c_double)
                + 0.5f64) as std::ffi::c_int;
            v3 = ((v3 - mi3) as std::ffi::c_double
                * (255 as std::ffi::c_int as std::ffi::c_double / ma3 as std::ffi::c_double)
                + 0.5f64) as std::ffi::c_int;
            v4 = ((v4 - mi4) as std::ffi::c_double
                * (255 as std::ffi::c_int as std::ffi::c_double / ma4 as std::ffi::c_double)
                + 0.5f64) as std::ffi::c_int;
            if v1 > 255 as std::ffi::c_int {
                v1 = 255 as std::ffi::c_int;
            }
            if v2 > 255 as std::ffi::c_int {
                v2 = 255 as std::ffi::c_int;
            }
            if v3 > 255 as std::ffi::c_int {
                v3 = 255 as std::ffi::c_int;
            }
            if v4 > 255 as std::ffi::c_int {
                v4 = 255 as std::ffi::c_int;
            }
            if v1 < 0 as std::ffi::c_int {
                v1 = 0 as std::ffi::c_int;
            }
            if v2 < 0 as std::ffi::c_int {
                v2 = 0 as std::ffi::c_int;
            }
            if v3 < 0 as std::ffi::c_int {
                v3 = 0 as std::ffi::c_int;
            }
            if v4 < 0 as std::ffi::c_int {
                v4 = 0 as std::ffi::c_int;
            }
            (*parameters.offset(i as isize)).p[0 as std::ffi::c_int as usize] =
                v1 as std::ffi::c_uint;
            (*parameters.offset(i as isize)).p[1 as std::ffi::c_int as usize] =
                v2 as std::ffi::c_uint;
            (*parameters.offset(i as isize)).p[2 as std::ffi::c_int as usize] =
                v3 as std::ffi::c_uint;
            (*parameters.offset(i as isize)).p[3 as std::ffi::c_int as usize] =
                v4 as std::ffi::c_uint;
            (*parameters.offset(i as isize)).p[4 as std::ffi::c_int as usize] =
                sum as std::ffi::c_uint;
        }
        i += 1;
        i;
    }
}
