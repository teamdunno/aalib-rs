use super::aain::{aa_context, aa_driver, aa_hardware_params};
use super::aalib::aa_init;
use super::aarec::aa_displayrecommended;
use super::aarec::{aa_getfirst, aa_linkedlist};
unsafe extern "C" {
    fn printf(_: *const std::ffi::c_char, _: ...) -> std::ffi::c_int;
    fn free(_: *mut std::ffi::c_void);
    static linux_d: aa_driver;
    static slang_d: aa_driver;
    static stdout_d: aa_driver;
    static stderr_d: aa_driver;
    static X11_d: aa_driver;
    fn strcmp(_: *const std::ffi::c_char, _: *const std::ffi::c_char) -> std::ffi::c_int;
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
pub static mut aa_drivers: [*const aa_driver; 6] = unsafe {
    [
        &X11_d as *const aa_driver,
        &linux_d as *const aa_driver,
        &slang_d as *const aa_driver,
        &stdout_d as *const aa_driver,
        &stderr_d as *const aa_driver,
        0 as *const aa_driver,
    ]
};

pub unsafe extern "C" fn aa_autoinit(mut params: *const aa_hardware_params) -> *mut aa_context {
    let mut context: *mut aa_context = 0 as *mut aa_context;
    let mut i: std::ffi::c_int = 0 as std::ffi::c_int;
    let mut t: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
    loop {
        t = aa_getfirst(&mut aa_displayrecommended);
        if t.is_null() {
            break;
        }
        if context.is_null() {
            i = 0 as std::ffi::c_int;
            while !(aa_drivers[i as usize]).is_null() {
                if strcmp(t, (*aa_drivers[i as usize]).name) == 0
                    || strcmp(t, (*aa_drivers[i as usize]).shortname) == 0
                {
                    context = aa_init(aa_drivers[i as usize], params, 0 as *const std::ffi::c_void);
                    break;
                } else {
                    i += 1;
                    i;
                }
            }
            if (aa_drivers[i as usize]).is_null() {
                printf(
                    b"Driver %s unknown\0" as *const u8 as *const std::ffi::c_char,
                    t,
                );
            }
            free(t as *mut std::ffi::c_void);
        }
    }
    i = 0 as std::ffi::c_int;
    while context.is_null() {
        if (aa_drivers[i as usize]).is_null() {
            return 0 as *mut aa_context;
        }
        context = aa_init(aa_drivers[i as usize], params, 0 as *const std::ffi::c_void);
        i += 1;
        i;
    }
    return context;
}
