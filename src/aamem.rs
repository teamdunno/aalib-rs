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
pub struct aa_context {
    pub driver: *const aa_driver,
    pub kbddriver: *const aa_kbddriver,
    pub mousedriver: *const aa_mousedriver,
    pub params: aa_hardware_params,
    pub driverparams: aa_hardware_params,
    pub mulx: std::ffi::c_int,
    pub muly: std::ffi::c_int,
    pub imgwidth: std::ffi::c_int,
    pub imgheight: std::ffi::c_int,
    pub imagebuffer: *mut std::ffi::c_uchar,
    pub textbuffer: *mut std::ffi::c_uchar,
    pub attrbuffer: *mut std::ffi::c_uchar,
    pub table: *mut std::ffi::c_ushort,
    pub filltable: *mut std::ffi::c_ushort,
    pub parameters: *mut parameters,
    pub cursorx: std::ffi::c_int,
    pub cursory: std::ffi::c_int,
    pub cursorstate: std::ffi::c_int,
    pub mousex: std::ffi::c_int,
    pub mousey: std::ffi::c_int,
    pub buttons: std::ffi::c_int,
    pub mousemode: std::ffi::c_int,
    pub resizehandler: Option<unsafe extern "C" fn(*mut aa_context) -> ()>,
    pub driverdata: *mut std::ffi::c_void,
    pub kbddriverdata: *mut std::ffi::c_void,
    pub mousedriverdata: *mut std::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct parameters {
    pub p: [std::ffi::c_uint; 5],
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
unsafe extern "C" fn mem_init(
    mut p: *const aa_hardware_params,
    mut none: *const std::ffi::c_void,
    mut dest: *mut aa_hardware_params,
    mut params: *mut *mut std::ffi::c_void,
) -> std::ffi::c_int {
    let mut def: aa_hardware_params = {
        let mut init = aa_hardware_params {
            font: 0 as *const aa_font,
            supported: 2 as std::ffi::c_int
                | 16 as std::ffi::c_int
                | 1 as std::ffi::c_int
                | 4 as std::ffi::c_int
                | (128 as std::ffi::c_int | 256 as std::ffi::c_int),
            minwidth: 0,
            minheight: 0,
            maxwidth: 0,
            maxheight: 0,
            recwidth: 0,
            recheight: 0,
            mmwidth: 0,
            mmheight: 0,
            width: 0,
            height: 0,
            dimmul: 0.,
            boldmul: 0.,
        };
        init
    };
    *dest = def;
    return 1 as std::ffi::c_int;
}
unsafe extern "C" fn mem_uninit(mut c: *mut aa_context) {}
unsafe extern "C" fn mem_getsize(
    mut c: *mut aa_context,
    mut width: *mut std::ffi::c_int,
    mut height: *mut std::ffi::c_int,
) {
}
unsafe extern "C" fn mem_flush(mut c: *mut aa_context) {}
unsafe extern "C" fn mem_gotoxy(
    mut c: *mut aa_context,
    mut x: std::ffi::c_int,
    mut y: std::ffi::c_int,
) {
}

pub static mut mem_d: aa_driver = unsafe {
    {
        let mut init = aa_driver {
            shortname: b"mem\0" as *const u8 as *const std::ffi::c_char,
            name: b"Dummy memory driver 1.0\0" as *const u8 as *const std::ffi::c_char,
            init: Some(
                mem_init
                    as unsafe extern "C" fn(
                        *const aa_hardware_params,
                        *const std::ffi::c_void,
                        *mut aa_hardware_params,
                        *mut *mut std::ffi::c_void,
                    ) -> std::ffi::c_int,
            ),
            uninit: Some(mem_uninit as unsafe extern "C" fn(*mut aa_context) -> ()),
            getsize: Some(
                mem_getsize
                    as unsafe extern "C" fn(
                        *mut aa_context,
                        *mut std::ffi::c_int,
                        *mut std::ffi::c_int,
                    ) -> (),
            ),
            setattr: None,
            print: None,
            gotoxy: Some(
                mem_gotoxy
                    as unsafe extern "C" fn(
                        *mut aa_context,
                        std::ffi::c_int,
                        std::ffi::c_int,
                    ) -> (),
            ),
            flush: Some(mem_flush as unsafe extern "C" fn(*mut aa_context) -> ()),
            cursormode: None,
        };
        init
    }
};
