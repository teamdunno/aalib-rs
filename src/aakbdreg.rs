use super::aastructs::*;

unsafe extern "C" {
    fn printf(_: *const std::ffi::c_char, _: ...) -> std::ffi::c_int;
    fn free(_: *mut std::ffi::c_void);
    static kbd_slang_d: aa_kbddriver;
    static kbd_stdin_d: aa_kbddriver;
    static kbd_X11_d: aa_kbddriver;
    static kbd_linux_d: aa_kbddriver;
    static mut aa_kbdrecommended: *mut aa_linkedlist;
    fn aa_getfirst(l: *mut *mut aa_linkedlist) -> *mut std::ffi::c_char;
    fn aa_initkbd(
        context: *mut aa_context,
        drv: *const aa_kbddriver,
        mode: std::ffi::c_int,
    ) -> std::ffi::c_int;
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

pub unsafe extern "C" fn aa_autoinitkbd(
    mut context: *mut aa_context,
    mut mode: std::ffi::c_int,
) -> std::ffi::c_int {
    let mut i: std::ffi::c_int = 0 as std::ffi::c_int;
    let mut ok: std::ffi::c_int = 0 as std::ffi::c_int;
    let mut t: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
    loop {
        t = aa_getfirst(&mut aa_kbdrecommended);
        if t.is_null() {
            break;
        }
        if ok == 0 {
            i = 0 as std::ffi::c_int;
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
    i = 0 as std::ffi::c_int;
    if ok == 0 {
        while !(aa_kbddrivers[i as usize]).is_null() {
            if aa_initkbd(context, aa_kbddrivers[i as usize], mode) != 0 {
                return 1 as std::ffi::c_int;
            }
            i += 1;
            i;
        }
    }
    return ok;
}
