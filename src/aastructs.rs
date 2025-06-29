// Here are all structs shared across the code of the entire library.

use crate::aasave::FILE;

pub type aa_dithering_mode = u64;

#[derive(Copy, Clone)]
pub struct aa_driver {
    pub shortname: *const std::ffi::c_char,
    pub name: *const std::ffi::c_char,
    pub init: Option<
        unsafe fn(
            *const aa_hardware_params,
            *const std::ffi::c_void,
            *mut aa_hardware_params,
            *mut *mut std::ffi::c_void,
        ) -> i64,
    >,
    pub uninit: Option<unsafe fn(*mut aa_context) -> ()>,
    pub getsize: Option<unsafe fn(*mut aa_context, &mut i64, &mut i64) -> ()>,
    pub setattr: Option<unsafe fn(*mut aa_context, i64) -> ()>,
    pub print: Option<unsafe fn(*mut aa_context, *const std::ffi::c_char) -> ()>,
    pub gotoxy: Option<unsafe fn(*mut aa_context, i64, i64) -> ()>,
    pub flush: Option<unsafe fn(*mut aa_context) -> ()>,
    pub cursormode: Option<unsafe fn(*mut aa_context, i64) -> ()>,
}
#[derive(Copy, Clone)]
pub struct aa_context {
    pub driver: *const aa_driver,
    pub kbddriver: *const aa_kbddriver,
    pub mousedriver: *const aa_mousedriver,
    pub params: aa_hardware_params,
    pub driverparams: aa_hardware_params,
    pub mulx: i64,
    pub muly: i64,
    pub imgwidth: i64,
    pub imgheight: i64,
    pub imagebuffer: *mut std::ffi::c_uchar,
    pub textbuffer: *mut std::ffi::c_uchar,
    pub attrbuffer: *mut std::ffi::c_uchar,
    pub table: *mut u32,
    pub filltable: *mut u32,
    pub parameters: *mut parameters,
    pub cursorx: i64,
    pub cursory: i64,
    pub cursorstate: i64,
    pub mousex: i64,
    pub mousey: i64,
    pub buttons: i64,
    pub mousemode: i64,
    pub resizehandler: Option<unsafe fn(*mut aa_context) -> ()>,
    pub driverdata: *mut std::ffi::c_void,
    pub kbddriverdata: *mut std::ffi::c_void,
    pub mousedriverdata: *mut std::ffi::c_void,
}
#[derive(Copy, Clone)]
pub struct parameters {
    pub p: [i64; 5],
}
#[derive(Copy, Clone)]
pub struct aa_hardware_params {
    pub font: *const aa_font,
    pub supported: i64,
    pub minwidth: i64,
    pub minheight: i64,
    pub maxwidth: i64,
    pub maxheight: i64,
    pub recwidth: i64,
    pub recheight: i64,
    pub mmwidth: i64,
    pub mmheight: i64,
    pub width: i64,
    pub height: i64,
    pub dimmul: f32,
    pub boldmul: f32,
}
#[derive(Copy, Clone)]
pub struct aa_font {
    pub data: *const std::ffi::c_uchar,
    pub height: i64,
    pub name: *const std::ffi::c_char,
    pub shortname: *const std::ffi::c_char,
}
#[derive(Copy, Clone)]
pub struct aa_mousedriver {
    pub shortname: *const std::ffi::c_char,
    pub name: *const std::ffi::c_char,
    pub flags: i64,
    pub init: Option<unsafe fn(*mut aa_context, i64) -> i64>,
    pub uninit: Option<unsafe fn(*mut aa_context) -> ()>,
    pub getmouse: Option<unsafe fn(*mut aa_context, *mut i64, *mut i64, *mut i64) -> ()>,
    pub cursormode: Option<unsafe fn(*mut aa_context, i64) -> ()>,
}
#[derive(Copy, Clone)]
pub struct aa_kbddriver {
    pub shortname: *const std::ffi::c_char,
    pub name: *const std::ffi::c_char,
    pub flags: i64,
    pub init: Option<unsafe fn(*mut aa_context, i64) -> i64>,
    pub uninit: Option<unsafe fn(*mut aa_context) -> ()>,
    pub getkey: Option<unsafe fn(*mut aa_context, i64) -> i64>,
}
#[derive(Copy, Clone)]
pub struct aa_edit {
    pub maxsize: i64,
    pub data: *mut std::ffi::c_char,
    pub cursor: i64,
    pub clearafterpress: i64,
    pub printpos: i64,
    pub x: i64,
    pub y: i64,
    pub size: i64,
    pub c: *mut aa_context,
}
#[derive(Copy, Clone)]
pub struct aa_renderparams {
    pub bright: i64,
    pub contrast: i64,
    pub gamma: std::ffi::c_float,
    pub dither: aa_dithering_mode,
    pub inversion: i64,
    pub randomval: i64,
}
#[derive(Copy, Clone)]
pub struct aa_linkedlist {
    pub text: *mut std::ffi::c_char,
    pub next: *mut aa_linkedlist,
    pub previous: *mut aa_linkedlist,
}
#[derive(Copy, Clone)]
pub struct aa_savedata {
    pub name: *mut std::ffi::c_char,
    pub format: *const aa_format,
    pub file: *mut FILE,
}
#[derive(Copy, Clone)]
pub struct aa_format {
    pub width: i64,
    pub height: i64,
    pub pagewidth: i64,
    pub pageheight: i64,
    pub flags: i64,
    pub supported: i64,
    pub font: *const aa_font,
    pub formatname: *const std::ffi::c_char,
    pub extension: *const std::ffi::c_char,
    pub head: *const std::ffi::c_char,
    pub end: *const std::ffi::c_char,
    pub newline: *const std::ffi::c_char,
    pub prints: [*const std::ffi::c_char; 5],
    pub begin: [*const std::ffi::c_char; 5],
    pub ends: [*const std::ffi::c_char; 5],
    pub conversions: *const *const std::ffi::c_char,
}
