use super::aastructs::*;

unsafe fn mem_init(
    p: *const aa_hardware_params,
    none: *const std::ffi::c_void,
    dest: *mut aa_hardware_params,
    params: *mut *mut std::ffi::c_void,
) -> i64 { unsafe {
    let def: aa_hardware_params = aa_hardware_params {
        font: 0 as *const aa_font,
        supported: 2 | 16 | 1 | 4 | (128 | 256),
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
    *dest = def;
    return 1;
}}
unsafe fn mem_uninit(_: *mut aa_context) {}
unsafe fn mem_getsize(_: *mut aa_context, _: &mut i64, _: &mut i64) {}
unsafe fn mem_flush(_: *mut aa_context) {}
unsafe fn mem_gotoxy(_: *mut aa_context, _: i64, _: i64) {}

pub static mut mem_d: aa_driver = unsafe {
    {
        let init = aa_driver {
            shortname: b"mem\0" as *const u8 as *const std::ffi::c_char,
            name: b"Dummy memory driver 1.0\0" as *const u8 as *const std::ffi::c_char,
            init: Some(mem_init),
            uninit: Some(mem_uninit),
            getsize: Some(mem_getsize),
            setattr: None,
            print: None,
            gotoxy: Some(mem_gotoxy),
            flush: Some(mem_flush),
            cursormode: None,
        };
        init
    }
};
