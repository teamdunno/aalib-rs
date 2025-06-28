use super::aafont::__aa_calcparams;
use super::aain::{aa_context, aa_font, parameters};

unsafe extern "C" {
    fn __ctype_b_loc() -> *mut *const std::ffi::c_ushort;
    fn malloc(_: std::ffi::c_ulong) -> *mut std::ffi::c_void;
    fn calloc(_: std::ffi::c_ulong, _: std::ffi::c_ulong) -> *mut std::ffi::c_void;
    fn free(_: *mut std::ffi::c_void);
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aa_driver {
    pub shortname: *const std::ffi::c_char,
    pub name: *const std::ffi::c_char,
    pub init: Option<
        unsafe extern "C" fn(
            *const aa_hardware_params,
            *const std::ffi::c_void,
            *mut aa_hardware_params,
            *mut *mut std::ffi::c_void,
        ) -> std::ffi::c_int,
    >,
    pub uninit: Option<unsafe extern "C" fn(*mut aa_context) -> ()>,
    pub getsize: Option<
        unsafe extern "C" fn(*mut aa_context, *mut std::ffi::c_int, *mut std::ffi::c_int) -> (),
    >,
    pub setattr: Option<unsafe extern "C" fn(*mut aa_context, std::ffi::c_int) -> ()>,
    pub print: Option<unsafe extern "C" fn(*mut aa_context, *const std::ffi::c_char) -> ()>,
    pub gotoxy:
        Option<unsafe extern "C" fn(*mut aa_context, std::ffi::c_int, std::ffi::c_int) -> ()>,
    pub flush: Option<unsafe extern "C" fn(*mut aa_context) -> ()>,
    pub cursormode: Option<unsafe extern "C" fn(*mut aa_context, std::ffi::c_int) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aa_hardware_params {
    pub font: *const aa_font,
    pub supported: std::ffi::c_int,
    pub minwidth: std::ffi::c_int,
    pub minheight: std::ffi::c_int,
    pub maxwidth: std::ffi::c_int,
    pub maxheight: std::ffi::c_int,
    pub recwidth: std::ffi::c_int,
    pub recheight: std::ffi::c_int,
    pub mmwidth: std::ffi::c_int,
    pub mmheight: std::ffi::c_int,
    pub width: std::ffi::c_int,
    pub height: std::ffi::c_int,
    pub dimmul: std::ffi::c_double,
    pub boldmul: std::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aa_mousedriver {
    pub shortname: *const std::ffi::c_char,
    pub name: *const std::ffi::c_char,
    pub flags: std::ffi::c_int,
    pub init: Option<unsafe extern "C" fn(*mut aa_context, std::ffi::c_int) -> std::ffi::c_int>,
    pub uninit: Option<unsafe extern "C" fn(*mut aa_context) -> ()>,
    pub getmouse: Option<
        unsafe extern "C" fn(
            *mut aa_context,
            *mut std::ffi::c_int,
            *mut std::ffi::c_int,
            *mut std::ffi::c_int,
        ) -> (),
    >,
    pub cursormode: Option<unsafe extern "C" fn(*mut aa_context, std::ffi::c_int) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aa_kbddriver {
    pub shortname: *const std::ffi::c_char,
    pub name: *const std::ffi::c_char,
    pub flags: std::ffi::c_int,
    pub init: Option<unsafe extern "C" fn(*mut aa_context, std::ffi::c_int) -> std::ffi::c_int>,
    pub uninit: Option<unsafe extern "C" fn(*mut aa_context) -> ()>,
    pub getkey: Option<unsafe extern "C" fn(*mut aa_context, std::ffi::c_int) -> std::ffi::c_int>,
}
static mut priority: [std::ffi::c_int; 5] = [
    4 as std::ffi::c_int,
    5 as std::ffi::c_int,
    3 as std::ffi::c_int,
    2 as std::ffi::c_int,
    1 as std::ffi::c_int,
];

pub unsafe extern "C" fn aa_mktable(mut c: *mut aa_context) -> *mut std::ffi::c_ushort {
    let mut i: std::ffi::c_int = 0;
    let mut i1: std::ffi::c_int = 0;
    let mut i2: std::ffi::c_int = 0;
    let mut i3: std::ffi::c_int = 0;
    let mut i4: std::ffi::c_int = 0;
    let mut sum: std::ffi::c_int = 0;
    let mut pos: std::ffi::c_int = 0;
    let mut currfont: *const aa_font = (*c).params.font;
    let mut supported: std::ffi::c_int = (*c).params.supported;
    let mut next: *mut std::ffi::c_ushort = 0 as *mut std::ffi::c_ushort;
    let mut first: std::ffi::c_int = -(1 as std::ffi::c_int);
    let mut last: std::ffi::c_int = -(1 as std::ffi::c_int);
    let mut table: *mut std::ffi::c_ushort = 0 as *mut std::ffi::c_ushort;
    let mut filltable: *mut std::ffi::c_ushort = 0 as *mut std::ffi::c_ushort;
    static mut parameters: *mut parameters = 0 as *const parameters as *mut parameters;
    next = malloc(
        (::core::mem::size_of::<std::ffi::c_ushort>() as std::ffi::c_ulong)
            .wrapping_mul(65536 as std::ffi::c_int as std::ffi::c_ulong),
    ) as *mut std::ffi::c_ushort;
    parameters = calloc(
        1 as std::ffi::c_int as std::ffi::c_ulong,
        (::core::mem::size_of::<parameters>() as std::ffi::c_ulong).wrapping_mul(
            (256 as std::ffi::c_int * 5 as std::ffi::c_int + 1 as std::ffi::c_int)
                as std::ffi::c_ulong,
        ),
    ) as *mut parameters;
    table = calloc(
        1 as std::ffi::c_int as std::ffi::c_ulong,
        (65536 as std::ffi::c_int as std::ffi::c_ulong)
            .wrapping_mul(::core::mem::size_of::<std::ffi::c_ushort>() as std::ffi::c_ulong),
    ) as *mut std::ffi::c_ushort;
    filltable = calloc(
        1 as std::ffi::c_int as std::ffi::c_ulong,
        (256 as std::ffi::c_int as std::ffi::c_ulong)
            .wrapping_mul(::core::mem::size_of::<std::ffi::c_ushort>() as std::ffi::c_ulong),
    ) as *mut std::ffi::c_ushort;
    first = -(1 as std::ffi::c_int);
    last = -(1 as std::ffi::c_int);
    i = 0 as std::ffi::c_int;
    while i < 65536 as std::ffi::c_int {
        *next.offset(i as isize) = i as std::ffi::c_ushort;
        *table.offset(i as isize) = 0 as std::ffi::c_int as std::ffi::c_ushort;
        i += 1;
        i;
    }
    __aa_calcparams(
        currfont,
        parameters,
        supported,
        (*c).params.dimmul,
        (*c).params.boldmul,
    );
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
            let mut current_block_33: u64;
            let mut p1: std::ffi::c_int = 0;
            let mut p2: std::ffi::c_int = 0;
            let mut p3: std::ffi::c_int = 0;
            let mut p4: std::ffi::c_int = 0;
            i1 = (*parameters.offset(i as isize)).p[0 as std::ffi::c_int as usize]
                as std::ffi::c_int;
            i2 = (*parameters.offset(i as isize)).p[1 as std::ffi::c_int as usize]
                as std::ffi::c_int;
            i3 = (*parameters.offset(i as isize)).p[2 as std::ffi::c_int as usize]
                as std::ffi::c_int;
            i4 = (*parameters.offset(i as isize)).p[3 as std::ffi::c_int as usize]
                as std::ffi::c_int;
            p1 = i1 >> 4 as std::ffi::c_int;
            p2 = i2 >> 4 as std::ffi::c_int;
            p3 = i3 >> 4 as std::ffi::c_int;
            p4 = i4 >> 4 as std::ffi::c_int;
            sum = (*parameters.offset(i as isize)).p[4 as std::ffi::c_int as usize]
                as std::ffi::c_int;
            pos = (p1 << 12 as std::ffi::c_int)
                + (p2 << 8 as std::ffi::c_int)
                + (p3 << 4 as std::ffi::c_int)
                + p4;
            if *table.offset(pos as isize) != 0 {
                let mut sum_0: std::ffi::c_int = 0;
                p1 = (p1 << 4 as std::ffi::c_int) + p1;
                p2 = (p2 << 4 as std::ffi::c_int) + p2;
                p3 = (p3 << 4 as std::ffi::c_int) + p3;
                p4 = (p4 << 4 as std::ffi::c_int) + p4;
                sum_0 = p1 + p2 + p3 + p4;
                p1 = 2 as std::ffi::c_int
                    * (((*parameters.offset(i as isize)).p[0 as std::ffi::c_int as usize]
                        as std::ffi::c_int
                        - p1)
                        * ((*parameters.offset(i as isize)).p[0 as std::ffi::c_int as usize]
                            as std::ffi::c_int
                            - p1)
                        + ((*parameters.offset(i as isize)).p[1 as std::ffi::c_int as usize]
                            as std::ffi::c_int
                            - p2)
                            * ((*parameters.offset(i as isize)).p[1 as std::ffi::c_int as usize]
                                as std::ffi::c_int
                                - p2)
                        + (((*parameters.offset(i as isize)).p[2 as std::ffi::c_int as usize]
                            as std::ffi::c_int
                            - p3)
                            * ((*parameters.offset(i as isize)).p[2 as std::ffi::c_int as usize]
                                as std::ffi::c_int
                                - p3)
                            + ((*parameters.offset(i as isize)).p[3 as std::ffi::c_int as usize]
                                as std::ffi::c_int
                                - p4)
                                * ((*parameters.offset(i as isize)).p[3 as std::ffi::c_int as usize]
                                    as std::ffi::c_int
                                    - p4)))
                    + 1 as std::ffi::c_int
                        * (((*parameters.offset(i as isize)).p[4 as std::ffi::c_int as usize]
                            as std::ffi::c_int
                            - sum_0)
                            * ((*parameters.offset(i as isize)).p[4 as std::ffi::c_int as usize]
                                as std::ffi::c_int
                                - sum_0));
                p1 = 2 as std::ffi::c_int
                    * (((*parameters.offset(*table.offset(pos as isize) as isize)).p
                        [0 as std::ffi::c_int as usize]
                        as std::ffi::c_int
                        - p1)
                        * ((*parameters.offset(*table.offset(pos as isize) as isize)).p
                            [0 as std::ffi::c_int as usize]
                            as std::ffi::c_int
                            - p1)
                        + ((*parameters.offset(*table.offset(pos as isize) as isize)).p
                            [1 as std::ffi::c_int as usize]
                            as std::ffi::c_int
                            - p2)
                            * ((*parameters.offset(*table.offset(pos as isize) as isize)).p
                                [1 as std::ffi::c_int as usize]
                                as std::ffi::c_int
                                - p2)
                        + (((*parameters.offset(*table.offset(pos as isize) as isize)).p
                            [2 as std::ffi::c_int as usize]
                            as std::ffi::c_int
                            - p3)
                            * ((*parameters.offset(*table.offset(pos as isize) as isize)).p
                                [2 as std::ffi::c_int as usize]
                                as std::ffi::c_int
                                - p3)
                            + ((*parameters.offset(*table.offset(pos as isize) as isize)).p
                                [3 as std::ffi::c_int as usize]
                                as std::ffi::c_int
                                - p4)
                                * ((*parameters.offset(*table.offset(pos as isize) as isize)).p
                                    [3 as std::ffi::c_int as usize]
                                    as std::ffi::c_int
                                    - p4)))
                    + 1 as std::ffi::c_int
                        * (((*parameters.offset(*table.offset(pos as isize) as isize)).p
                            [4 as std::ffi::c_int as usize]
                            as std::ffi::c_int
                            - sum_0)
                            * ((*parameters.offset(*table.offset(pos as isize) as isize)).p
                                [4 as std::ffi::c_int as usize]
                                as std::ffi::c_int
                                - sum_0));
                if p1 >= p1
                    && (p1 != p2
                        || priority[(i / 256 as std::ffi::c_int) as usize]
                            <= priority[(*table.offset(pos as isize) / 256) as usize])
                {
                    current_block_33 = 3275366147856559585;
                } else {
                    current_block_33 = 18386322304582297246;
                }
            } else {
                current_block_33 = 18386322304582297246;
            }
            match current_block_33 {
                18386322304582297246 => {
                    *table.offset(pos as isize) = i as std::ffi::c_ushort;
                    if *next.offset(pos as isize) as std::ffi::c_int == pos && last != pos {
                        if last != -(1 as std::ffi::c_int) {
                            *next.offset(last as isize) = pos as std::ffi::c_ushort;
                            last = pos;
                        } else {
                            first = pos;
                            last = first;
                        }
                    }
                }
                _ => {}
            }
        }
        i += 1;
        i;
    }
    pos = 0 as std::ffi::c_int;
    while pos < 256 as std::ffi::c_int {
        let mut mindist: std::ffi::c_int = 2147483647 as std::ffi::c_int;
        let mut d1: std::ffi::c_int = 0;
        i = 0 as std::ffi::c_int;
        while i < 256 as std::ffi::c_int * 5 as std::ffi::c_int {
            if (*(*__ctype_b_loc()).offset((i & 0xff as std::ffi::c_int) as isize)
                as std::ffi::c_int
                & _ISgraph as std::ffi::c_int as std::ffi::c_ushort as std::ffi::c_int
                != 0
                || i & 0xff as std::ffi::c_int == ' ' as i32
                || i & 0xff as std::ffi::c_int > 160 as std::ffi::c_int
                    && supported & 256 as std::ffi::c_int != 0
                || supported & 128 as std::ffi::c_int != 0 && i & 0xff as std::ffi::c_int != 0)
                && supported & (1 as std::ffi::c_int) << (i >> 8 as std::ffi::c_int) != 0
            {
                d1 = ((*parameters.offset(i as isize)).p[0 as std::ffi::c_int as usize]
                    as std::ffi::c_int
                    - pos)
                    * ((*parameters.offset(i as isize)).p[0 as std::ffi::c_int as usize]
                        as std::ffi::c_int
                        - pos)
                    + ((*parameters.offset(i as isize)).p[1 as std::ffi::c_int as usize]
                        as std::ffi::c_int
                        - pos)
                        * ((*parameters.offset(i as isize)).p[1 as std::ffi::c_int as usize]
                            as std::ffi::c_int
                            - pos)
                    + (((*parameters.offset(i as isize)).p[2 as std::ffi::c_int as usize]
                        as std::ffi::c_int
                        - pos)
                        * ((*parameters.offset(i as isize)).p[2 as std::ffi::c_int as usize]
                            as std::ffi::c_int
                            - pos)
                        + ((*parameters.offset(i as isize)).p[3 as std::ffi::c_int as usize]
                            as std::ffi::c_int
                            - pos)
                            * ((*parameters.offset(i as isize)).p[3 as std::ffi::c_int as usize]
                                as std::ffi::c_int
                                - pos))
                    + 2 as std::ffi::c_int
                        * (((*parameters.offset(i as isize)).p[4 as std::ffi::c_int as usize]
                            as std::ffi::c_int
                            - pos * 4 as std::ffi::c_int)
                            * ((*parameters.offset(i as isize)).p[4 as std::ffi::c_int as usize]
                                as std::ffi::c_int
                                - pos * 4 as std::ffi::c_int));
                if d1 <= mindist
                    && (d1 != mindist
                        || priority[(i / 256 as std::ffi::c_int) as usize]
                            > priority[(*filltable.offset(pos as isize) / 256) as usize])
                {
                    *filltable.offset(pos as isize) = i as std::ffi::c_ushort;
                    mindist = d1;
                }
            }
            i += 1;
            i;
        }
        pos += 1;
        pos;
    }
    loop {
        let mut blocked: std::ffi::c_int = 0;
        if !(last != -(1 as std::ffi::c_int)) {
            break;
        }
        *next.offset(last as isize) = last as std::ffi::c_ushort;
        blocked = last;
        i = first;
        if i == -(1 as std::ffi::c_int) {
            break;
        }
        last = -(1 as std::ffi::c_int);
        first = last;
        loop {
            let mut m1: std::ffi::c_int = 0;
            let mut m2: std::ffi::c_int = 0;
            let mut m3: std::ffi::c_int = 0;
            let mut m4: std::ffi::c_int = 0;
            let mut ii: std::ffi::c_int = 0;
            let mut dm: std::ffi::c_int = 0;
            let mut c_0: std::ffi::c_ushort = *table.offset(i as isize);
            m1 = i >> 12 as std::ffi::c_int;
            m2 = i >> 8 as std::ffi::c_int & 15 as std::ffi::c_int;
            m3 = i >> 4 as std::ffi::c_int & 15 as std::ffi::c_int;
            m4 = i & 15 as std::ffi::c_int;
            dm = 0 as std::ffi::c_int;
            while dm < 4 as std::ffi::c_int {
                let mut current_block_77: u64;
                ii = -(1 as std::ffi::c_int);
                while ii <= 1 as std::ffi::c_int {
                    let mut dist: std::ffi::c_int = 0;
                    let mut dist1: std::ffi::c_int = 0;
                    let mut index: std::ffi::c_int = 0;
                    let mut ch: std::ffi::c_ushort = 0;
                    i1 = m1;
                    i2 = m2;
                    i3 = m3;
                    i4 = m4;
                    match dm {
                        0 => {
                            i1 += ii;
                            if i1 < 0 as std::ffi::c_int || i1 >= 16 as std::ffi::c_int {
                                current_block_77 = 10930818133215224067;
                            } else {
                                current_block_77 = 2606304779496145856;
                            }
                        }
                        1 => {
                            i2 += ii;
                            if i2 < 0 as std::ffi::c_int || i2 >= 16 as std::ffi::c_int {
                                current_block_77 = 10930818133215224067;
                            } else {
                                current_block_77 = 2606304779496145856;
                            }
                        }
                        2 => {
                            i3 += ii;
                            if i3 < 0 as std::ffi::c_int || i3 >= 16 as std::ffi::c_int {
                                current_block_77 = 10930818133215224067;
                            } else {
                                current_block_77 = 2606304779496145856;
                            }
                        }
                        3 => {
                            i4 += ii;
                            if i4 < 0 as std::ffi::c_int || i4 >= 16 as std::ffi::c_int {
                                current_block_77 = 10930818133215224067;
                            } else {
                                current_block_77 = 2606304779496145856;
                            }
                        }
                        _ => {
                            current_block_77 = 2606304779496145856;
                        }
                    }
                    match current_block_77 {
                        2606304779496145856 => {
                            index = (i1 << 12 as std::ffi::c_int)
                                + (i2 << 8 as std::ffi::c_int)
                                + (i3 << 4 as std::ffi::c_int)
                                + i4;
                            ch = *table.offset(index as isize);
                            if !(ch as std::ffi::c_int == c_0 as std::ffi::c_int
                                || index == blocked)
                            {
                                if ch != 0 {
                                    let mut ii1: std::ffi::c_int =
                                        (i1 << 4 as std::ffi::c_int) + i1;
                                    let mut ii2: std::ffi::c_int =
                                        (i2 << 4 as std::ffi::c_int) + i2;
                                    let mut ii3: std::ffi::c_int =
                                        (i3 << 4 as std::ffi::c_int) + i3;
                                    let mut ii4: std::ffi::c_int =
                                        (i4 << 4 as std::ffi::c_int) + i4;
                                    let mut iisum: std::ffi::c_int = ii1 + ii2 + ii3 + ii4;
                                    dist = 2 as std::ffi::c_int
                                        * ((ii1
                                            - (*parameters.offset(c_0 as isize)).p
                                                [0 as std::ffi::c_int as usize]
                                                as std::ffi::c_int)
                                            * (ii1
                                                - (*parameters.offset(c_0 as isize)).p
                                                    [0 as std::ffi::c_int as usize]
                                                    as std::ffi::c_int)
                                            + (ii2
                                                - (*parameters.offset(c_0 as isize)).p
                                                    [1 as std::ffi::c_int as usize]
                                                    as std::ffi::c_int)
                                                * (ii2
                                                    - (*parameters.offset(c_0 as isize)).p
                                                        [1 as std::ffi::c_int as usize]
                                                        as std::ffi::c_int)
                                            + ((ii3
                                                - (*parameters.offset(c_0 as isize)).p
                                                    [2 as std::ffi::c_int as usize]
                                                    as std::ffi::c_int)
                                                * (ii3
                                                    - (*parameters.offset(c_0 as isize)).p
                                                        [2 as std::ffi::c_int as usize]
                                                        as std::ffi::c_int)
                                                + (ii4
                                                    - (*parameters.offset(c_0 as isize)).p
                                                        [3 as std::ffi::c_int as usize]
                                                        as std::ffi::c_int)
                                                    * (ii4
                                                        - (*parameters.offset(c_0 as isize)).p
                                                            [3 as std::ffi::c_int as usize]
                                                            as std::ffi::c_int)))
                                        + 1 as std::ffi::c_int
                                            * ((iisum
                                                - (*parameters.offset(c_0 as isize)).p
                                                    [4 as std::ffi::c_int as usize]
                                                    as std::ffi::c_int)
                                                * (iisum
                                                    - (*parameters.offset(c_0 as isize)).p
                                                        [4 as std::ffi::c_int as usize]
                                                        as std::ffi::c_int));
                                    dist1 = 2 as std::ffi::c_int
                                        * ((ii1
                                            - (*parameters.offset(ch as isize)).p
                                                [0 as std::ffi::c_int as usize]
                                                as std::ffi::c_int)
                                            * (ii1
                                                - (*parameters.offset(ch as isize)).p
                                                    [0 as std::ffi::c_int as usize]
                                                    as std::ffi::c_int)
                                            + (ii2
                                                - (*parameters.offset(ch as isize)).p
                                                    [1 as std::ffi::c_int as usize]
                                                    as std::ffi::c_int)
                                                * (ii2
                                                    - (*parameters.offset(ch as isize)).p
                                                        [1 as std::ffi::c_int as usize]
                                                        as std::ffi::c_int)
                                            + ((ii3
                                                - (*parameters.offset(ch as isize)).p
                                                    [2 as std::ffi::c_int as usize]
                                                    as std::ffi::c_int)
                                                * (ii3
                                                    - (*parameters.offset(ch as isize)).p
                                                        [2 as std::ffi::c_int as usize]
                                                        as std::ffi::c_int)
                                                + (ii4
                                                    - (*parameters.offset(ch as isize)).p
                                                        [3 as std::ffi::c_int as usize]
                                                        as std::ffi::c_int)
                                                    * (ii4
                                                        - (*parameters.offset(ch as isize)).p
                                                            [3 as std::ffi::c_int as usize]
                                                            as std::ffi::c_int)))
                                        + 1 as std::ffi::c_int
                                            * ((iisum
                                                - (*parameters.offset(ch as isize)).p
                                                    [4 as std::ffi::c_int as usize]
                                                    as std::ffi::c_int)
                                                * (iisum
                                                    - (*parameters.offset(ch as isize)).p
                                                        [4 as std::ffi::c_int as usize]
                                                        as std::ffi::c_int));
                                }
                                if ch == 0 || dist < dist1 {
                                    *table.offset(index as isize) = c_0;
                                    if *next.offset(index as isize) as std::ffi::c_int == index
                                        && last != index
                                    {
                                        if last != -(1 as std::ffi::c_int) {
                                            *next.offset(last as isize) =
                                                index as std::ffi::c_ushort;
                                            last = index;
                                        } else {
                                            first = index;
                                            last = first;
                                        }
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                    ii += 2 as std::ffi::c_int;
                }
                dm += 1;
                dm;
            }
            i1 = i;
            i = *next.offset(i as isize) as std::ffi::c_int;
            *next.offset(i1 as isize) = i1 as std::ffi::c_ushort;
            if !(i != i1) {
                break;
            }
        }
        if !(last != -(1 as std::ffi::c_int)) {
            break;
        }
    }
    (*c).table = table;
    (*c).filltable = filltable;
    (*c).parameters = parameters;
    free(next as *mut std::ffi::c_void);
    return table;
}
