use super::aastructs::*;
use ::c2rust_bitfields;
unsafe extern "C" {
    fn exit(_: std::ffi::c_int) -> !;
    fn fopen(_: *const std::ffi::c_char, _: *const std::ffi::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const std::ffi::c_char, _: ...) -> std::ffi::c_int;
    fn printf(_: *const std::ffi::c_char, _: ...) -> std::ffi::c_int;
    static aa_help: *const std::ffi::c_char;
    static mut aa_defparams: aa_hardware_params;
    fn aa_currentfont(a: *mut aa_context) -> *const aa_font;
    fn aa_autoinit(params: *const aa_hardware_params) -> *mut aa_context;
    fn aa_close(c: *mut aa_context);
    fn aa_parseoptions(
        p: *mut aa_hardware_params,
        r: *mut aa_renderparams,
        argc: *mut std::ffi::c_int,
        argv: *mut *mut std::ffi::c_char,
    ) -> std::ffi::c_int;
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
pub type aa_dithering_mode = std::ffi::c_uint;
pub const AA_DITHERTYPES: aa_dithering_mode = 3;
pub const AA_FLOYD_S: aa_dithering_mode = 2;
pub const AA_ERRORDISTRIB: aa_dithering_mode = 1;
pub const AA_NONE: aa_dithering_mode = 0;
unsafe fn main_0(
    mut argc: std::ffi::c_int,
    mut argv: *mut *mut std::ffi::c_char,
) -> std::ffi::c_int {
    let mut c: *mut aa_context = 0 as *mut aa_context;
    let mut font: *const aa_font = 0 as *const aa_font;
    let mut i: std::ffi::c_int = 0;
    let mut f: *mut FILE = 0 as *mut FILE;
    if aa_parseoptions(
        0 as *mut aa_hardware_params,
        0 as *mut aa_renderparams,
        &mut argc,
        argv,
    ) == 0
        || argc != 5 as std::ffi::c_int
    {
        printf(
            b"Usage: %s [options] file_name variable_name long_name short_name\n%s\0" as *const u8
                as *const std::ffi::c_char,
            *argv.offset(0 as std::ffi::c_int as isize),
            aa_help,
        );
        exit(1 as std::ffi::c_int);
    }
    c = aa_autoinit(&mut aa_defparams);
    f = fopen(
        *argv.offset(1 as std::ffi::c_int as isize),
        b"w\0" as *const u8 as *const std::ffi::c_char,
    );
    if f.is_null() {
        aa_close(c);
        printf(b"Can not open output file\n\0" as *const u8 as *const std::ffi::c_char);
    }
    fprintf(
        f,
        b"#include \"aalib.h\"\nstatic unsigned char %sdata[] =\n{\0" as *const u8
            as *const std::ffi::c_char,
        *argv.offset(2 as std::ffi::c_int as isize),
    );
    font = aa_currentfont(c);
    i = 0 as std::ffi::c_int;
    while i < (*font).height * 256 as std::ffi::c_int {
        if i % (*font).height == 0 {
            fprintf(f, b"\n\t\0" as *const u8 as *const std::ffi::c_char);
        }
        fprintf(
            f,
            b"0x%02x, \0" as *const u8 as *const std::ffi::c_char,
            *((*font).data).offset(i as isize) as std::ffi::c_int,
        );
        i += 1;
        i;
    }
    fprintf(
        f,
        b"};\n\nstruct aa_font %s =\n\t{%sdata, %i, \"%s\", \"%s\"};\n\0" as *const u8
            as *const std::ffi::c_char,
        *argv.offset(2 as std::ffi::c_int as isize),
        *argv.offset(2 as std::ffi::c_int as isize),
        (*font).height,
        *argv.offset(3 as std::ffi::c_int as isize),
        *argv.offset(4 as std::ffi::c_int as isize),
    );
    aa_close(c);
    return 1 as std::ffi::c_int;
}
pub fn main() {
    let mut args: Vec<*mut std::ffi::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0(
            (args.len() - 1) as std::ffi::c_int,
            args.as_mut_ptr() as *mut *mut std::ffi::c_char,
        ) as i32)
    }
}
