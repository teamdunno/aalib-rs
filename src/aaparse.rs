use super::aarec::aa_recommendhi;
use super::aastructs::*;
use ::c2rust_bitfields;
unsafe extern "C" {
    fn strncpy(
        _: *mut std::ffi::c_char,
        _: *const std::ffi::c_char,
        _: std::ffi::c_ulong,
    ) -> *mut std::ffi::c_char;
    fn strcmp(_: *const std::ffi::c_char, _: *const std::ffi::c_char) -> std::ffi::c_int;
    fn strlen(_: *const std::ffi::c_char) -> std::ffi::c_ulong;
    fn strtod(_: *const std::ffi::c_char, _: *mut *mut std::ffi::c_char) -> std::ffi::c_double;
    fn strtol(
        _: *const std::ffi::c_char,
        _: *mut *mut std::ffi::c_char,
        _: std::ffi::c_int,
    ) -> std::ffi::c_long;
    fn calloc(_: std::ffi::c_ulong, _: std::ffi::c_ulong) -> *mut std::ffi::c_void;
    fn free(_: *mut std::ffi::c_void);
    fn getenv(__name: *const std::ffi::c_char) -> *mut std::ffi::c_char;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const std::ffi::c_char, _: ...) -> std::ffi::c_int;
    fn printf(_: *const std::ffi::c_char, _: ...) -> std::ffi::c_int;
    static mut aa_fonts: [*const aa_font; 0];
    static mut aa_kbdrecommended: *mut aa_linkedlist;
    static mut aa_mouserecommended: *mut aa_linkedlist;
    static mut aa_displayrecommended: *mut aa_linkedlist;
    static mut aa_defparams: aa_hardware_params;
    static mut aa_defrenderparams: aa_renderparams;
}
pub type __off_t = std::ffi::c_long;
pub type __off64_t = std::ffi::c_long;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: std::ffi::c_int,
    pub _IO_read_ptr: *mut std::ffi::c_char,
    pub _IO_read_end: *mut std::ffi::c_char,
    pub _IO_read_base: *mut std::ffi::c_char,
    pub _IO_write_base: *mut std::ffi::c_char,
    pub _IO_write_ptr: *mut std::ffi::c_char,
    pub _IO_write_end: *mut std::ffi::c_char,
    pub _IO_buf_base: *mut std::ffi::c_char,
    pub _IO_buf_end: *mut std::ffi::c_char,
    pub _IO_save_base: *mut std::ffi::c_char,
    pub _IO_backup_base: *mut std::ffi::c_char,
    pub _IO_save_end: *mut std::ffi::c_char,
    pub _chain: *mut _IO_FILE,
    pub _fileno: std::ffi::c_int,
    #[bitfield(name = "_flags2", ty = "std::ffi::c_int", bits = "0..=23")]
    pub _flags2: [u8; 3],
    pub _short_backupbuf: [std::ffi::c_char; 1],
    pub _old_offset: __off_t,
    pub _cur_column: std::ffi::c_ushort,
    pub _vtable_offset: std::ffi::c_schar,
    pub _shortbuf: [std::ffi::c_char; 1],
    pub _lock: *mut std::ffi::c_void,
    pub _offset: __off64_t,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut std::ffi::c_void,
    pub _prevchain: *mut *mut _IO_FILE,
    pub _mode: std::ffi::c_int,
    pub _unused2: [std::ffi::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type aa_dithering_mode = std::ffi::c_uint;
pub const AA_DITHERTYPES: aa_dithering_mode = 3;
pub const AA_FLOYD_S: aa_dithering_mode = 2;
pub const AA_ERRORDISTRIB: aa_dithering_mode = 1;
pub const AA_NONE: aa_dithering_mode = 0;
#[inline]
unsafe extern "C" fn atol(mut __nptr: *const std::ffi::c_char) -> std::ffi::c_long {
    return strtol(
        __nptr,
        0 as *mut std::ffi::c_void as *mut *mut std::ffi::c_char,
        10 as std::ffi::c_int,
    );
}
#[inline]
unsafe extern "C" fn atof(mut __nptr: *const std::ffi::c_char) -> std::ffi::c_double {
    return strtod(
        __nptr,
        0 as *mut std::ffi::c_void as *mut *mut std::ffi::c_char,
    );
}
static mut inparse: std::ffi::c_int = 0;
unsafe extern "C" fn aa_remove(
    mut i: std::ffi::c_int,
    mut argc: *mut std::ffi::c_int,
    mut argv: *mut *mut std::ffi::c_char,
) {
    let mut y: std::ffi::c_int = 0;
    if i < 0 as std::ffi::c_int || i >= *argc {
        printf(b"AA Internal error #1-please report\n\0" as *const u8 as *const std::ffi::c_char);
        return;
    }
    y = i;
    while y < *argc - 1 as std::ffi::c_int {
        let ref mut fresh0 = *argv.offset(y as isize);
        *fresh0 = *argv.offset((y + 1 as std::ffi::c_int) as isize);
        y += 1;
        y;
    }
    let ref mut fresh1 = *argv.offset((*argc - 1 as std::ffi::c_int) as isize);
    *fresh1 = 0 as *mut std::ffi::c_char;
    *argc -= 1;
    *argc;
}

pub unsafe extern "C" fn aa_parseoptions(
    mut p: *mut aa_hardware_params,
    mut r: *mut aa_renderparams,
    mut argc: *mut std::ffi::c_int,
    mut argv: *mut *mut std::ffi::c_char,
) -> std::ffi::c_int {
    let mut i: std::ffi::c_int = 0;
    let mut y: std::ffi::c_int = 0;
    let mut supported: std::ffi::c_int = 0;
    if inparse == 0 {
        parseenv(p, r);
    }
    if argc.is_null() || argv.is_null() {
        return 1 as std::ffi::c_int;
    }
    supported = if !p.is_null() {
        (*p).supported
    } else {
        aa_defparams.supported
    };
    if p.is_null() {
        p = &mut aa_defparams;
    }
    if r.is_null() {
        r = &mut aa_defrenderparams;
    }
    i = 1 as std::ffi::c_int;
    while i < *argc {
        if strcmp(
            *argv.offset(i as isize),
            b"-font\0" as *const u8 as *const std::ffi::c_char,
        ) == 0 as std::ffi::c_int
        {
            aa_remove(i, argc, argv);
            if *argc == i {
                fprintf(
                    stderr,
                    b"font name expected\n\0" as *const u8 as *const std::ffi::c_char,
                );
                return 0 as std::ffi::c_int;
            }
            y = 0 as std::ffi::c_int;
            while !(*aa_fonts.as_mut_ptr().offset(y as isize)).is_null() {
                if strcmp(
                    *argv.offset(i as isize),
                    (**aa_fonts.as_mut_ptr().offset(y as isize)).name,
                ) == 0
                    || strcmp(
                        *argv.offset(i as isize),
                        (**aa_fonts.as_mut_ptr().offset(y as isize)).shortname,
                    ) == 0
                {
                    (*p).font = *aa_fonts.as_mut_ptr().offset(y as isize);
                    aa_remove(i, argc, argv);
                    break;
                } else {
                    y += 1;
                    y;
                }
            }
            if (*aa_fonts.as_mut_ptr().offset(i as isize)).is_null() {
                fprintf(
                    stderr,
                    b"font name expected\n\0" as *const u8 as *const std::ffi::c_char,
                );
                return 0 as std::ffi::c_int;
            }
            i -= 1;
            i;
        } else if strcmp(
            *argv.offset(i as isize),
            b"-normal\0" as *const u8 as *const std::ffi::c_char,
        ) == 0 as std::ffi::c_int
        {
            aa_remove(i, argc, argv);
            i -= 1;
            i;
            supported |= 1 as std::ffi::c_int;
            (*p).supported = supported;
        } else if strcmp(
            *argv.offset(i as isize),
            b"-nonormal\0" as *const u8 as *const std::ffi::c_char,
        ) == 0 as std::ffi::c_int
        {
            aa_remove(i, argc, argv);
            i -= 1;
            i;
            supported &= !(1 as std::ffi::c_int);
            (*p).supported = supported;
        } else if strcmp(
            *argv.offset(i as isize),
            b"-bold\0" as *const u8 as *const std::ffi::c_char,
        ) == 0 as std::ffi::c_int
        {
            aa_remove(i, argc, argv);
            i -= 1;
            i;
            supported |= 4 as std::ffi::c_int;
            (*p).supported = supported;
        } else if strcmp(
            *argv.offset(i as isize),
            b"-nobold\0" as *const u8 as *const std::ffi::c_char,
        ) == 0 as std::ffi::c_int
        {
            aa_remove(i, argc, argv);
            i -= 1;
            i;
            supported &= !(4 as std::ffi::c_int);
            (*p).supported = supported;
        } else if strcmp(
            *argv.offset(i as isize),
            b"-boldfont\0" as *const u8 as *const std::ffi::c_char,
        ) == 0 as std::ffi::c_int
        {
            aa_remove(i, argc, argv);
            i -= 1;
            i;
            supported |= 8 as std::ffi::c_int;
            (*p).supported = supported;
        } else if strcmp(
            *argv.offset(i as isize),
            b"-noboldfont\0" as *const u8 as *const std::ffi::c_char,
        ) == 0 as std::ffi::c_int
        {
            aa_remove(i, argc, argv);
            i -= 1;
            i;
            supported &= !(8 as std::ffi::c_int);
            (*p).supported = supported;
        } else if strcmp(
            *argv.offset(i as isize),
            b"-dim\0" as *const u8 as *const std::ffi::c_char,
        ) == 0 as std::ffi::c_int
        {
            aa_remove(i, argc, argv);
            i -= 1;
            i;
            supported |= 2 as std::ffi::c_int;
            (*p).supported = supported;
        } else if strcmp(
            *argv.offset(i as isize),
            b"-nodim\0" as *const u8 as *const std::ffi::c_char,
        ) == 0 as std::ffi::c_int
        {
            aa_remove(i, argc, argv);
            i -= 1;
            i;
            supported &= !(2 as std::ffi::c_int);
            (*p).supported = supported;
        } else if strcmp(
            *argv.offset(i as isize),
            b"-reverse\0" as *const u8 as *const std::ffi::c_char,
        ) == 0 as std::ffi::c_int
        {
            aa_remove(i, argc, argv);
            i -= 1;
            i;
            supported |= 16 as std::ffi::c_int;
            (*p).supported = supported;
        } else if strcmp(
            *argv.offset(i as isize),
            b"-extended\0" as *const u8 as *const std::ffi::c_char,
        ) == 0 as std::ffi::c_int
        {
            aa_remove(i, argc, argv);
            i -= 1;
            i;
            supported |= 128 as std::ffi::c_int | 256 as std::ffi::c_int;
            (*p).supported = supported;
        } else if strcmp(
            *argv.offset(i as isize),
            b"-eight\0" as *const u8 as *const std::ffi::c_char,
        ) == 0 as std::ffi::c_int
        {
            aa_remove(i, argc, argv);
            i -= 1;
            i;
            supported |= 256 as std::ffi::c_int;
            (*p).supported = supported;
        } else if strcmp(
            *argv.offset(i as isize),
            b"-noreverse\0" as *const u8 as *const std::ffi::c_char,
        ) == 0 as std::ffi::c_int
        {
            aa_remove(i, argc, argv);
            i -= 1;
            i;
            supported &= !(16 as std::ffi::c_int);
            (*p).supported = supported;
        } else if strcmp(
            *argv.offset(i as isize),
            b"-inverse\0" as *const u8 as *const std::ffi::c_char,
        ) == 0 as std::ffi::c_int
        {
            aa_remove(i, argc, argv);
            i -= 1;
            i;
            (*r).inversion = 1 as std::ffi::c_int;
        } else if strcmp(
            *argv.offset(i as isize),
            b"-noinverse\0" as *const u8 as *const std::ffi::c_char,
        ) == 0 as std::ffi::c_int
        {
            aa_remove(i, argc, argv);
            i -= 1;
            i;
            (*r).inversion = 0 as std::ffi::c_int;
        } else if strcmp(
            *argv.offset(i as isize),
            b"-nodither\0" as *const u8 as *const std::ffi::c_char,
        ) == 0 as std::ffi::c_int
        {
            aa_remove(i, argc, argv);
            i -= 1;
            i;
            (*r).dither = AA_NONE;
        } else if strcmp(
            *argv.offset(i as isize),
            b"-floyd_steinberg\0" as *const u8 as *const std::ffi::c_char,
        ) == 0 as std::ffi::c_int
        {
            aa_remove(i, argc, argv);
            i -= 1;
            i;
            (*r).dither = AA_FLOYD_S;
        } else if strcmp(
            *argv.offset(i as isize),
            b"-error_distribution\0" as *const u8 as *const std::ffi::c_char,
        ) == 0 as std::ffi::c_int
        {
            aa_remove(i, argc, argv);
            i -= 1;
            i;
            (*r).dither = AA_ERRORDISTRIB;
        } else if strcmp(
            *argv.offset(i as isize),
            b"-random\0" as *const u8 as *const std::ffi::c_char,
        ) == 0 as std::ffi::c_int
        {
            aa_remove(i, argc, argv);
            if *argc == i {
                fprintf(
                    stderr,
                    b"Random dithering value expected\n\0" as *const u8 as *const std::ffi::c_char,
                );
                return 0 as std::ffi::c_int;
            }
            (*r).randomval = atol(*argv.offset(i as isize)) as std::ffi::c_int;
            aa_remove(i, argc, argv);
            i -= 1;
            i;
        } else if strcmp(
            *argv.offset(i as isize),
            b"-bright\0" as *const u8 as *const std::ffi::c_char,
        ) == 0 as std::ffi::c_int
        {
            aa_remove(i, argc, argv);
            if *argc == i {
                fprintf(
                    stderr,
                    b"Bright value expected(0-255)\n\0" as *const u8 as *const std::ffi::c_char,
                );
                return 0 as std::ffi::c_int;
            }
            (*r).bright = atol(*argv.offset(i as isize)) as std::ffi::c_int;
            aa_remove(i, argc, argv);
            i -= 1;
            i;
        } else if strcmp(
            *argv.offset(i as isize),
            b"-contrast\0" as *const u8 as *const std::ffi::c_char,
        ) == 0 as std::ffi::c_int
        {
            aa_remove(i, argc, argv);
            if *argc == i {
                fprintf(
                    stderr,
                    b"Contrast value expected(0-255)\n\0" as *const u8 as *const std::ffi::c_char,
                );
                return 0 as std::ffi::c_int;
            }
            (*r).contrast = atol(*argv.offset(i as isize)) as std::ffi::c_int;
            aa_remove(i, argc, argv);
            i -= 1;
            i;
        } else if strcmp(
            *argv.offset(i as isize),
            b"-width\0" as *const u8 as *const std::ffi::c_char,
        ) == 0 as std::ffi::c_int
        {
            aa_remove(i, argc, argv);
            if *argc == i {
                fprintf(
                    stderr,
                    b"width expected\n\0" as *const u8 as *const std::ffi::c_char,
                );
                return 0 as std::ffi::c_int;
            }
            (*p).width = atol(*argv.offset(i as isize)) as std::ffi::c_int;
            aa_remove(i, argc, argv);
            i -= 1;
            i;
        } else if strcmp(
            *argv.offset(i as isize),
            b"-recwidth\0" as *const u8 as *const std::ffi::c_char,
        ) == 0 as std::ffi::c_int
        {
            aa_remove(i, argc, argv);
            if *argc == i {
                fprintf(
                    stderr,
                    b"width expected\n\0" as *const u8 as *const std::ffi::c_char,
                );
                return 0 as std::ffi::c_int;
            }
            (*p).recwidth = atol(*argv.offset(i as isize)) as std::ffi::c_int;
            aa_remove(i, argc, argv);
            i -= 1;
            i;
        } else if strcmp(
            *argv.offset(i as isize),
            b"-minwidth\0" as *const u8 as *const std::ffi::c_char,
        ) == 0 as std::ffi::c_int
        {
            aa_remove(i, argc, argv);
            if *argc == i {
                fprintf(
                    stderr,
                    b"width expected\n\0" as *const u8 as *const std::ffi::c_char,
                );
                return 0 as std::ffi::c_int;
            }
            (*p).minwidth = atol(*argv.offset(i as isize)) as std::ffi::c_int;
            aa_remove(i, argc, argv);
            i -= 1;
            i;
        } else if strcmp(
            *argv.offset(i as isize),
            b"-maxwidth\0" as *const u8 as *const std::ffi::c_char,
        ) == 0 as std::ffi::c_int
        {
            aa_remove(i, argc, argv);
            if *argc == i {
                fprintf(
                    stderr,
                    b"width expected\n\0" as *const u8 as *const std::ffi::c_char,
                );
                return 0 as std::ffi::c_int;
            }
            (*p).maxwidth = atol(*argv.offset(i as isize)) as std::ffi::c_int;
            aa_remove(i, argc, argv);
            i -= 1;
            i;
        } else if strcmp(
            *argv.offset(i as isize),
            b"-height\0" as *const u8 as *const std::ffi::c_char,
        ) == 0 as std::ffi::c_int
        {
            aa_remove(i, argc, argv);
            if *argc == i {
                fprintf(
                    stderr,
                    b"height expected\n\0" as *const u8 as *const std::ffi::c_char,
                );
                return 0 as std::ffi::c_int;
            }
            (*p).height = atol(*argv.offset(i as isize)) as std::ffi::c_int;
            aa_remove(i, argc, argv);
            i -= 1;
            i;
        } else if strcmp(
            *argv.offset(i as isize),
            b"-recheight\0" as *const u8 as *const std::ffi::c_char,
        ) == 0 as std::ffi::c_int
        {
            aa_remove(i, argc, argv);
            if *argc == i {
                fprintf(
                    stderr,
                    b"height expected\n\0" as *const u8 as *const std::ffi::c_char,
                );
                return 0 as std::ffi::c_int;
            }
            (*p).recheight = atol(*argv.offset(i as isize)) as std::ffi::c_int;
            aa_remove(i, argc, argv);
            i -= 1;
            i;
        } else if strcmp(
            *argv.offset(i as isize),
            b"-minheight\0" as *const u8 as *const std::ffi::c_char,
        ) == 0 as std::ffi::c_int
        {
            aa_remove(i, argc, argv);
            if *argc == i {
                fprintf(
                    stderr,
                    b"height expected\n\0" as *const u8 as *const std::ffi::c_char,
                );
                return 0 as std::ffi::c_int;
            }
            (*p).minheight = atol(*argv.offset(i as isize)) as std::ffi::c_int;
            aa_remove(i, argc, argv);
            i -= 1;
            i;
        } else if strcmp(
            *argv.offset(i as isize),
            b"-maxheight\0" as *const u8 as *const std::ffi::c_char,
        ) == 0 as std::ffi::c_int
        {
            aa_remove(i, argc, argv);
            if *argc == i {
                fprintf(
                    stderr,
                    b"height expected\n\0" as *const u8 as *const std::ffi::c_char,
                );
                return 0 as std::ffi::c_int;
            }
            (*p).maxheight = atol(*argv.offset(i as isize)) as std::ffi::c_int;
            aa_remove(i, argc, argv);
            i -= 1;
            i;
        } else if strcmp(
            *argv.offset(i as isize),
            b"-gamma\0" as *const u8 as *const std::ffi::c_char,
        ) == 0 as std::ffi::c_int
        {
            aa_remove(i, argc, argv);
            if *argc == i {
                fprintf(
                    stderr,
                    b"Gamma value expected\n\0" as *const u8 as *const std::ffi::c_char,
                );
                return 0 as std::ffi::c_int;
            }
            (*r).gamma = atof(*argv.offset(i as isize)) as std::ffi::c_float;
            aa_remove(i, argc, argv);
            i -= 1;
            i;
        } else if strcmp(
            *argv.offset(i as isize),
            b"-dimmul\0" as *const u8 as *const std::ffi::c_char,
        ) == 0 as std::ffi::c_int
        {
            aa_remove(i, argc, argv);
            if *argc == i {
                fprintf(
                    stderr,
                    b"Dimmul value expected\n\0" as *const u8 as *const std::ffi::c_char,
                );
                return 0 as std::ffi::c_int;
            }
            (*p).dimmul = atof(*argv.offset(i as isize));
            aa_remove(i, argc, argv);
            i -= 1;
            i;
        } else if strcmp(
            *argv.offset(i as isize),
            b"-boldmul\0" as *const u8 as *const std::ffi::c_char,
        ) == 0 as std::ffi::c_int
        {
            aa_remove(i, argc, argv);
            if *argc == i {
                fprintf(
                    stderr,
                    b"Dimmul value expected\n\0" as *const u8 as *const std::ffi::c_char,
                );
                return 0 as std::ffi::c_int;
            }
            (*p).boldmul = atof(*argv.offset(i as isize));
            aa_remove(i, argc, argv);
            i -= 1;
            i;
        } else if strcmp(
            *argv.offset(i as isize),
            b"-driver\0" as *const u8 as *const std::ffi::c_char,
        ) == 0 as std::ffi::c_int
        {
            aa_remove(i, argc, argv);
            if *argc == i {
                fprintf(
                    stderr,
                    b"Driver name expected\n\0" as *const u8 as *const std::ffi::c_char,
                );
                return 0 as std::ffi::c_int;
            }
            aa_recommendhi(&mut aa_displayrecommended, *argv.offset(i as isize));
            aa_remove(i, argc, argv);
            i -= 1;
            i;
        } else if strcmp(
            *argv.offset(i as isize),
            b"-kbddriver\0" as *const u8 as *const std::ffi::c_char,
        ) == 0 as std::ffi::c_int
        {
            aa_remove(i, argc, argv);
            if *argc == i {
                fprintf(
                    stderr,
                    b"Driver name expected\n\0" as *const u8 as *const std::ffi::c_char,
                );
                return 0 as std::ffi::c_int;
            }
            aa_recommendhi(&mut aa_kbdrecommended, *argv.offset(i as isize));
            aa_remove(i, argc, argv);
            i -= 1;
            i;
        } else if strcmp(
            *argv.offset(i as isize),
            b"-mousedriver\0" as *const u8 as *const std::ffi::c_char,
        ) == 0 as std::ffi::c_int
        {
            aa_remove(i, argc, argv);
            if *argc == i {
                fprintf(
                    stderr,
                    b"Driver name expected\n\0" as *const u8 as *const std::ffi::c_char,
                );
                return 0 as std::ffi::c_int;
            }
            aa_recommendhi(&mut aa_mouserecommended, *argv.offset(i as isize));
            aa_remove(i, argc, argv);
            i -= 1;
            i;
        }
        i += 1;
        i;
    }
    return 1 as std::ffi::c_int;
}
unsafe extern "C" fn parseenv(mut p: *mut aa_hardware_params, mut r: *mut aa_renderparams) {
    let mut env: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
    let mut argc: std::ffi::c_int = 1 as std::ffi::c_int;
    let mut i: std::ffi::c_int = 0;
    let mut argv: [*mut std::ffi::c_char; 256] = [0 as *mut std::ffi::c_char; 256];
    let mut argv1: [*mut std::ffi::c_char; 256] = [0 as *mut std::ffi::c_char; 256];
    inparse = 1 as std::ffi::c_int;
    env = getenv(b"AAOPTS\0" as *const u8 as *const std::ffi::c_char);
    if env.is_null() {
        return;
    }
    if *env.offset(0 as std::ffi::c_int as isize) != 0 {
        i = 0 as std::ffi::c_int;
        while i < strlen(env) as std::ffi::c_int {
            let mut s: std::ffi::c_int = 0;
            let mut stop: std::ffi::c_char = ' ' as i32 as std::ffi::c_char;
            while *env.offset(i as isize) as std::ffi::c_int == ' ' as i32 {
                i += 1;
                i;
            }
            if *env.offset(i as isize) as std::ffi::c_int == '"' as i32 {
                i += 1;
                i;
                stop = '"' as i32 as std::ffi::c_char;
            }
            s = i;
            while *env.offset(i as isize) as std::ffi::c_int != stop as std::ffi::c_int
                && *env.offset(i as isize) as std::ffi::c_int != 0
            {
                i += 1;
                i;
            }
            if i - s != 0 {
                argv[argc as usize] = calloc(
                    (i - s + 1 as std::ffi::c_int) as std::ffi::c_ulong,
                    1 as std::ffi::c_int as std::ffi::c_ulong,
                ) as *mut std::ffi::c_char;
                argv1[argc as usize] = argv[argc as usize];
                strncpy(
                    argv[argc as usize],
                    env.offset(s as isize),
                    (i - s) as std::ffi::c_ulong,
                );
                argc += 1;
                argc;
                if argc == 255 as std::ffi::c_int {
                    break;
                }
            }
            i += 1;
            i;
        }
    }
    i = argc;
    if i != 1 as std::ffi::c_int {
        aa_parseoptions(p, r, &mut i, argv.as_mut_ptr());
        i = 1 as std::ffi::c_int;
        while i < argc {
            free(argv1[i as usize] as *mut std::ffi::c_void);
            i += 1;
            i;
        }
    }
    inparse = 0 as std::ffi::c_int;
}
