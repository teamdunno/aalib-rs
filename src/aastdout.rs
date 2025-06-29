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
unsafe fn stdout_init(
    p: *const aa_hardware_params,
    none: *const std::ffi::c_void,
    dest: *mut aa_hardware_params,
    n: *mut *mut std::ffi::c_void,
) -> i64 { unsafe {
    static mut def: aa_hardware_params = {
        let init = aa_hardware_params {
            font: 0 as *const aa_font,
            supported: 1 | (128 | 256),
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
    return 1;
}}
unsafe fn stdout_uninit(_: *mut aa_context) {}
unsafe fn stdout_getsize(_: *mut aa_context, _: &mut i64, _: &mut i64) {}
unsafe fn stdout_flush(c: *mut aa_context) { unsafe {
    let mut x = 0;
    let mut y = 0;
    y = 0;
    while y < (*c).params.height {
        x = 0;
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
}}
unsafe fn stdout_gotoxy(_: *mut aa_context, _: i64, _: i64) {}

#[unsafe(no_mangle)]
pub static mut stdout_d: aa_driver = unsafe {
    {
        let init = aa_driver {
            shortname: b"stdout\0" as *const u8 as *const std::ffi::c_char,
            name: b"Standard output driver\0" as *const u8 as *const std::ffi::c_char,
            init: Some(stdout_init),
            uninit: Some(stdout_uninit),
            getsize: Some(stdout_getsize),
            setattr: None,
            print: None,
            gotoxy: Some(stdout_gotoxy),
            flush: Some(stdout_flush),
            cursormode: None,
        };
        init
    }
};
unsafe fn stderr_flush(c: *mut aa_context) { unsafe {
    let mut x = 0;
    let mut y = 0;
    y = 0;
    while y < (*c).params.height {
        x = 0;
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
}}

#[unsafe(no_mangle)]
pub static mut stderr_d: aa_driver = unsafe {
    {
        let init = aa_driver {
            shortname: b"stderr\0" as *const u8 as *const std::ffi::c_char,
            name: b"Standard error driver\0" as *const u8 as *const std::ffi::c_char,
            init: Some(stdout_init),
            uninit: Some(stdout_uninit),
            getsize: Some(stdout_getsize),
            setattr: None,
            print: None,
            gotoxy: Some(stdout_gotoxy),
            flush: Some(stderr_flush),
            cursormode: None,
        };
        init
    }
};
