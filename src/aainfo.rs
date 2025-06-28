use super::aain::{aa_context, aa_hardware_params};
use super::aakbdreg::aa_autoinitkbd;
use super::aalib::aa_close;
use super::aamoureg::aa_autoinitmouse;
use super::aaparse::aa_parseoptions;
use super::aaregist::aa_autoinit;
use super::aarender::aa_renderparams;

unsafe extern "C" {
    fn printf(_: *const std::ffi::c_char, _: ...) -> std::ffi::c_int;
    fn sprintf(_: *mut std::ffi::c_char, _: *const std::ffi::c_char, _: ...) -> std::ffi::c_int;
    static aa_help: *const std::ffi::c_char;
    static mut aa_defparams: aa_hardware_params;
    fn exit(_: std::ffi::c_int) -> !;
}
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
pub struct parameters {
    pub p: [std::ffi::c_uint; 5],
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
    let mut str: [std::ffi::c_char; 256] = [0; 256];
    let mut str1: [std::ffi::c_char; 256] = [0; 256];
    let mut str2: [std::ffi::c_char; 256] = [0; 256];
    if aa_parseoptions(
        0 as *mut aa_hardware_params,
        0 as *mut aa_renderparams,
        &mut argc,
        argv,
    ) == 0
        || argc != 1 as std::ffi::c_int
    {
        printf(b"%s\0" as *const u8 as *const std::ffi::c_char, aa_help);
        exit(1 as std::ffi::c_int);
    }
    c = aa_autoinit(&mut aa_defparams);
    if c.is_null() {
        printf(b"aalib initialization failed\n\0" as *const u8 as *const std::ffi::c_char);
        exit(1 as std::ffi::c_int);
    }
    aa_autoinitkbd(c, 0 as std::ffi::c_int);
    aa_autoinitmouse(c, 7 as std::ffi::c_int);
    sprintf(
        str.as_mut_ptr(),
        b"\n Current driver:%s\n  Short name   :%s\n Font          :%s\n  Short name   :%s\n  height       :%i\n Width         :%i\n Height        :%i\n Width in mm   :%i\n Height in mm  :%i\n Supported     :%i\n\0"
            as *const u8 as *const std::ffi::c_char,
        (*(*c).driver).name,
        (*(*c).driver).shortname,
        (*(*c).params.font).name,
        (*(*c).params.font).shortname,
        (*(*c).params.font).height,
        (*c).params.width,
        (*c).params.height,
        (*c).params.mmwidth,
        (*c).params.mmheight,
        (*c).driverparams.supported,
    );
    if !((*c).kbddriver).is_null() {
        sprintf(
            str1.as_mut_ptr(),
            b"\n Current driver:%s\n  Short name   :%s\n Flags         :%i\n\0" as *const u8
                as *const std::ffi::c_char,
            (*(*c).kbddriver).name,
            (*(*c).kbddriver).shortname,
            (*(*c).kbddriver).flags,
        );
    } else {
        sprintf(
            str1.as_mut_ptr(),
            b"not available\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    if !((*c).mousedriver).is_null() {
        sprintf(
            str2.as_mut_ptr(),
            b"\n Current driver:%s\n  Short name   :%s\n Flags         :%i\n\0" as *const u8
                as *const std::ffi::c_char,
            (*(*c).mousedriver).name,
            (*(*c).mousedriver).shortname,
            (*(*c).mousedriver).flags,
        );
    } else {
        sprintf(
            str2.as_mut_ptr(),
            b"not available\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    aa_close(c);
    printf(
        b"AAlib version:%i.%i\nDisplay:%s\nKeyboard:%s\nMouse:%s\n\0" as *const u8
            as *const std::ffi::c_char,
        1 as std::ffi::c_int,
        4 as std::ffi::c_int,
        str.as_mut_ptr(),
        str1.as_mut_ptr(),
        str2.as_mut_ptr(),
    );
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
