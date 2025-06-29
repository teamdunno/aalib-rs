use super::aain::aa_initkbd;
use super::aarec::{aa_getfirst, aa_kbdrecommended};
use super::aastructs::*;

unsafe extern "C" {
    fn printf(_: *const std::ffi::c_char, _: ...) -> std::ffi::c_int;
    fn free(_: *mut std::ffi::c_void);
    static kbd_slang_d: aa_kbddriver;
    static kbd_stdin_d: aa_kbddriver;
    static kbd_X11_d: aa_kbddriver;
    static kbd_linux_d: aa_kbddriver;
    fn strcmp(_: *const std::ffi::c_char, _: *const std::ffi::c_char) -> std::ffi::c_int;
}
pub static mut aa_kbddrivers: [*const aa_kbddriver; 5] = unsafe {
    [
        &kbd_linux_d as *const aa_kbddriver,
        &kbd_X11_d as *const aa_kbddriver,
        &kbd_slang_d as *const aa_kbddriver,
        &kbd_stdin_d as *const aa_kbddriver,
        0 as *const aa_kbddriver,
    ]
};

pub fn aa_autoinitkbd(context: *mut aa_context, mode: i64) -> i64 {
    unsafe {
        let mut i = 0;
        let mut ok = 0;
        let mut t: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
        loop {
            t = aa_getfirst(&mut aa_kbdrecommended);
            if t.is_null() {
                break;
            }
            if ok == 0 {
                i = 0;
                while !(aa_kbddrivers[i as usize]).is_null() {
                    if strcmp(t, (*aa_kbddrivers[i as usize]).name) == 0
                        || strcmp(t, (*aa_kbddrivers[i as usize]).shortname) == 0
                    {
                        ok = aa_initkbd(context, aa_kbddrivers[i as usize], mode);
                        break;
                    } else {
                        i += 1;
                        i;
                    }
                }
                if (aa_kbddrivers[i as usize]).is_null() {
                    printf(
                        b"Driver %s unknown\0" as *const u8 as *const std::ffi::c_char,
                        t,
                    );
                }
                free(t as *mut std::ffi::c_void);
            }
        }
        i = 0;
        if ok == 0 {
            while !(aa_kbddrivers[i as usize]).is_null() {
                if aa_initkbd(context, aa_kbddrivers[i as usize], mode) != 0 {
                    return 1;
                }
                i += 1;
            }
        }
        return ok as i64;
    }
}
