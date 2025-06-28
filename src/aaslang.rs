use super::aarec::{aa_kbdrecommended, aa_linkedlist, aa_mouserecommended, aa_recommendlow};
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
pub struct aa_context {
    pub driver: *const aa_driver,
    pub kbddriver: *const aa_kbddriver,
    pub mousedriver: *const aa_mousedriver,
    pub params: aa_hardware_params,
    pub driverparams: aa_hardware_params,
    pub mulx: std::ffi::c_int,
    pub muly: std::ffi::c_int,
    pub imgwidth: std::ffi::c_int,
    pub imgheight: std::ffi::c_int,
    pub imagebuffer: *mut std::ffi::c_uchar,
    pub textbuffer: *mut std::ffi::c_uchar,
    pub attrbuffer: *mut std::ffi::c_uchar,
    pub table: *mut std::ffi::c_ushort,
    pub filltable: *mut std::ffi::c_ushort,
    pub parameters: *mut parameters,
    pub cursorx: std::ffi::c_int,
    pub cursory: std::ffi::c_int,
    pub cursorstate: std::ffi::c_int,
    pub mousex: std::ffi::c_int,
    pub mousey: std::ffi::c_int,
    pub buttons: std::ffi::c_int,
    pub mousemode: std::ffi::c_int,
    pub resizehandler: Option<unsafe extern "C" fn(*mut aa_context) -> ()>,
    pub driverdata: *mut std::ffi::c_void,
    pub kbddriverdata: *mut std::ffi::c_void,
    pub mousedriverdata: *mut std::ffi::c_void,
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

#[unsafe(no_mangle)]
pub static mut slang_d: aa_driver = unsafe {
    {
        let mut init = aa_driver {
            shortname: b"slang\0" as *const u8 as *const std::ffi::c_char,
            name: b"Slang driver 1.0\0" as *const u8 as *const std::ffi::c_char,
            init: Some(
                slang_init
                    as unsafe extern "C" fn(
                        *const aa_hardware_params,
                        *const std::ffi::c_void,
                        *mut aa_hardware_params,
                        *mut *mut std::ffi::c_void,
                    ) -> std::ffi::c_int,
            ),
            uninit: Some(slang_uninit as unsafe extern "C" fn(*mut aa_context) -> ()),
            getsize: Some(
                slang_getsize
                    as unsafe extern "C" fn(
                        *mut aa_context,
                        *mut std::ffi::c_int,
                        *mut std::ffi::c_int,
                    ) -> (),
            ),
            setattr: Some(
                slang_setattr as unsafe extern "C" fn(*mut aa_context, std::ffi::c_int) -> (),
            ),
            print: Some(
                slang_print as unsafe extern "C" fn(*mut aa_context, *const std::ffi::c_char) -> (),
            ),
            gotoxy: Some(
                slang_gotoxy
                    as unsafe extern "C" fn(
                        *mut aa_context,
                        std::ffi::c_int,
                        std::ffi::c_int,
                    ) -> (),
            ),
            flush: Some(slang_flush as unsafe extern "C" fn(*mut aa_context) -> ()),
            cursormode: Some(
                slang_cursor as unsafe extern "C" fn(*mut aa_context, std::ffi::c_int) -> (),
            ),
        };
        init
    }
};

pub static mut __slang_is_up: std::ffi::c_int = 0 as std::ffi::c_int;

pub static mut __resized_slang: std::ffi::c_int = 0 as std::ffi::c_int;
static mut uninitslang: std::ffi::c_int = 0;
unsafe extern "C" fn slang_init(
    mut p: *const aa_hardware_params,
    mut none: *const std::ffi::c_void,
    mut dest: *mut aa_hardware_params,
    mut params: *mut *mut std::ffi::c_void,
) -> std::ffi::c_int {
    let mut def: aa_hardware_params = {
        let mut init = aa_hardware_params {
            font: 0 as *const aa_font,
            supported: 1 as std::ffi::c_int
                | 4 as std::ffi::c_int
                | 16 as std::ffi::c_int
                | 8 as std::ffi::c_int
                | 2 as std::ffi::c_int,
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
        __slang_is_up = 1 as std::ffi::c_int;
        uninitslang = 1 as std::ffi::c_int;
    }
    if SLsmg_init_smg() != 0 as std::ffi::c_int {
        return 0 as std::ffi::c_int;
    }
    if SLtt_Use_Ansi_Colors != 0 {
        (*dest).supported &= !(8 as std::ffi::c_int);
    }
    SLsmg_Display_Eight_Bit = 128 as std::ffi::c_int;
    (*dest).supported |= 256 as std::ffi::c_int;
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
    return 1 as std::ffi::c_int;
}
unsafe extern "C" fn slang_uninit(mut c: *mut aa_context) {
    SLsmg_reset_smg();
    if uninitslang != 0 {
        uninitslang = 0 as std::ffi::c_int;
        __slang_is_up = 0 as std::ffi::c_int;
    }
}
unsafe extern "C" fn slang_getsize(
    mut c: *mut aa_context,
    mut width: *mut std::ffi::c_int,
    mut height: *mut std::ffi::c_int,
) {
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
    *width = SLtt_Screen_Cols;
    *height = SLtt_Screen_Rows;
    gpm_mx = *width;
    gpm_my = *height;
}
unsafe extern "C" fn slang_setattr(mut c: *mut aa_context, mut attr: std::ffi::c_int) {
    SLsmg_set_color(attr as SLsmg_Color_Type);
}
unsafe extern "C" fn slang_print(mut c: *mut aa_context, mut text: *const std::ffi::c_char) {
    SLsmg_write_string(text as *mut std::ffi::c_char);
}
unsafe extern "C" fn slang_flush(mut c: *mut aa_context) {
    SLsmg_refresh();
}
unsafe extern "C" fn slang_gotoxy(
    mut c: *mut aa_context,
    mut x: std::ffi::c_int,
    mut y: std::ffi::c_int,
) {
    SLsmg_gotorc(y, x);
}
unsafe extern "C" fn slang_cursor(mut c: *mut aa_context, mut mode: std::ffi::c_int) {
    SLtt_set_cursor_visibility(mode);
}
