use super::aastructs::*;
use ::c2rust_bitfields;
unsafe extern "C" {
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fflush(__stream: *mut FILE) -> std::ffi::c_int;
    fn putc(__c: std::ffi::c_int, __stream: *mut FILE) -> std::ffi::c_int;
}
pub type __off_t = std::ffi::c_long;
pub type __off64_t = std::ffi::c_long;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: std::ffi::c_int,
    pub _IO_read_ptr: *mut std::ffi::c_char,
    pub _IO_read_end: *mut std::ffi::c_char,
    pub _IO_read_base: *mut std::ffi::c_char,
    pub _IO_write_base: *mut std::ffi::c_char,
    pub _IO_write_ptr: *mut std::ffi::c_char,
    pub _IO_write_end: *mut std::ffi::c_char,
    pub _IO_buf_base: *mut std::ffi::c_char,
    pub _IO_buf_end: *mut std::ffi::c_char,
    pub _IO_save_base: *mut std::ffi::c_char,
    pub _IO_backup_base: *mut std::ffi::c_char,
    pub _IO_save_end: *mut std::ffi::c_char,
    pub _chain: *mut _IO_FILE,
    pub _fileno: std::ffi::c_int,
    #[bitfield(name = "_flags2", ty = "std::ffi::c_int", bits = "0..=23")]
    pub _flags2: [u8; 3],
    pub _short_backupbuf: [std::ffi::c_char; 1],
    pub _old_offset: __off_t,
    pub _cur_column: std::ffi::c_ushort,
    pub _vtable_offset: std::ffi::c_schar,
    pub _shortbuf: [std::ffi::c_char; 1],
    pub _lock: *mut std::ffi::c_void,
    pub _offset: __off64_t,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut std::ffi::c_void,
    pub _prevchain: *mut *mut _IO_FILE,
    pub _mode: std::ffi::c_int,
    pub _unused2: [std::ffi::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
unsafe extern "C" fn stdout_init(
    mut p: *const aa_hardware_params,
    mut none: *const std::ffi::c_void,
    mut dest: *mut aa_hardware_params,
    mut n: *mut *mut std::ffi::c_void,
) -> std::ffi::c_int {
    static mut def: aa_hardware_params = {
        let mut init = aa_hardware_params {
            font: 0 as *const aa_font,
            supported: 1 as std::ffi::c_int | (128 as std::ffi::c_int | 256 as std::ffi::c_int),
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
unsafe extern "C" fn stdout_uninit(mut c: *mut aa_context) {}
unsafe extern "C" fn stdout_getsize(
    mut c: *mut aa_context,
    mut width: *mut std::ffi::c_int,
    mut height: *mut std::ffi::c_int,
) {
}
unsafe extern "C" fn stdout_flush(mut c: *mut aa_context) {
    let mut x: std::ffi::c_int = 0;
    let mut y: std::ffi::c_int = 0;
    y = 0 as std::ffi::c_int;
    while y < (*c).params.height {
        x = 0 as std::ffi::c_int;
        while x < (*c).params.width {
            putc(
                *((*c).textbuffer).offset((x + y * (*c).params.width) as isize) as std::ffi::c_int,
                stdout,
            );
            x += 1;
            x;
        }
        putc('\n' as i32, stdout);
        y += 1;
        y;
    }
    putc('\u{c}' as i32, stdout);
    putc('\n' as i32, stdout);
    fflush(stdout);
}
unsafe extern "C" fn stdout_gotoxy(
    mut c: *mut aa_context,
    mut x: std::ffi::c_int,
    mut y: std::ffi::c_int,
) {
}

#[unsafe(no_mangle)]
pub static mut stdout_d: aa_driver = unsafe {
    {
        let mut init = aa_driver {
            shortname: b"stdout\0" as *const u8 as *const std::ffi::c_char,
            name: b"Standard output driver\0" as *const u8 as *const std::ffi::c_char,
            init: Some(
                stdout_init
                    as unsafe extern "C" fn(
                        *const aa_hardware_params,
                        *const std::ffi::c_void,
                        *mut aa_hardware_params,
                        *mut *mut std::ffi::c_void,
                    ) -> std::ffi::c_int,
            ),
            uninit: Some(stdout_uninit as unsafe extern "C" fn(*mut aa_context) -> ()),
            getsize: Some(
                stdout_getsize
                    as unsafe extern "C" fn(
                        *mut aa_context,
                        *mut std::ffi::c_int,
                        *mut std::ffi::c_int,
                    ) -> (),
            ),
            setattr: None,
            print: None,
            gotoxy: Some(
                stdout_gotoxy
                    as unsafe extern "C" fn(
                        *mut aa_context,
                        std::ffi::c_int,
                        std::ffi::c_int,
                    ) -> (),
            ),
            flush: Some(stdout_flush as unsafe extern "C" fn(*mut aa_context) -> ()),
            cursormode: None,
        };
        init
    }
};
unsafe extern "C" fn stderr_flush(mut c: *mut aa_context) {
    let mut x: std::ffi::c_int = 0;
    let mut y: std::ffi::c_int = 0;
    y = 0 as std::ffi::c_int;
    while y < (*c).params.height {
        x = 0 as std::ffi::c_int;
        while x < (*c).params.width {
            putc(
                *((*c).textbuffer).offset((x + y * (*c).params.width) as isize) as std::ffi::c_int,
                stderr,
            );
            x += 1;
            x;
        }
        putc('\n' as i32, stderr);
        y += 1;
        y;
    }
    putc('\u{c}' as i32, stderr);
    putc('\n' as i32, stderr);
    fflush(stderr);
}

#[unsafe(no_mangle)]
pub static mut stderr_d: aa_driver = unsafe {
    {
        let mut init = aa_driver {
            shortname: b"stderr\0" as *const u8 as *const std::ffi::c_char,
            name: b"Standard error driver\0" as *const u8 as *const std::ffi::c_char,
            init: Some(
                stdout_init
                    as unsafe extern "C" fn(
                        *const aa_hardware_params,
                        *const std::ffi::c_void,
                        *mut aa_hardware_params,
                        *mut *mut std::ffi::c_void,
                    ) -> std::ffi::c_int,
            ),
            uninit: Some(stdout_uninit as unsafe extern "C" fn(*mut aa_context) -> ()),
            getsize: Some(
                stdout_getsize
                    as unsafe extern "C" fn(
                        *mut aa_context,
                        *mut std::ffi::c_int,
                        *mut std::ffi::c_int,
                    ) -> (),
            ),
            setattr: None,
            print: None,
            gotoxy: Some(
                stdout_gotoxy
                    as unsafe extern "C" fn(
                        *mut aa_context,
                        std::ffi::c_int,
                        std::ffi::c_int,
                    ) -> (),
            ),
            flush: Some(stderr_flush as unsafe extern "C" fn(*mut aa_context) -> ()),
            cursormode: None,
        };
        init
    }
};
