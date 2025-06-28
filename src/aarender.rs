use super::aamktabl::aa_mktable;
use super::aastructs::*;

unsafe extern "C" {
    fn calloc(_: std::ffi::c_ulong, _: std::ffi::c_ulong) -> *mut std::ffi::c_void;
    fn free(_: *mut std::ffi::c_void);
    fn abs(_: std::ffi::c_int) -> std::ffi::c_int;
    fn pow(_: std::ffi::c_double, _: std::ffi::c_double) -> std::ffi::c_double;
}
pub type aa_dithering_mode = std::ffi::c_uint;
pub const AA_DITHERTYPES: aa_dithering_mode = 3;
pub const AA_FLOYD_S: aa_dithering_mode = 2;
pub const AA_ERRORDISTRIB: aa_dithering_mode = 1;
pub const AA_NONE: aa_dithering_mode = 0;
pub type aa_palette = [std::ffi::c_int; 256];

pub static mut aa_defrenderparams: aa_renderparams = {
    let mut init = aa_renderparams {
        bright: 0 as std::ffi::c_int,
        contrast: 0 as std::ffi::c_int,
        gamma: 1.0f64 as std::ffi::c_float,
        dither: AA_FLOYD_S,
        inversion: 0 as std::ffi::c_int,
        randomval: 0 as std::ffi::c_int,
    };
    init
};

pub static mut aa_dithernames: [*const std::ffi::c_char; 4] = [
    b"no dithering\0" as *const u8 as *const std::ffi::c_char,
    b"error-distribution\0" as *const u8 as *const std::ffi::c_char,
    b"floyd-steelberg dithering\0" as *const u8 as *const std::ffi::c_char,
    0 as *const std::ffi::c_char,
];

pub unsafe extern "C" fn aa_getrenderparams() -> *mut aa_renderparams {
    let mut p: *mut aa_renderparams = calloc(
        1 as std::ffi::c_int as std::ffi::c_ulong,
        ::core::mem::size_of::<aa_renderparams>() as std::ffi::c_ulong,
    ) as *mut aa_renderparams;
    if p.is_null() {
        return 0 as *mut aa_renderparams;
    }
    *p = aa_defrenderparams;
    return p;
}

pub unsafe extern "C" fn aa_renderpalette(
    mut c: *mut aa_context,
    mut palette: *const std::ffi::c_int,
    mut p: *const aa_renderparams,
    mut x1: std::ffi::c_int,
    mut y1: std::ffi::c_int,
    mut x2: std::ffi::c_int,
    mut y2: std::ffi::c_int,
) {
    static mut state: std::ffi::c_int = 0;
    let mut x: std::ffi::c_int = 0;
    let mut y: std::ffi::c_int = 0;
    let mut val: std::ffi::c_int = 0;
    let mut wi: std::ffi::c_int = (*c).imgwidth;
    let mut pos: std::ffi::c_int = 0;
    let mut i: std::ffi::c_int = 0;
    let mut pos1: std::ffi::c_int = 0;
    let mut i1: std::ffi::c_int = 0;
    let mut i2: std::ffi::c_int = 0;
    let mut i3: std::ffi::c_int = 0;
    let mut i4: std::ffi::c_int = 0;
    let mut esum: std::ffi::c_int = 0;
    let mut errors: [*mut std::ffi::c_int; 2] = [0 as *mut std::ffi::c_int; 2];
    let mut cur: std::ffi::c_int = 0 as std::ffi::c_int;
    let mut mval: std::ffi::c_int = 0;
    let mut gamma: std::ffi::c_int =
        ((*p).gamma as std::ffi::c_double != 1.0f64) as std::ffi::c_int;
    let mut randomval: std::ffi::c_int = (*p).randomval;
    let mut dither: std::ffi::c_int = (*p).dither as std::ffi::c_int;
    let mut table: aa_palette = [0; 256];
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
    if dither == AA_FLOYD_S as std::ffi::c_int {
        errors[0 as std::ffi::c_int as usize] = calloc(
            1 as std::ffi::c_int as std::ffi::c_ulong,
            ((x2 + 5 as std::ffi::c_int) as std::ffi::c_ulong)
                .wrapping_mul(::core::mem::size_of::<std::ffi::c_int>() as std::ffi::c_ulong),
        ) as *mut std::ffi::c_int;
        if (errors[0 as std::ffi::c_int as usize]).is_null() {
            dither = AA_ERRORDISTRIB as std::ffi::c_int;
        }
        errors[0 as std::ffi::c_int as usize] =
            (errors[0 as std::ffi::c_int as usize]).offset(3 as std::ffi::c_int as isize);
        errors[1 as std::ffi::c_int as usize] = calloc(
            1 as std::ffi::c_int as std::ffi::c_ulong,
            ((x2 + 5 as std::ffi::c_int) as std::ffi::c_ulong)
                .wrapping_mul(::core::mem::size_of::<std::ffi::c_int>() as std::ffi::c_ulong),
        ) as *mut std::ffi::c_int;
        if (errors[1 as std::ffi::c_int as usize]).is_null() {
            free(errors[0 as std::ffi::c_int as usize] as *mut std::ffi::c_void);
            dither = AA_ERRORDISTRIB as std::ffi::c_int;
        }
        errors[1 as std::ffi::c_int as usize] =
            (errors[1 as std::ffi::c_int as usize]).offset(3 as std::ffi::c_int as isize);
        cur = 0 as std::ffi::c_int;
    }
    i = 0 as std::ffi::c_int;
    while i < 256 as std::ffi::c_int {
        y = *palette.offset(i as isize) + (*p).bright;
        if y > 255 as std::ffi::c_int {
            y = 255 as std::ffi::c_int;
        }
        if y < 0 as std::ffi::c_int {
            y = 0 as std::ffi::c_int;
        }
        if (*p).contrast != 0 {
            y = if y < (*p).contrast {
                0 as std::ffi::c_int
            } else if y > 256 as std::ffi::c_int - (*p).contrast {
                255 as std::ffi::c_int
            } else {
                (y - (*p).contrast) * 255 as std::ffi::c_int
                    / (255 as std::ffi::c_int - 2 as std::ffi::c_int * (*p).contrast)
            };
        }
        if gamma != 0 {
            y = (pow(
                y as std::ffi::c_double / 255.0f64,
                (*p).gamma as std::ffi::c_double,
            ) * 255 as std::ffi::c_int as std::ffi::c_double
                + 0.5f64) as std::ffi::c_int;
        }
        if (*p).inversion != 0 {
            y = 255 as std::ffi::c_int - y;
        }
        if y > 255 as std::ffi::c_int {
            y = 255 as std::ffi::c_int;
        } else if y < 0 as std::ffi::c_int {
            y = 0 as std::ffi::c_int;
        }
        table[i as usize] = y;
        i += 1;
        i;
    }
    gamma = 0;
    if randomval != 0 {
        gamma = randomval / 2 as std::ffi::c_int;
    }
    mval = (*((*c).parameters)
        .offset(*((*c).filltable).offset(255 as std::ffi::c_int as isize) as isize))
    .p[4 as std::ffi::c_int as usize] as std::ffi::c_int;
    y = y1;
    while y < y2 {
        pos = 2 as std::ffi::c_int * y * wi;
        pos1 = y * (*c).params.width;
        esum = 0 as std::ffi::c_int;
        x = x1;
        while x < x2 {
            i1 = table[*((*c).imagebuffer).offset(pos as isize) as std::ffi::c_int as usize];
            i2 = table[*((*c).imagebuffer).offset((pos + 1 as std::ffi::c_int) as isize)
                as std::ffi::c_int as usize];
            i3 = table[*((*c).imagebuffer).offset((pos + wi) as isize) as std::ffi::c_int as usize];
            i4 = table[*((*c).imagebuffer).offset((pos + 1 as std::ffi::c_int + wi) as isize)
                as std::ffi::c_int as usize];
            if gamma != 0 {
                state = ((state * 1103515245 as std::ffi::c_int + 12345 as std::ffi::c_int)
                    as std::ffi::c_uint
                    & 0xffffffff as std::ffi::c_uint) as std::ffi::c_int;
                i = state;
                i1 += i % randomval - gamma;
                i2 += (i >> 8 as std::ffi::c_int) % randomval - gamma;
                i3 += (i >> 16 as std::ffi::c_int) % randomval - gamma;
                i4 += (i >> 24 as std::ffi::c_int) % randomval - gamma;
                if (i1 | i2 | i3 | i4) & !(255 as std::ffi::c_int) != 0 {
                    if i1 < 0 as std::ffi::c_int {
                        i1 = 0 as std::ffi::c_int;
                    } else if i1 > 255 as std::ffi::c_int {
                        i1 = 255 as std::ffi::c_int;
                    }
                    if i2 < 0 as std::ffi::c_int {
                        i2 = 0 as std::ffi::c_int;
                    } else if i2 > 255 as std::ffi::c_int {
                        i2 = 255 as std::ffi::c_int;
                    }
                    if i3 < 0 as std::ffi::c_int {
                        i3 = 0 as std::ffi::c_int;
                    } else if i3 > 255 as std::ffi::c_int {
                        i3 = 255 as std::ffi::c_int;
                    }
                    if i4 < 0 as std::ffi::c_int {
                        i4 = 0 as std::ffi::c_int;
                    } else if i4 > 255 as std::ffi::c_int {
                        i4 = 255 as std::ffi::c_int;
                    }
                }
            }
            match dither {
                1 => {
                    esum = esum + 2 as std::ffi::c_int >> 2 as std::ffi::c_int;
                    i1 += esum;
                    i2 += esum;
                    i3 += esum;
                    i4 += esum;
                }
                2 => {
                    if i1 | i2 | i3 | i4 != 0 {
                        *(errors[cur as usize]).offset((x - 2 as std::ffi::c_int) as isize) +=
                            esum >> 4 as std::ffi::c_int;
                        *(errors[cur as usize]).offset((x - 1 as std::ffi::c_int) as isize) +=
                            5 as std::ffi::c_int * esum >> 4 as std::ffi::c_int;
                        *(errors[cur as usize]).offset(x as isize) =
                            3 as std::ffi::c_int * esum >> 4 as std::ffi::c_int;
                        esum = 7 as std::ffi::c_int * esum >> 4 as std::ffi::c_int;
                        esum += *(errors[(cur ^ 1 as std::ffi::c_int) as usize]).offset(x as isize);
                        i1 += esum + 1 as std::ffi::c_int >> 2 as std::ffi::c_int;
                        i2 += esum >> 2 as std::ffi::c_int;
                        i3 += esum + 3 as std::ffi::c_int >> 2 as std::ffi::c_int;
                        i4 += esum + 2 as std::ffi::c_int >> 2 as std::ffi::c_int;
                    }
                }
                _ => {}
            }
            if dither != 0 {
                esum = i1 + i2 + i3 + i4;
                val = esum >> 2 as std::ffi::c_int;
                if abs(i1 - val) < 13 as std::ffi::c_int
                    && abs(i2 - val) < 13 as std::ffi::c_int
                    && abs(i3 - val) < 13 as std::ffi::c_int
                    && abs(i4 - val) < 13 as std::ffi::c_int
                {
                    if esum >= 4 as std::ffi::c_int * 256 as std::ffi::c_int {
                        val = 255 as std::ffi::c_int;
                        esum = 4 as std::ffi::c_int * 256 as std::ffi::c_int - 1 as std::ffi::c_int;
                    }
                    if val < 0 as std::ffi::c_int {
                        val = 0 as std::ffi::c_int;
                    }
                    val = *((*c).filltable).offset(val as isize) as std::ffi::c_int;
                } else {
                    if (i1 | i2 | i3 | i4) & !(255 as std::ffi::c_int) != 0 {
                        if i1 < 0 as std::ffi::c_int {
                            i1 = 0 as std::ffi::c_int;
                        } else if i1 > 255 as std::ffi::c_int {
                            i1 = 255 as std::ffi::c_int;
                        }
                        if i2 < 0 as std::ffi::c_int {
                            i2 = 0 as std::ffi::c_int;
                        } else if i2 > 255 as std::ffi::c_int {
                            i2 = 255 as std::ffi::c_int;
                        }
                        if i3 < 0 as std::ffi::c_int {
                            i3 = 0 as std::ffi::c_int;
                        } else if i3 > 255 as std::ffi::c_int {
                            i3 = 255 as std::ffi::c_int;
                        }
                        if i4 < 0 as std::ffi::c_int {
                            i4 = 0 as std::ffi::c_int;
                        } else if i4 > 255 as std::ffi::c_int {
                            i4 = 255 as std::ffi::c_int;
                        }
                    }
                    esum = i1 + i2 + i3 + i4;
                    i1 >>= 4 as std::ffi::c_int;
                    i2 >>= 4 as std::ffi::c_int;
                    i3 >>= 4 as std::ffi::c_int;
                    i4 >>= 4 as std::ffi::c_int;
                    val = *((*c).table).offset(
                        ((i2 << 12 as std::ffi::c_int)
                            + (i1 << 8 as std::ffi::c_int)
                            + (i4 << 4 as std::ffi::c_int)
                            + i3) as isize,
                    ) as std::ffi::c_int;
                }
                esum = (esum as std::ffi::c_uint).wrapping_sub(
                    ((*((*c).parameters).offset(val as isize)).p[4 as std::ffi::c_int as usize])
                        .wrapping_mul(1020 as std::ffi::c_int as std::ffi::c_uint)
                        .wrapping_div(mval as std::ffi::c_uint),
                ) as std::ffi::c_int;
            } else {
                val = i1 + i2 + i3 + i4 >> 2 as std::ffi::c_int;
                if abs(i1 - val) < 13 as std::ffi::c_int
                    && abs(i2 - val) < 13 as std::ffi::c_int
                    && abs(i3 - val) < 13 as std::ffi::c_int
                    && abs(i4 - val) < 13 as std::ffi::c_int
                {
                    val = *((*c).filltable).offset(val as isize) as std::ffi::c_int;
                } else {
                    i1 >>= 4 as std::ffi::c_int;
                    i2 >>= 4 as std::ffi::c_int;
                    i3 >>= 4 as std::ffi::c_int;
                    i4 >>= 4 as std::ffi::c_int;
                    val = *((*c).table).offset(
                        ((i2 << 12 as std::ffi::c_int)
                            + (i1 << 8 as std::ffi::c_int)
                            + (i4 << 4 as std::ffi::c_int)
                            + i3) as isize,
                    ) as std::ffi::c_int;
                }
            }
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
        if dither == AA_FLOYD_S as std::ffi::c_int {
            if x2 - 1 as std::ffi::c_int > x1 {
                *(errors[cur as usize]).offset((x2 - 2 as std::ffi::c_int) as isize) +=
                    esum >> 4 as std::ffi::c_int;
            }
            if x2 > x1 {
                *(errors[cur as usize]).offset((x2 - 1 as std::ffi::c_int) as isize) +=
                    5 as std::ffi::c_int * esum >> 4 as std::ffi::c_int;
            }
            cur ^= 1 as std::ffi::c_int;
            *(errors[cur as usize]).offset(x1 as isize) = 0;
            *(errors[(cur ^ 1 as std::ffi::c_int) as usize])
                .offset(-(1 as std::ffi::c_int) as isize) = 0;
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

pub unsafe extern "C" fn aa_render(
    mut c: *mut aa_context,
    mut p: *const aa_renderparams,
    mut x1: std::ffi::c_int,
    mut y1: std::ffi::c_int,
    mut x2: std::ffi::c_int,
    mut y2: std::ffi::c_int,
) {
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
        table.as_mut_ptr() as *const std::ffi::c_int,
        p,
        x1,
        y1,
        x2,
        y2,
    );
}
