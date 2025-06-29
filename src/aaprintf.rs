use super::aaout::aa_puts;
use super::aastructs::*;

pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: std::ffi::c_uint,
    pub fp_offset: std::ffi::c_uint,
    pub overflow_arg_area: *mut std::ffi::c_void,
    pub reg_save_area: *mut std::ffi::c_void,
}
pub type va_list = __builtin_va_list;
pub type aa_attribute = std::ffi::c_uint;
pub const AA_SPECIAL: aa_attribute = 5;
pub const AA_REVERSE: aa_attribute = 4;
pub const AA_BOLDFONT: aa_attribute = 3;
pub const AA_BOLD: aa_attribute = 2;
pub const AA_DIM: aa_attribute = 1;
pub const AA_NORMAL: aa_attribute = 0;
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

pub fn aa_printf(
    c: *mut aa_context,
    x: i64,
    y: i64,
    attr: aa_attribute,
    fmt: *const std::ffi::c_char,
    args: String,
) -> std::ffi::c_int {
    let mut buf = [0 as std::ffi::c_char; 1025];
    let bytes = args.as_bytes();
    let len = std::cmp::min(bytes.len(), 1024);
    for (i, &b) in bytes.iter().take(len).enumerate() {
        buf[i] = b as std::ffi::c_char;
    }
    buf[len] = 0;
    unsafe { aa_puts(c, x, y, attr, buf.as_mut_ptr()) };
    len.try_into().unwrap()
}
