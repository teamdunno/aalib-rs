use super::aarec::{aa_kbdrecommended, aa_mouserecommended, aa_recommendlow};
use super::aastructs::*;
use ::c2rust_bitfields;

unsafe extern "C" {
    static mut stdout: *mut FILE;
    fn fflush(__stream: *mut FILE) -> std::ffi::c_int;
    fn printf(_: *const std::ffi::c_char, _: ...) -> std::ffi::c_int;
    static mut SLtt_Screen_Rows: std::ffi::c_int;
    static mut SLtt_Screen_Cols: std::ffi::c_int;
    static mut SLtt_Use_Ansi_Colors: std::ffi::c_int;
    fn SLtt_get_terminfo();
    fn SLtt_get_screen_size();
    fn SLtt_set_cursor_visibility(_: std::ffi::c_int) -> std::ffi::c_int;
    fn SLtt_set_color(
        _: std::ffi::c_int,
        _: *mut std::ffi::c_char,
        _: *mut std::ffi::c_char,
        _: *mut std::ffi::c_char,
    ) -> std::ffi::c_int;
    fn SLtt_set_mono(
        _: std::ffi::c_int,
        _: *mut std::ffi::c_char,
        _: SLtt_Char_Type,
    ) -> std::ffi::c_int;
    fn SLsmg_gotorc(_: std::ffi::c_int, _: std::ffi::c_int);
    fn SLsmg_set_color(_: SLsmg_Color_Type);
    fn SLsmg_write_string(_: *mut std::ffi::c_char);
    fn SLsmg_refresh();
    fn SLsmg_init_smg() -> std::ffi::c_int;
    fn SLsmg_reset_smg();
    static mut SLsmg_Display_Eight_Bit: std::ffi::c_int;
    static mut gpm_mx: std::ffi::c_int;
    static mut gpm_my: std::ffi::c_int;
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
pub type SLtt_Char_Type = std::ffi::c_ulong;
pub type SLsmg_Color_Type = std::ffi::c_ushort;
pub type aa_attribute = std::ffi::c_uint;
pub const AA_SPECIAL: aa_attribute = 5;
pub const AA_REVERSE: aa_attribute = 4;
pub const AA_BOLDFONT: aa_attribute = 3;
pub const AA_BOLD: aa_attribute = 2;
pub const AA_DIM: aa_attribute = 1;
pub const AA_NORMAL: aa_attribute = 0;
#[unsafe(no_mangle)]
pub static mut slang_d: aa_driver = unsafe {
    {
        let mut init = aa_driver {
            shortname: b"slang\0" as *const u8 as *const std::ffi::c_char,
            name: b"Slang driver 1.0\0" as *const u8 as *const std::ffi::c_char,
            init: Some(slang_init),
            uninit: Some(slang_uninit),
            getsize: Some(slang_getsize),
            setattr: Some(slang_setattr),
            print: Some(slang_print),
            gotoxy: Some(slang_gotoxy),
            flush: Some(slang_flush),
            cursormode: Some(slang_cursor),
        };
        init
    }
};

pub static mut __slang_is_up: std::ffi::c_int = 0 as std::ffi::c_int;

pub static mut __resized_slang: std::ffi::c_int = 0 as std::ffi::c_int;
static mut uninitslang: std::ffi::c_int = 0;
unsafe fn slang_init(
    mut p: *const aa_hardware_params,
    mut none: *const std::ffi::c_void,
    mut dest: *mut aa_hardware_params,
    mut params: *mut *mut std::ffi::c_void,
) -> i64 {
    let mut def: aa_hardware_params = {
        let mut init = aa_hardware_params {
            font: 0 as *const aa_font,
            supported: 1 | 4 | 16 | 8 | 2,
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
    fflush(stdout);
    if __slang_is_up == 0 {
        SLtt_get_terminfo();
        __slang_is_up = 1;
        uninitslang = 1;
    }
    if SLsmg_init_smg() != 0 {
        return 0;
    }
    if SLtt_Use_Ansi_Colors != 0 {
        (*dest).supported &= !8;
    }
    SLsmg_Display_Eight_Bit = 128;
    (*dest).supported |= 256;
    aa_recommendlow(
        &mut aa_mouserecommended,
        b"gpm\0" as *const u8 as *const std::ffi::c_char,
    );
    aa_recommendlow(
        &mut aa_kbdrecommended,
        b"linux\0" as *const u8 as *const std::ffi::c_char,
    );
    aa_recommendlow(
        &mut aa_kbdrecommended,
        b"slang\0" as *const u8 as *const std::ffi::c_char,
    );
    return 1;
}
unsafe fn slang_uninit(mut c: *mut aa_context) {
    SLsmg_reset_smg();
    if uninitslang != 0 {
        uninitslang = 0 as std::ffi::c_int;
        __slang_is_up = 0 as std::ffi::c_int;
    }
}
unsafe fn slang_getsize(mut c: *mut aa_context, mut width: &mut i64, mut height: &mut i64) {
    SLtt_get_screen_size();
    SLsmg_reset_smg();
    if SLsmg_init_smg() != 0 as std::ffi::c_int {
        printf(b"Internal error!\n\0" as *const u8 as *const std::ffi::c_char);
    }
    SLtt_set_mono(
        AA_NORMAL as std::ffi::c_int,
        b"normal\0" as *const u8 as *const std::ffi::c_char as *mut std::ffi::c_char,
        0 as std::ffi::c_int as SLtt_Char_Type,
    );
    SLtt_set_mono(
        AA_BOLD as std::ffi::c_int,
        b"bold\0" as *const u8 as *const std::ffi::c_char as *mut std::ffi::c_char,
        0x1000000 as std::ffi::c_ulong,
    );
    SLtt_set_mono(
        AA_DIM as std::ffi::c_int,
        b"dim\0" as *const u8 as *const std::ffi::c_char as *mut std::ffi::c_char,
        0x10000000 as std::ffi::c_ulong,
    );
    SLtt_set_mono(
        AA_REVERSE as std::ffi::c_int,
        b"reverse\0" as *const u8 as *const std::ffi::c_char as *mut std::ffi::c_char,
        0x8000000 as std::ffi::c_ulong,
    );
    SLtt_set_mono(
        AA_SPECIAL as std::ffi::c_int,
        b"special\0" as *const u8 as *const std::ffi::c_char as *mut std::ffi::c_char,
        0 as std::ffi::c_int as SLtt_Char_Type,
    );
    SLtt_set_mono(
        AA_BOLDFONT as std::ffi::c_int,
        b"boldfont\0" as *const u8 as *const std::ffi::c_char as *mut std::ffi::c_char,
        0x1000000 as std::ffi::c_ulong,
    );
    SLtt_set_color(
        AA_NORMAL as std::ffi::c_int,
        b"normal\0" as *const u8 as *const std::ffi::c_char as *mut std::ffi::c_char,
        b"lightgray\0" as *const u8 as *const std::ffi::c_char as *mut std::ffi::c_char,
        b"black\0" as *const u8 as *const std::ffi::c_char as *mut std::ffi::c_char,
    );
    SLtt_set_color(
        AA_BOLD as std::ffi::c_int,
        b"bold\0" as *const u8 as *const std::ffi::c_char as *mut std::ffi::c_char,
        b"white\0" as *const u8 as *const std::ffi::c_char as *mut std::ffi::c_char,
        b"black\0" as *const u8 as *const std::ffi::c_char as *mut std::ffi::c_char,
    );
    SLtt_set_color(
        AA_DIM as std::ffi::c_int,
        b"dim\0" as *const u8 as *const std::ffi::c_char as *mut std::ffi::c_char,
        b"gray\0" as *const u8 as *const std::ffi::c_char as *mut std::ffi::c_char,
        b"black\0" as *const u8 as *const std::ffi::c_char as *mut std::ffi::c_char,
    );
    SLtt_set_color(
        AA_REVERSE as std::ffi::c_int,
        b"bold\0" as *const u8 as *const std::ffi::c_char as *mut std::ffi::c_char,
        b"black\0" as *const u8 as *const std::ffi::c_char as *mut std::ffi::c_char,
        b"lightgray\0" as *const u8 as *const std::ffi::c_char as *mut std::ffi::c_char,
    );
    SLtt_set_color(
        AA_SPECIAL as std::ffi::c_int,
        b"dim\0" as *const u8 as *const std::ffi::c_char as *mut std::ffi::c_char,
        b"lightgray\0" as *const u8 as *const std::ffi::c_char as *mut std::ffi::c_char,
        b"blue\0" as *const u8 as *const std::ffi::c_char as *mut std::ffi::c_char,
    );
    SLtt_set_color(
        AA_BOLDFONT as std::ffi::c_int,
        b"bold\0" as *const u8 as *const std::ffi::c_char as *mut std::ffi::c_char,
        b"white\0" as *const u8 as *const std::ffi::c_char as *mut std::ffi::c_char,
        b"black\0" as *const u8 as *const std::ffi::c_char as *mut std::ffi::c_char,
    );
    *width = SLtt_Screen_Cols as i64;
    *height = SLtt_Screen_Rows as i64;
    gpm_mx = *width as i32;
    gpm_my = *height as i32;
}
unsafe fn slang_setattr(mut c: *mut aa_context, mut attr: i64) {
    SLsmg_set_color(attr as SLsmg_Color_Type);
}
unsafe fn slang_print(mut c: *mut aa_context, mut text: *const std::ffi::c_char) {
    SLsmg_write_string(text as *mut std::ffi::c_char);
}
unsafe fn slang_flush(mut c: *mut aa_context) {
    SLsmg_refresh();
}
unsafe fn slang_gotoxy(mut c: *mut aa_context, mut x: i64, mut y: i64) {
    SLsmg_gotorc(y as std::ffi::c_int, x as std::ffi::c_int);
}
unsafe fn slang_cursor(mut c: *mut aa_context, mut mode: i64) {
    SLtt_set_cursor_visibility(mode as std::ffi::c_int);
}
