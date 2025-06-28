use super::aastructs::*;

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
