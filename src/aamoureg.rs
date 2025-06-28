use super::aain::aa_context;

unsafe extern "C" {
    fn printf(_: *const std::ffi::c_char, _: ...) -> std::ffi::c_int;
    fn free(_: *mut std::ffi::c_void);
    static mouse_gpm_d: aa_mousedriver;
    static mouse_X11_d: aa_mousedriver;
    static mut aa_mouserecommended: *mut aa_linkedlist;
    fn aa_getfirst(l: *mut *mut aa_linkedlist) -> *mut std::ffi::c_char;
    fn aa_initmouse(
        c: *mut aa_context,
        d: *const aa_mousedriver,
        mode: std::ffi::c_int,
    ) -> std::ffi::c_int;
    fn strcmp(_: *const std::ffi::c_char, _: *const std::ffi::c_char) -> std::ffi::c_int;
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aa_linkedlist {
    pub text: *mut std::ffi::c_char,
    pub next: *mut aa_linkedlist,
    pub previous: *mut aa_linkedlist,
}

pub static mut aa_mousedrivers: [*const aa_mousedriver; 3] = unsafe {
    [
        &mouse_X11_d as *const aa_mousedriver,
        &mouse_gpm_d as *const aa_mousedriver,
        0 as *const aa_mousedriver,
    ]
};

pub unsafe extern "C" fn aa_autoinitmouse(
    mut context: *mut aa_context,
    mut mode: std::ffi::c_int,
) -> std::ffi::c_int {
    let mut i: std::ffi::c_int = 0 as std::ffi::c_int;
    let mut ok: std::ffi::c_int = 0 as std::ffi::c_int;
    let mut t: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
    loop {
        t = aa_getfirst(&mut aa_mouserecommended);
        if t.is_null() {
            break;
        }
        if ok == 0 {
            i = 0 as std::ffi::c_int;
            while !(aa_mousedrivers[i as usize]).is_null() {
                if strcmp(t, (*aa_mousedrivers[i as usize]).name) == 0
                    || strcmp(t, (*aa_mousedrivers[i as usize]).shortname) == 0
                {
                    ok = aa_initmouse(context, aa_mousedrivers[i as usize], mode);
                    break;
                } else {
                    i += 1;
                    i;
                }
            }
            if (aa_mousedrivers[i as usize]).is_null() {
                printf(
                    b"Driver %s unknown\0" as *const u8 as *const std::ffi::c_char,
                    t,
                );
            }
            free(t as *mut std::ffi::c_void);
        }
    }
    i = 0 as std::ffi::c_int;
    if ok == 0 {
        while !(aa_mousedrivers[i as usize]).is_null() {
            if aa_initmouse(context, aa_mousedrivers[i as usize], mode) != 0 {
                return 1 as std::ffi::c_int;
            }
            i += 1;
            i;
        }
    }
    return ok;
}
