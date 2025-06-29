use super::aafont::__aa_calcparams;
use super::aastructs::*;

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
static mut priority: [i64; 5] = [4, 5, 3, 2, 1];

pub fn aa_mktable(mut c: *mut aa_context) -> *mut u32 {
    unsafe {
        let mut i: i64 = 0;
        let mut i1: i64 = 0;
        let mut i2: i64 = 0;
        let mut i3: i64 = 0;
        let mut i4: i64 = 0;
        let mut sum: i64 = 0;
        let mut pos: i64 = 0;
        let mut currfont: *const aa_font = (*c).params.font;
        let mut supported = (*c).params.supported;
        let mut next: *mut u32 = 0 as *mut _;
        let mut first: i64 = -1;
        let mut last: i64 = -1;
        let mut table: *mut u32 = 0 as *mut _;
        let mut filltable: *mut u32 = 0 as *mut _;
        static mut parameters: *mut parameters = 0 as *const parameters as *mut parameters;
        next = malloc((::core::mem::size_of::<i64>() as u64).wrapping_mul(65536)) as *mut u32;
        parameters = calloc(
            1,
            (core::mem::size_of::<parameters>() as u64).wrapping_mul((256 * 5 + 1) as u64),
        ) as *mut parameters;
        table = calloc(
            1,
            (65536 as u64).wrapping_mul(::core::mem::size_of::<u32>() as u64),
        ) as *mut u32;
        filltable = calloc(
            1,
            (256 as u64).wrapping_mul(::core::mem::size_of::<u32>() as u64),
        ) as *mut u32;
        first = -1;
        last = -1;
        i = 0;
        while i < 65536 {
            *next.offset(i as isize) = i as u32;
            *table.offset(i as isize) = 0;
            i += 1;
            i;
        }
        __aa_calcparams(
            currfont,
            parameters,
            supported,
            (*c).params.dimmul.into(),
            (*c).params.boldmul.into(),
        );
        i = 0;
        while i < 256 * 5 {
            if (*(*__ctype_b_loc()).offset((i & 0xff) as isize) as i64 & _ISgraph as i64 != 0
                || i & 0xff as i64 == ' ' as i64
                || i & 0xff as i64 > 160 as i64 && supported & 256 != 0
                || supported & 128 != 0 && i & 0xff != 0)
                && supported & 1 << (i >> 8) != 0
            {
                let mut current_block_33: u64;
                let mut p1 = 0;
                let mut p2 = 0;
                let mut p3 = 0;
                let mut p4 = 0;
                i1 = (*parameters.offset(i as isize)).p[0] as i64;
                i2 = (*parameters.offset(i as isize)).p[1] as i64;
                i3 = (*parameters.offset(i as isize)).p[2] as i64;
                i4 = (*parameters.offset(i as isize)).p[3] as i64;
                p1 = i1 >> 4;
                p2 = i2 >> 4;
                p3 = i3 >> 4;
                p4 = i4 >> 4;
                sum = (*parameters.offset(i as isize)).p[4] as i64;
                pos = (p1 << 12) + (p2 << 8) + (p3 << 4) + p4;
                if *table.offset(pos as isize) != 0 {
                    let mut sum_0 = 0;
                    p1 = (p1 << 4) + p1;
                    p2 = (p2 << 4) + p2;
                    p3 = (p3 << 4) + p3;
                    p4 = (p4 << 4) + p4;
                    sum_0 = p1 + p2 + p3 + p4;
                    p1 = 2
                        * (((*parameters.offset(i as isize)).p[0] - p1)
                            * ((*parameters.offset(i as isize)).p[0] - p1)
                            + ((*parameters.offset(i as isize)).p[1] - p2)
                                * ((*parameters.offset(i as isize)).p[1] - p2)
                            + (((*parameters.offset(i as isize)).p[2] - p3)
                                * ((*parameters.offset(i as isize)).p[2] - p3)
                                + ((*parameters.offset(i as isize)).p[3] - p4)
                                    * ((*parameters.offset(i as isize)).p[3] - p4)))
                        + 1 * (((*parameters.offset(i as isize)).p[4] - sum_0)
                            * ((*parameters.offset(i as isize)).p[4] - sum_0));
                    p1 = 2
                        * (((*parameters.offset(*table.offset(pos as isize) as isize)).p[0] - p1)
                            * ((*parameters.offset(*table.offset(pos as isize) as isize)).p[0]
                                - p1)
                            + ((*parameters.offset(*table.offset(pos as isize) as isize)).p[1]
                                - p2)
                                * ((*parameters.offset(*table.offset(pos as isize) as isize)).p
                                    [1]
                                    - p2)
                            + (((*parameters.offset(*table.offset(pos as isize) as isize)).p[2]
                                - p3)
                                * ((*parameters.offset(*table.offset(pos as isize) as isize)).p
                                    [2]
                                    - p3)
                                + ((*parameters.offset(*table.offset(pos as isize) as isize)).p
                                    [3]
                                    - p4)
                                    * ((*parameters
                                        .offset(*table.offset(pos as isize) as isize))
                                    .p[3]
                                        - p4)))
                        + 1 * (((*parameters.offset(*table.offset(pos as isize) as isize)).p[4]
                            - sum_0)
                            * ((*parameters.offset(*table.offset(pos as isize) as isize)).p[4]
                                - sum_0));
                    if p1 >= p1
                        && (p1 != p2
                            || priority[(i / 256) as usize]
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
                        *table.offset(pos as isize) = i as u32;
                        if *next.offset(pos as isize) as i64 == pos
                            && last != pos.try_into().unwrap()
                        {
                            if last != -(1) {
                                *next.offset(last as isize) = pos as u32;
                                last = pos as i64;
                            } else {
                                first = pos as i64;
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
        pos = 0;
        while pos < 256 {
            let mut mindist = 2147483647;
            let mut d1 = 0;
            i = 0;
            while i < 256 * 5 {
                if (*(*__ctype_b_loc()).offset((i & 0xff) as isize)
                    & _ISgraph as std::ffi::c_ushort
                    != 0
                    || i & 0xff == ' ' as i64
                    || i & 0xff > 160 && supported & 256 != 0
                    || supported & 128 != 0 && i & 0xff != 0)
                    && supported & (1) << (i >> 8) != 0
                {
                    d1 = ((*parameters.offset(i as isize)).p[0 as usize] - pos)
                        * ((*parameters.offset(i as isize)).p[0 as usize] - pos)
                        + ((*parameters.offset(i as isize)).p[1 as usize] - pos)
                            * ((*parameters.offset(i as isize)).p[1 as usize] - pos)
                        + (((*parameters.offset(i as isize)).p[2 as usize] - pos)
                            * ((*parameters.offset(i as isize)).p[2 as usize] - pos)
                            + ((*parameters.offset(i as isize)).p[3 as usize] - pos)
                                * ((*parameters.offset(i as isize)).p[3 as usize] - pos))
                        + 2 * (((*parameters.offset(i as isize)).p[4 as usize] - pos * 4)
                            * ((*parameters.offset(i as isize)).p[4 as usize] - pos * 4));
                    if d1 <= mindist
                        && (d1 != mindist
                            || priority[(i / 256) as usize]
                                > priority[(*filltable.offset(pos as isize) / 256) as usize])
                    {
                        *filltable.offset(pos as isize) = i as u32;
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
            let mut blocked = 0;
            if !(last != -1) {
                break;
            }
            *next.offset(last as isize) = last as u32;
            blocked = last;
            i = first;
            if i == -(1) {
                break;
            }
            last = -(1);
            first = last;
            loop {
                let mut m1 = 0;
                let mut m2 = 0;
                let mut m3 = 0;
                let mut m4 = 0;
                let mut ii: i64 = 0;
                let mut dm = 0;
                let mut c_0 = *table.offset(i as isize);
                m1 = i >> 12;
                m2 = i >> 8 & 15;
                m3 = i >> 4 & 15;
                m4 = i & 15;
                dm = 0;
                while dm < 4 {
                    let mut current_block_77: u64;
                    ii = -(1);
                    while ii <= 1 {
                        let mut dist = 0;
                        let mut dist1 = 0;
                        let mut index = 0;
                        let mut ch = 0;
                        i1 = m1.try_into().unwrap();
                        i2 = m2.try_into().unwrap();
                        i3 = m3.try_into().unwrap();
                        i4 = m4.try_into().unwrap();
                        match dm {
                            0 => {
                                i1 += ii;
                                if i1 < 0 || i1 >= 16 {
                                    current_block_77 = 10930818133215224067;
                                } else {
                                    current_block_77 = 2606304779496145856;
                                }
                            }
                            1 => {
                                i2 += ii;
                                if i2 < 0 || i2 >= 16 {
                                    current_block_77 = 10930818133215224067;
                                } else {
                                    current_block_77 = 2606304779496145856;
                                }
                            }
                            2 => {
                                i3 += ii;
                                if i3 < 0 || i3 >= 16 {
                                    current_block_77 = 10930818133215224067;
                                } else {
                                    current_block_77 = 2606304779496145856;
                                }
                            }
                            3 => {
                                i4 += ii;
                                if i4 < 0 || i4 >= 16 {
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
                                index = (i1 << 12) + (i2 << 8) + (i3 << 4) + i4;
                                ch = *table.offset(index as isize);
                                if !(ch == c_0 || index == blocked.try_into().unwrap()) {
                                    if ch != 0 {
                                        let mut ii1 = (i1 << 4) + i1;
                                        let mut ii2 = (i2 << 4) + i2;
                                        let mut ii3 = (i3 << 4) + i3;
                                        let mut ii4 = (i4 << 4) + i4;
                                        let mut iisum = ii1 + ii2 + ii3 + ii4;
                                        dist = 2
                                            * ((ii1
                                                - (*parameters.offset(c_0 as isize)).p
                                                    [0 as usize])
                                                * (ii1
                                                    - (*parameters.offset(c_0 as isize)).p
                                                        [0 as usize])
                                                + (ii2
                                                    - (*parameters.offset(c_0 as isize)).p
                                                        [1 as usize])
                                                    * (ii2
                                                        - (*parameters.offset(c_0 as isize)).p
                                                            [1 as usize])
                                                + ((ii3
                                                    - (*parameters.offset(c_0 as isize)).p
                                                        [2 as usize])
                                                    * (ii3
                                                        - (*parameters.offset(c_0 as isize)).p
                                                            [2 as usize])
                                                    + (ii4
                                                        - (*parameters.offset(c_0 as isize)).p
                                                            [3 as usize])
                                                        * (ii4
                                                            - (*parameters.offset(c_0 as isize))
                                                                .p
                                                                [3 as usize])))
                                            + 1 * ((iisum
                                                - (*parameters.offset(c_0 as isize)).p
                                                    [4 as usize])
                                                * (iisum
                                                    - (*parameters.offset(c_0 as isize)).p
                                                        [4 as usize]));
                                        dist1 = 2
                                            * ((ii1
                                                - (*parameters.offset(ch as isize)).p[0 as usize])
                                                * (ii1
                                                    - (*parameters.offset(ch as isize)).p
                                                        [0 as usize])
                                                + (ii2
                                                    - (*parameters.offset(ch as isize)).p
                                                        [1 as usize])
                                                    * (ii2
                                                        - (*parameters.offset(ch as isize)).p
                                                            [1 as usize])
                                                + ((ii3
                                                    - (*parameters.offset(ch as isize)).p
                                                        [2 as usize])
                                                    * (ii3
                                                        - (*parameters.offset(ch as isize)).p
                                                            [2 as usize])
                                                    + (ii4
                                                        - (*parameters.offset(ch as isize)).p
                                                            [3 as usize])
                                                        * (ii4
                                                            - (*parameters.offset(ch as isize))
                                                                .p
                                                                [3 as usize])))
                                            + 1 * ((iisum
                                                - (*parameters.offset(ch as isize)).p[4 as usize])
                                                * (iisum
                                                    - (*parameters.offset(ch as isize)).p
                                                        [4 as usize]));
                                    }
                                    if ch == 0 || dist < dist1 {
                                        *table.offset(index as isize) = c_0;
                                        if *next.offset(index as isize) as i64 == index
                                            && last != index
                                        {
                                            if last != -(1) {
                                                *next.offset(last as isize) = index as u32;
                                                last = index as i64;
                                            } else {
                                                first = index as i64;
                                                last = first;
                                            }
                                        }
                                    }
                                }
                            }
                            _ => {}
                        }
                        ii += 2;
                    }
                    dm += 1;
                    dm;
                }
                i1 = i;
                i = *next.offset(i as isize) as i64;
                *next.offset(i1 as isize) = i1 as u32;
                if !(i != i1.try_into().unwrap()) {
                    break;
                }
            }
            if !(last != -(1)) {
                break;
            }
        }
        (*c).table = table;
        (*c).filltable = filltable;
        (*c).parameters = parameters;
        free(next as *mut std::ffi::c_void);
        return table;
    }
}
