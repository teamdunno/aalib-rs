use super::aaedit::aa_edit;
use super::aafastre::aa_fastrender;
use super::aaflush::aa_flush;
use super::aaflush::aa_hidemouse;
use super::aain::{aa_context, aa_getevent, aa_getkey, aa_getmouse, aa_hardware_params};
use super::aakbdreg::aa_autoinitkbd;
use super::aalib::{aa_close, aa_uninitmouse};
use super::aamoureg::aa_autoinitmouse;
use super::aaout::{aa_hidecursor, aa_puts};
use super::aaparse::aa_parseoptions;
use super::aaprintf::aa_printf;
use super::aaregist::aa_autoinit;
use super::aarender::{aa_getrenderparams, aa_render, aa_renderparams};

unsafe extern "C" {
    fn printf(_: *const std::ffi::c_char, _: ...) -> std::ffi::c_int;
    fn sprintf(_: *mut std::ffi::c_char, _: *const std::ffi::c_char, _: ...) -> std::ffi::c_int;
    static aa_help: *const std::ffi::c_char;
    static aa_dithernames: [*const std::ffi::c_char; 0];
    static mut aa_defparams: aa_hardware_params;
    fn strcpy(_: *mut std::ffi::c_char, _: *const std::ffi::c_char) -> *mut std::ffi::c_char;
    fn exit(_: std::ffi::c_int) -> !;
}
pub type aa_attribute = std::ffi::c_uint;
pub const AA_SPECIAL: aa_attribute = 5;
pub const AA_REVERSE: aa_attribute = 4;
pub const AA_BOLDFONT: aa_attribute = 3;
pub const AA_BOLD: aa_attribute = 2;
pub const AA_DIM: aa_attribute = 1;
pub const AA_NORMAL: aa_attribute = 0;
pub type aa_dithering_mode = std::ffi::c_uint;
pub const AA_DITHERTYPES: aa_dithering_mode = 3;
pub const AA_FLOYD_S: aa_dithering_mode = 2;
pub const AA_ERRORDISTRIB: aa_dithering_mode = 1;
pub const AA_NONE: aa_dithering_mode = 0;
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
pub struct aa_font {
    pub data: *const std::ffi::c_uchar,
    pub height: std::ffi::c_int,
    pub name: *const std::ffi::c_char,
    pub shortname: *const std::ffi::c_char,
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
unsafe fn main_0(
    mut argc: std::ffi::c_int,
    mut argv: *mut *mut std::ffi::c_char,
) -> std::ffi::c_int {
    let mut c: *mut aa_context = 0 as *mut aa_context;
    let mut i: std::ffi::c_int = 0;
    let mut y: std::ffi::c_int = 0;
    let mut s: [std::ffi::c_char; 256] = [0; 256];
    let mut p: *mut aa_renderparams = 0 as *mut aa_renderparams;
    strcpy(
        s.as_mut_ptr(),
        b"line editor.\0" as *const u8 as *const std::ffi::c_char,
    );
    if aa_parseoptions(
        0 as *mut aa_hardware_params,
        0 as *mut aa_renderparams,
        &mut argc,
        argv,
    ) == 0
        || argc != 1 as std::ffi::c_int
    {
        printf(b"%s\n\0" as *const u8 as *const std::ffi::c_char, aa_help);
        exit(1 as std::ffi::c_int);
    }
    c = aa_autoinit(&mut aa_defparams);
    if c.is_null() {
        printf(b"Can not intialize aalib\n\0" as *const u8 as *const std::ffi::c_char);
        exit(2 as std::ffi::c_int);
    }
    if aa_autoinitkbd(c, 0 as std::ffi::c_int) == 0 {
        printf(b"Can not intialize keyboard\n\0" as *const u8 as *const std::ffi::c_char);
        aa_close(c);
        exit(3 as std::ffi::c_int);
    }
    i = 0 as std::ffi::c_int;
    while i < (*c).imgwidth {
        y = 0 as std::ffi::c_int;
        while y < (*c).imgheight {
            ({
                let mut ___aa_context: *mut aa_context = c;
                *((*___aa_context).imagebuffer)
                    .offset((i + y * (*___aa_context).imgwidth) as isize) =
                    (if i + y < 80 as std::ffi::c_int {
                        i
                    } else if i + y < 100 as std::ffi::c_int {
                        if i + y == 89 as std::ffi::c_int {
                            150 as std::ffi::c_int
                        } else {
                            0 as std::ffi::c_int
                        }
                    } else {
                        y * 8 as std::ffi::c_int
                    }) as std::ffi::c_uchar;
                0 as std::ffi::c_int
            });
            y += 1;
            y;
        }
        i += 1;
        i;
    }
    aa_hidecursor(c);
    aa_fastrender(
        c,
        0 as std::ffi::c_int,
        0 as std::ffi::c_int,
        (*c).params.width,
        (*c).params.height,
    );
    aa_printf(
        c,
        0 as std::ffi::c_int,
        0 as std::ffi::c_int,
        AA_SPECIAL,
        b"Fast rendering routine %i\0" as *const u8 as *const std::ffi::c_char,
        1.to_string(),
    );
    aa_flush(c);
    aa_getkey(c, 1 as std::ffi::c_int);
    aa_edit(
        c,
        0 as std::ffi::c_int,
        1 as std::ffi::c_int,
        20 as std::ffi::c_int,
        s.as_mut_ptr(),
        256 as std::ffi::c_int,
    );
    aa_puts(
        c,
        0 as std::ffi::c_int,
        0 as std::ffi::c_int,
        AA_SPECIAL,
        b"Key lookup test        \0" as *const u8 as *const std::ffi::c_char,
    );
    aa_flush(c);
    while aa_getkey(c, 0 as std::ffi::c_int) == AA_NONE as std::ffi::c_int {}
    if aa_autoinitmouse(c, 7 as std::ffi::c_int) != 0 {
        let mut co: std::ffi::c_int = 0 as std::ffi::c_int;
        sprintf(
            s.as_mut_ptr(),
            b"Mouse test-space to exit\0" as *const u8 as *const std::ffi::c_char,
        );
        aa_puts(
            c,
            0 as std::ffi::c_int,
            0 as std::ffi::c_int,
            AA_SPECIAL,
            s.as_mut_ptr(),
        );
        aa_flush(c);
        while aa_getevent(c, 1 as std::ffi::c_int) != ' ' as i32 {
            let mut x: std::ffi::c_int = 0;
            let mut y_0: std::ffi::c_int = 0;
            let mut b: std::ffi::c_int = 0;
            let mut s_0: [std::ffi::c_char; 80] = [0; 80];
            co += 1;
            co;
            aa_getmouse(c, &mut x, &mut y_0, &mut b);
            sprintf(
                s_0.as_mut_ptr(),
                b"Mouse test-space to exit. x:%i y:%i b:%i event #%i  \0" as *const u8
                    as *const std::ffi::c_char,
                x,
                y_0,
                b,
                co,
            );
            aa_puts(
                c,
                0 as std::ffi::c_int,
                0 as std::ffi::c_int,
                AA_SPECIAL,
                s_0.as_mut_ptr(),
            );
            aa_flush(c);
        }
        aa_hidemouse(c);
        while aa_getevent(c, 1 as std::ffi::c_int) != ' ' as i32 {
            let mut x_0: std::ffi::c_int = 0;
            let mut y_1: std::ffi::c_int = 0;
            let mut b_0: std::ffi::c_int = 0;
            let mut s_1: [std::ffi::c_char; 80] = [0; 80];
            co += 1;
            co;
            aa_getmouse(c, &mut x_0, &mut y_1, &mut b_0);
            sprintf(
                s_1.as_mut_ptr(),
                b"Hidden mouse test-space to exit. x:%i y:%i b:%i event #%i  \0" as *const u8
                    as *const std::ffi::c_char,
                x_0,
                y_1,
                b_0,
                co,
            );
            aa_puts(
                c,
                0 as std::ffi::c_int,
                0 as std::ffi::c_int,
                AA_SPECIAL,
                s_1.as_mut_ptr(),
            );
            aa_flush(c);
        }
        aa_uninitmouse(c);
    }
    p = aa_getrenderparams();
    i = 0 as std::ffi::c_int;
    while i < AA_DITHERTYPES as std::ffi::c_int {
        (*p).dither = i as aa_dithering_mode;
        aa_render(
            c,
            p,
            0 as std::ffi::c_int,
            0 as std::ffi::c_int,
            (*c).params.width,
            (*c).params.height,
        );
        aa_puts(
            c,
            0 as std::ffi::c_int,
            0 as std::ffi::c_int,
            AA_SPECIAL,
            *aa_dithernames.as_ptr().offset(i as isize),
        );
        aa_flush(c);
        aa_getkey(c, 1 as std::ffi::c_int);
        i += 1;
        i;
    }
    i = 0 as std::ffi::c_int;
    while i < 255 as std::ffi::c_int {
        (*p).bright = i;
        aa_render(
            c,
            p,
            0 as std::ffi::c_int,
            0 as std::ffi::c_int,
            (*c).params.width,
            (*c).params.height,
        );
        aa_puts(
            c,
            0 as std::ffi::c_int,
            0 as std::ffi::c_int,
            AA_SPECIAL,
            b"Normal rendering - bright changes\0" as *const u8 as *const std::ffi::c_char,
        );
        aa_flush(c);
        aa_getkey(c, 1 as std::ffi::c_int);
        i += 32 as std::ffi::c_int;
    }
    (*p).bright = 0 as std::ffi::c_int;
    i = 0 as std::ffi::c_int;
    while i < 128 as std::ffi::c_int {
        (*p).contrast = i;
        aa_render(
            c,
            p,
            0 as std::ffi::c_int,
            0 as std::ffi::c_int,
            (*c).params.width,
            (*c).params.height,
        );
        aa_puts(
            c,
            0 as std::ffi::c_int,
            0 as std::ffi::c_int,
            AA_SPECIAL,
            b"Normal rendering - contrast changes\0" as *const u8 as *const std::ffi::c_char,
        );
        aa_flush(c);
        aa_getkey(c, 1 as std::ffi::c_int);
        i += 16 as std::ffi::c_int;
    }
    (*p).contrast = 0 as std::ffi::c_int;
    i = 0 as std::ffi::c_int;
    while i < 255 as std::ffi::c_int {
        (*p).gamma = (1 as std::ffi::c_int as std::ffi::c_double
            + i as std::ffi::c_double / 32.0f64) as std::ffi::c_float;
        aa_render(
            c,
            p,
            0 as std::ffi::c_int,
            0 as std::ffi::c_int,
            (*c).params.width,
            (*c).params.height,
        );
        aa_puts(
            c,
            0 as std::ffi::c_int,
            0 as std::ffi::c_int,
            AA_SPECIAL,
            b"Normal rendering - gamma changes\0" as *const u8 as *const std::ffi::c_char,
        );
        aa_flush(c);
        aa_getkey(c, 1 as std::ffi::c_int);
        i += 32 as std::ffi::c_int;
    }
    (*p).gamma = 1.0f64 as std::ffi::c_float;
    i = 0 as std::ffi::c_int;
    while i < 255 as std::ffi::c_int {
        (*p).randomval = i;
        aa_render(
            c,
            p,
            0 as std::ffi::c_int,
            0 as std::ffi::c_int,
            (*c).params.width,
            (*c).params.height,
        );
        aa_puts(
            c,
            0 as std::ffi::c_int,
            0 as std::ffi::c_int,
            AA_SPECIAL,
            b"Normal rendering - randomval changes\0" as *const u8 as *const std::ffi::c_char,
        );
        aa_flush(c);
        aa_getkey(c, 1 as std::ffi::c_int);
        i += 32 as std::ffi::c_int;
    }
    aa_close(c);
    return 0 as std::ffi::c_int;
}
pub fn main() {
    let mut args: Vec<*mut std::ffi::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0(
            (args.len() - 1) as std::ffi::c_int,
            args.as_mut_ptr() as *mut *mut std::ffi::c_char,
        ) as i32)
    }
}
