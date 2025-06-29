use super::aain::aa_initmouse;
use super::aarec::aa_getfirst;
use super::aastructs::*;

unsafe extern "C" {
    fn printf(_: *const std::ffi::c_char, _: ...) -> std::ffi::c_int;
    fn free(_: *mut std::ffi::c_void);
    static mouse_gpm_d: aa_mousedriver;
    static mouse_X11_d: aa_mousedriver;
    static mut aa_mouserecommended: *mut aa_linkedlist;
    fn strcmp(_: *const std::ffi::c_char, _: *const std::ffi::c_char) -> std::ffi::c_int;
}

pub static mut aa_mousedrivers: [*const aa_mousedriver; 3] = unsafe {
    [
        &mouse_X11_d as *const aa_mousedriver,
        &mouse_gpm_d as *const aa_mousedriver,
        0 as *const aa_mousedriver,
    ]
};

pub fn aa_autoinitmouse(mut context: *mut aa_context, mut mode: std::ffi::c_int) -> i64 {
    unsafe {
        let mut i = 0;
        let mut ok = 0;
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
                        ok = aa_initmouse(context, aa_mousedrivers[i as usize], mode.into());
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
                if aa_initmouse(context, aa_mousedrivers[i as usize], mode.into()) != 0 {
                    return 1;
                }
                i += 1;
                i;
            }
        }
        return ok;
    }
}
