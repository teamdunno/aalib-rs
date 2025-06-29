use super::aalib::aa_init;
use super::aarec::aa_displayrecommended;
use super::aarec::aa_getfirst;
use super::aastructs::*;
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

pub fn aa_autoinit(params: *const aa_hardware_params) -> *mut aa_context {
    unsafe {
        let mut context: *mut aa_context = 0 as *mut aa_context;
        let mut i = 0;
        let mut t: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
        loop {
            t = aa_getfirst(&mut aa_displayrecommended);
            if t.is_null() {
                break;
            }
            if context.is_null() {
                i = 0;
                while !(aa_drivers[i as usize]).is_null() {
                    if strcmp(t, (*aa_drivers[i as usize]).name) == 0
                        || strcmp(t, (*aa_drivers[i as usize]).shortname) == 0
                    {
                        context =
                            aa_init(aa_drivers[i as usize], params, 0 as *const std::ffi::c_void);
                        break;
                    } else {
                        i += 1;
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
        i = 0;
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
}
