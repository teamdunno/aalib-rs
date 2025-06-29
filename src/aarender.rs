use super::aaattributes::*;
use super::aamktabl::aa_mktable;
use super::aastructs::*;

unsafe extern "C" {
    fn calloc(_: std::ffi::c_ulong, _: std::ffi::c_ulong) -> *mut std::ffi::c_void;
    fn free(_: *mut std::ffi::c_void);
    fn abs(_: std::ffi::c_int) -> std::ffi::c_int;
    fn pow(_: std::ffi::c_double, _: std::ffi::c_double) -> std::ffi::c_double;
}
pub type aa_palette = [std::ffi::c_int; 256];

pub static mut aa_defrenderparams: aa_renderparams = {
    let init = aa_renderparams {
        bright: 0,
        contrast: 0,
        gamma: 1.0,
        dither: AA_FLOYD_S as u64,
        inversion: 0,
        randomval: 0,
    };
    init
};

pub static mut aa_dithernames: [*const std::ffi::c_char; 4] = [
    b"no dithering\0" as *const u8 as *const std::ffi::c_char,
    b"error-distribution\0" as *const u8 as *const std::ffi::c_char,
    b"floyd-steelberg dithering\0" as *const u8 as *const std::ffi::c_char,
    0 as *const std::ffi::c_char,
];

pub fn aa_getrenderparams() -> *mut aa_renderparams {
    unsafe {
        let p: *mut aa_renderparams = calloc(
            1 as std::ffi::c_int as std::ffi::c_ulong,
            ::core::mem::size_of::<aa_renderparams>() as std::ffi::c_ulong,
        ) as *mut aa_renderparams;
        if p.is_null() {
            return 0 as *mut aa_renderparams;
        }
        *p = aa_defrenderparams;
        return p;
    }
}

pub fn aa_renderpalette(
    c: *mut aa_context,
    palette: *const i64,
    p: *const aa_renderparams,
    mut x1: i64,
    mut y1: i64,
    mut x2: i64,
    mut y2: i64,
) {
    unsafe {
        static mut state: i64 = 0;
        let mut x = 0;
        let mut y = 0;
        let mut val = 0;
        let wi = (*c).imgwidth;
        let mut pos = 0;
        let mut i = 0;
        let mut pos1 = 0;
        let mut i1: i64 = 0;
        let mut i2: i64 = 0;
        let mut i3: i64 = 0;
        let mut i4: i64 = 0;
        let mut esum: i64 = 0;
        let mut errors: [*mut i64; 2] = [0 as *mut i64; 2];
        let mut cur: i64 = 0;
        let mut mval = 0;
        let mut gamma = ((*p).gamma as f64 != 1.0f64) as i64;
        let randomval = (*p).randomval;
        let mut dither = (*p).dither as std::ffi::c_int;
        let mut table: aa_palette = [0; 256];
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
        if dither == AA_FLOYD_S.try_into().unwrap() {
            errors[0] = calloc(
                1,
                (x2 + 5).wrapping_mul(::core::mem::size_of::<i64>() as i64) as u64,
            ) as *mut i64;
            if (errors[0]).is_null() {
                dither = AA_ERRORDISTRIB.try_into().unwrap();
            }
            errors[0] = (errors[0]).offset(3);
            errors[1] = calloc(
                1,
                (x2 + 5).wrapping_mul(::core::mem::size_of::<i64>() as i64) as u64,
            ) as *mut i64;
            if (errors[1]).is_null() {
                free(errors[0] as *mut std::ffi::c_void);
                dither = AA_ERRORDISTRIB.try_into().unwrap();
            }
            errors[1] = (errors[1]).offset(3);
            cur = 0;
        }
        i = 0;
        while i < 256 {
            y = *palette.offset(i as isize) + (*p).bright;
            if y > 255 {
                y = 255;
            }
            if y < 0 {
                y = 0;
            }
            if (*p).contrast != 0 {
                y = if y < (*p).contrast {
                    0
                } else if y > 256 - (*p).contrast {
                    255
                } else {
                    (y - (*p).contrast) * 255 / (255 - 2 * (*p).contrast)
                };
            }
            if gamma != 0 {
                y = (pow(y as f64 / 255.0, (*p).gamma.into()) as f64 * 255.0 + 0.5) as i64;
            }
            if (*p).inversion != 0 {
                y = 255 - y;
            }
            if y > 255 {
                y = 255;
            } else if y < 0 {
                y = 0;
            }
            table[i as usize] = y as i32;
            i += 1;
            i;
        }
        gamma = 0;
        if randomval != 0 {
            gamma = (randomval / 2) as i64;
        }
        mval = (*((*c).parameters).offset(*((*c).filltable).offset(255) as isize)).p[4];
        y = y1;
        while y < y2 {
            pos = 2 * y * wi;
            pos1 = y * (*c).params.width;
            esum = 0;
            x = x1;
            while x < x2 {
                i1 = table[*((*c).imagebuffer).offset(pos as isize) as usize] as i64;
                i2 = table[*((*c).imagebuffer).offset((pos + 1) as isize) as usize] as i64;
                i3 = table[*((*c).imagebuffer).offset((pos + wi) as isize) as usize] as i64;
                i4 = table[*((*c).imagebuffer).offset((pos + 1 + wi) as isize) as usize] as i64;
                if gamma != 0 {
                    state = (state * 1103515245 + 12345) & 0xffffffff;
                    i = state;
                    i1 += i % randomval - gamma;
                    i2 += (i >> 8) % randomval - gamma;
                    i3 += (i >> 16) % randomval - gamma;
                    i4 += (i >> 24) % randomval - gamma;
                    if (i1 | i2 | i3 | i4) & !255 != 0 {
                        if i1 < 0 {
                            i1 = 0;
                        } else if i1 > 255 {
                            i1 = 255;
                        }
                        if i2 < 0 {
                            i2 = 0;
                        } else if i2 > 255 {
                            i2 = 255;
                        }
                        if i3 < 0 {
                            i3 = 0;
                        } else if i3 > 255 {
                            i3 = 255;
                        }
                        if i4 < 0 {
                            i4 = 0;
                        } else if i4 > 255 {
                            i4 = 255;
                        }
                    }
                }
                match dither {
                    1 => {
                        esum = esum + 2 >> 2;
                        i1 += esum;
                        i2 += esum;
                        i3 += esum;
                        i4 += esum;
                    }
                    2 => {
                        if i1 | i2 | i3 | i4 != 0 {
                            *(errors[cur as usize]).offset((x - 2) as isize) += esum as i64 >> 4;
                            *(errors[cur as usize]).offset((x - 1) as isize) +=
                                5 * esum as i64 >> 4;
                            *(errors[cur as usize]).offset(x as isize) = 3 * esum as i64 >> 4;
                            esum = 7 * esum >> 4;
                            esum += *(errors[(cur ^ 1) as usize]).offset(x as isize);
                            i1 += esum + 1 >> 2;
                            i2 += esum >> 2;
                            i3 += esum + 3 >> 2;
                            i4 += esum + 2 >> 2;
                        }
                    }
                    _ => {}
                }
                if dither != 0 {
                    esum = i1 + i2 + i3 + i4;
                    val = esum >> 2;
                    if abs((i1 - val) as i32) < 13
                        && abs((i2 - val) as i32) < 13
                        && abs((i3 - val) as i32) < 13
                        && abs((i4 - val) as i32) < 13
                    {
                        if esum >= 4 * 256 {
                            val = 255;
                            esum = 4 * 256 - 1;
                        }
                        if val < 0 {
                            val = 0;
                        }
                        val = *((*c).filltable).offset(val as isize) as i64;
                    } else {
                        if (i1 | i2 | i3 | i4) & !255 != 0 {
                            if i1 < 0 {
                                i1 = 0;
                            } else if i1 > 255 {
                                i1 = 255;
                            }
                            if i2 < 0 {
                                i2 = 0;
                            } else if i2 > 255 {
                                i2 = 255;
                            }
                            if i3 < 0 {
                                i3 = 0;
                            } else if i3 > 255 {
                                i3 = 255;
                            }
                            if i4 < 0 {
                                i4 = 0;
                            } else if i4 > 255 {
                                i4 = 255;
                            }
                        }
                        esum = i1 + i2 + i3 + i4;
                        i1 >>= 4;
                        i2 >>= 4;
                        i3 >>= 4;
                        i4 >>= 4;
                        val = *((*c).table)
                            .offset(((i2 << 12) + (i1 << 8) + (i4 << 4) + i3) as isize)
                            as i64;
                    }
                    esum = esum.wrapping_sub(
                        ((*((*c).parameters).offset(val as isize)).p[4])
                            .wrapping_mul(1020)
                            .wrapping_div(mval) as i64,
                    ) as i64;
                } else {
                    val = i1 + i2 + i3 + i4 >> 2;
                    if abs((i1 - val) as i32) < 13
                        && abs((i2 - val) as i32) < 13
                        && abs((i3 - val) as i32) < 13
                        && abs((i4 - val) as i32) < 13
                    {
                        val = *((*c).filltable).offset(val as isize) as i64;
                    } else {
                        i1 >>= 4;
                        i2 >>= 4;
                        i3 >>= 4;
                        i4 >>= 4;
                        val = *((*c).table)
                            .offset(((i2 << 12) + (i1 << 8) + (i4 << 4) + i3) as isize)
                            as i64;
                    }
                }
                *((*c).attrbuffer).offset(pos1 as isize) = (val >> 8) as std::ffi::c_uchar;
                *((*c).textbuffer).offset(pos1 as isize) = (val & 0xff) as std::ffi::c_uchar;
                pos += 2;
                pos1 += 1;
                x += 1;
            }
            if dither == AA_FLOYD_S.try_into().unwrap() {
                if x2 - 1 > x1 {
                    *(errors[cur as usize]).offset((x2 - 2) as isize) += (esum >> 4) as i64;
                }
                if x2 > x1 {
                    *(errors[cur as usize]).offset((x2 - 1) as isize) += (5 * esum >> 4) as i64;
                }
                cur ^= 1;
                *(errors[cur as usize]).offset(x1 as isize) = 0;
                *(errors[(cur ^ 1) as usize]).offset(-(1 as std::ffi::c_int) as isize) = 0;
            }
            y += 1;
            y;
        }
        if dither == AA_FLOYD_S as std::ffi::c_int {
            free(
                (errors[0 as std::ffi::c_int as usize]).offset(-(3 as std::ffi::c_int as isize))
                    as *mut std::ffi::c_void,
            );
            free(
                (errors[1 as std::ffi::c_int as usize]).offset(-(3 as std::ffi::c_int as isize))
                    as *mut std::ffi::c_void,
            );
        }
    }
}

pub fn aa_render(
    c: *mut aa_context,
    p: *const aa_renderparams,
    x1: i64,
    y1: i64,
    x2: i64,
    y2: i64,
) {
    unsafe {
        let mut i: std::ffi::c_int = 0;
        static mut table: aa_palette = [0; 256];
        if table[255 as std::ffi::c_int as usize] != 255 as std::ffi::c_int {
            i = 0 as std::ffi::c_int;
            while i < 256 as std::ffi::c_int {
                table[i as usize] = i;
                i += 1;
                i;
            }
        }
        aa_renderpalette(
            c,
            table.as_mut_ptr() as *const i64,
            p,
            x1.try_into().unwrap(),
            y1.try_into().unwrap(),
            x2.try_into().unwrap(),
            y2.try_into().unwrap(),
        );
    }
}
