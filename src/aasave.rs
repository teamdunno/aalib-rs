use super::aastructs::*;
use ::c2rust_bitfields;
unsafe extern "C" {
    fn fclose(__stream: *mut FILE) -> std::ffi::c_int;
    fn fflush(__stream: *mut FILE) -> std::ffi::c_int;
    fn fopen(_: *const std::ffi::c_char, _: *const std::ffi::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const std::ffi::c_char, _: ...) -> std::ffi::c_int;
    fn sprintf(_: *mut std::ffi::c_char, _: *const std::ffi::c_char, _: ...) -> std::ffi::c_int;
    fn fputs(__s: *const std::ffi::c_char, __stream: *mut FILE) -> std::ffi::c_int;
    fn memcpy(
        _: *mut std::ffi::c_void,
        _: *const std::ffi::c_void,
        _: std::ffi::c_ulong,
    ) -> *mut std::ffi::c_void;
    fn memset(
        _: *mut std::ffi::c_void,
        _: std::ffi::c_int,
        _: std::ffi::c_ulong,
    ) -> *mut std::ffi::c_void;
    fn malloc(_: std::ffi::c_ulong) -> *mut std::ffi::c_void;
    static aa_font16: aa_font;
    static aa_fontline: aa_font;
    static aa_fontcourier: aa_font;
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
pub type aa_attribute = std::ffi::c_uint;
pub const AA_SPECIAL: aa_attribute = 5;
pub const AA_REVERSE: aa_attribute = 4;
pub const AA_BOLDFONT: aa_attribute = 3;
pub const AA_BOLD: aa_attribute = 2;
pub const AA_DIM: aa_attribute = 1;
pub const AA_NORMAL: aa_attribute = 0;
static mut html_escapes: [*const std::ffi::c_char; 7] = [
    b"<\0" as *const u8 as *const std::ffi::c_char,
    b"&lt;\0" as *const u8 as *const std::ffi::c_char,
    b">\0" as *const u8 as *const std::ffi::c_char,
    b"&gt;\0" as *const u8 as *const std::ffi::c_char,
    b"&\0" as *const u8 as *const std::ffi::c_char,
    b"&amp;\0" as *const u8 as *const std::ffi::c_char,
    0 as *const std::ffi::c_char,
];
static mut html_alt_escapes: [*const std::ffi::c_char; 9] = [
    b"<\0" as *const u8 as *const std::ffi::c_char,
    b"&lt;\0" as *const u8 as *const std::ffi::c_char,
    b">\0" as *const u8 as *const std::ffi::c_char,
    b"&gt;\0" as *const u8 as *const std::ffi::c_char,
    b"&\0" as *const u8 as *const std::ffi::c_char,
    b"&amp;\0" as *const u8 as *const std::ffi::c_char,
    b"\"\0" as *const u8 as *const std::ffi::c_char,
    b"&quot;\0" as *const u8 as *const std::ffi::c_char,
    0 as *const std::ffi::c_char,
];
static mut irc_escapes: [*const std::ffi::c_char; 3] = [
    b"@\0" as *const u8 as *const std::ffi::c_char,
    b"@@\0" as *const u8 as *const std::ffi::c_char,
    0 as *const std::ffi::c_char,
];

pub static mut aa_nhtml_format: aa_format = unsafe {
    {
        let mut init = aa_format {
            width: 79,
            height: 36,
            pagewidth: 79,
            pageheight: 36,
            flags: 0,
            supported: 1 | 4 | 8
                | 2,
            font: &aa_fontcourier as *const aa_font,
            formatname: b"Nestcapeized html\0" as *const u8 as *const std::ffi::c_char,
            extension: b".html\0" as *const u8 as *const std::ffi::c_char,
            head: b"<HTML>\n<HEAD><TITLE>Ascii arted image done using aalib</TITLE>\n</HEAD>\n<BODY BGCOLOR=\"#000000\" TEXT=\"#b2b2b2\" LINK=\"#FFFFFF\">\n<FONT COLOR=#b2b2b2 SIZE=2><PRE>\n\0"
                as *const u8 as *const std::ffi::c_char,
            end: b"</PRE></FONT></BODY>\n</HTML>\n\0" as *const u8
                as *const std::ffi::c_char,
            newline: b"\n\0" as *const u8 as *const std::ffi::c_char,
            prints: [
                b"%s\0" as *const u8 as *const std::ffi::c_char,
                b"%s\0" as *const u8 as *const std::ffi::c_char,
                b"%s\0" as *const u8 as *const std::ffi::c_char,
                b"%s\0" as *const u8 as *const std::ffi::c_char,
                b"%s\0" as *const u8 as *const std::ffi::c_char,
            ],
            begin: [
                b"\0" as *const u8 as *const std::ffi::c_char,
                b"<FONT COLOR=\"686868\">\0" as *const u8 as *const std::ffi::c_char,
                b"<FONT COLOR=\"ffffff\">\0" as *const u8 as *const std::ffi::c_char,
                b"\0" as *const u8 as *const std::ffi::c_char,
                b"<B>\0" as *const u8 as *const std::ffi::c_char,
            ],
            ends: [
                b"\0" as *const u8 as *const std::ffi::c_char,
                b"</FONT>\0" as *const u8 as *const std::ffi::c_char,
                b"</FONT>\0" as *const u8 as *const std::ffi::c_char,
                b"\0" as *const u8 as *const std::ffi::c_char,
                b"</B>\0" as *const u8 as *const std::ffi::c_char,
            ],
            conversions: html_escapes.as_ptr(),
        };
        init
    }
};

pub static mut aa_html_format: aa_format = unsafe {
    {
        let mut init = aa_format {
            width: 79,
            height: 25,
            pagewidth: 79,
            pageheight: 25,
            flags: 0,
            supported: 1 | 4
                | 8,
            font: 0 as *const aa_font,
            formatname: b"Pure html\0" as *const u8 as *const std::ffi::c_char,
            extension: b".html\0" as *const u8 as *const std::ffi::c_char,
            head: b"<HTML>\n <HEAD> <TITLE>Ascii arted image done using aalib</TITLE>\n</HEAD>\n<BODY><PRE>\n\0"
                as *const u8 as *const std::ffi::c_char,
            end: b"</PRE></BODY>\n</HTML>\n\0" as *const u8 as *const std::ffi::c_char,
            newline: b"\n\0" as *const u8 as *const std::ffi::c_char,
            prints: [
                b"%s\0" as *const u8 as *const std::ffi::c_char,
                b"%s\0" as *const u8 as *const std::ffi::c_char,
                b"%s\0" as *const u8 as *const std::ffi::c_char,
                b"%s\0" as *const u8 as *const std::ffi::c_char,
                b"%s\0" as *const u8 as *const std::ffi::c_char,
            ],
            begin: [
                b"\0" as *const u8 as *const std::ffi::c_char,
                b"\0" as *const u8 as *const std::ffi::c_char,
                b"<B>\0" as *const u8 as *const std::ffi::c_char,
                b"\0" as *const u8 as *const std::ffi::c_char,
                b"<B>\0" as *const u8 as *const std::ffi::c_char,
            ],
            ends: [
                b"\0" as *const u8 as *const std::ffi::c_char,
                b"\0" as *const u8 as *const std::ffi::c_char,
                b"</B>\0" as *const u8 as *const std::ffi::c_char,
                b"\0" as *const u8 as *const std::ffi::c_char,
                b"</B>\0" as *const u8 as *const std::ffi::c_char,
            ],
            conversions: html_escapes.as_ptr(),
        };
        init
    }
};

pub static mut aa_ansi_format: aa_format = {
    let mut init = aa_format {
        width: 80,
        height: 25,
        pagewidth: 80,
        pageheight: 25,
        flags: 0,
        supported: 1 | 4 | 8 | 16 | 2,
        font: 0 as *const aa_font,
        formatname: b"ANSI escape seqences\0" as *const u8 as *const std::ffi::c_char,
        extension: b".ansi\0" as *const u8 as *const std::ffi::c_char,
        head: b"\0" as *const u8 as *const std::ffi::c_char,
        end: b"\0" as *const u8 as *const std::ffi::c_char,
        newline: b"\n\0" as *const u8 as *const std::ffi::c_char,
        prints: [
            b"%s\0" as *const u8 as *const std::ffi::c_char,
            b"%s\0" as *const u8 as *const std::ffi::c_char,
            b"%s\0" as *const u8 as *const std::ffi::c_char,
            b"%s\0" as *const u8 as *const std::ffi::c_char,
            b"%s\0" as *const u8 as *const std::ffi::c_char,
        ],
        begin: [
            b"\0" as *const u8 as *const std::ffi::c_char,
            b"\x1B[8m\0" as *const u8 as *const std::ffi::c_char,
            b"\x1B[1m\0" as *const u8 as *const std::ffi::c_char,
            b"\x1B[1m\0" as *const u8 as *const std::ffi::c_char,
            b"\x1B[7m\0" as *const u8 as *const std::ffi::c_char,
        ],
        ends: [
            b"\0" as *const u8 as *const std::ffi::c_char,
            b"\x1B[0;10m\0" as *const u8 as *const std::ffi::c_char,
            b"\x1B[0;10m\0" as *const u8 as *const std::ffi::c_char,
            b"\x1B[0;10m\0" as *const u8 as *const std::ffi::c_char,
            b"\x1B[0;10m\0" as *const u8 as *const std::ffi::c_char,
        ],
        conversions: 0 as *const *const std::ffi::c_char,
    };
    init
};

pub static mut aa_text_format: aa_format = {
    let mut init = aa_format {
        width: 80,
        height: 25,
        pagewidth: 80,
        pageheight: 25,
        flags: 0,
        supported: 1,
        font: 0 as *const aa_font,
        formatname: b"Text file\0" as *const u8 as *const std::ffi::c_char,
        extension: b".txt\0" as *const u8 as *const std::ffi::c_char,
        head: b"\0" as *const u8 as *const std::ffi::c_char,
        end: b"\0" as *const u8 as *const std::ffi::c_char,
        newline: b"\n\0" as *const u8 as *const std::ffi::c_char,
        prints: [
            b"%s\0" as *const u8 as *const std::ffi::c_char,
            b"%s\0" as *const u8 as *const std::ffi::c_char,
            b"%s\0" as *const u8 as *const std::ffi::c_char,
            b"%s\0" as *const u8 as *const std::ffi::c_char,
            b"%s\0" as *const u8 as *const std::ffi::c_char,
        ],
        begin: [
            b"\0" as *const u8 as *const std::ffi::c_char,
            b"\0" as *const u8 as *const std::ffi::c_char,
            b"\0" as *const u8 as *const std::ffi::c_char,
            b"\0" as *const u8 as *const std::ffi::c_char,
            b"\0" as *const u8 as *const std::ffi::c_char,
        ],
        ends: [
            b"\0" as *const u8 as *const std::ffi::c_char,
            b"\0" as *const u8 as *const std::ffi::c_char,
            b"\0" as *const u8 as *const std::ffi::c_char,
            b"\0" as *const u8 as *const std::ffi::c_char,
            b"\0" as *const u8 as *const std::ffi::c_char,
        ],
        conversions: 0 as *const *const std::ffi::c_char,
    };
    init
};

pub static mut aa_more_format: aa_format = {
    let mut init = aa_format {
        width: 80,
        height: 25,
        pagewidth: 80,
        pageheight: 25,
        flags: 8,
        supported: 1 | 4 | 8,
        font: 0 as *const aa_font,
        formatname: b"For more/less\0" as *const u8 as *const std::ffi::c_char,
        extension: b".cat\0" as *const u8 as *const std::ffi::c_char,
        head: b"\0" as *const u8 as *const std::ffi::c_char,
        end: b"\0" as *const u8 as *const std::ffi::c_char,
        newline: b"\n\0" as *const u8 as *const std::ffi::c_char,
        prints: [
            b"%s\0" as *const u8 as *const std::ffi::c_char,
            b"%s\0" as *const u8 as *const std::ffi::c_char,
            b"%s\x08%s\0" as *const u8 as *const std::ffi::c_char,
            b"%s\x08%s\0" as *const u8 as *const std::ffi::c_char,
            b"%s\0" as *const u8 as *const std::ffi::c_char,
        ],
        begin: [
            b"\0" as *const u8 as *const std::ffi::c_char,
            b"\0" as *const u8 as *const std::ffi::c_char,
            b"\0" as *const u8 as *const std::ffi::c_char,
            b"\0" as *const u8 as *const std::ffi::c_char,
            b"\0" as *const u8 as *const std::ffi::c_char,
        ],
        ends: [
            b"\0" as *const u8 as *const std::ffi::c_char,
            b"\0" as *const u8 as *const std::ffi::c_char,
            b"\0" as *const u8 as *const std::ffi::c_char,
            b"\0" as *const u8 as *const std::ffi::c_char,
            b"\0" as *const u8 as *const std::ffi::c_char,
        ],
        conversions: 0 as *const *const std::ffi::c_char,
    };
    init
};

pub static mut aa_hp_format: aa_format = unsafe {
    {
        let mut init = aa_format {
            width: 130 * 2,
            height: 64 * 2 - 1,
            pagewidth: 130,
            pageheight: 64 * 2 - 1,
            flags: 1,
            supported: 1,
            font: &aa_fontline as *const aa_font,
            formatname: b"HP laser jet - A4 small font\0" as *const u8 as *const std::ffi::c_char,
            extension: b".hp\0" as *const u8 as *const std::ffi::c_char,
            head: b"\x1B(10U\x1B(s0p16.67h8.5v0s0b0T\x1B%%0A\x1B&/0U\x1B&/0Z\x1B&/24D\0"
                as *const u8 as *const std::ffi::c_char,
            end: b"\x0C\0" as *const u8 as *const std::ffi::c_char,
            newline: b"\r\x1B=\0" as *const u8 as *const std::ffi::c_char,
            prints: [
                b"%s\0" as *const u8 as *const std::ffi::c_char,
                b"%s\0" as *const u8 as *const std::ffi::c_char,
                b"%s\0" as *const u8 as *const std::ffi::c_char,
                b"%s\0" as *const u8 as *const std::ffi::c_char,
                b"%s\0" as *const u8 as *const std::ffi::c_char,
            ],
            begin: [
                b"\0" as *const u8 as *const std::ffi::c_char,
                b"\0" as *const u8 as *const std::ffi::c_char,
                b"\0" as *const u8 as *const std::ffi::c_char,
                b"\0" as *const u8 as *const std::ffi::c_char,
                b"\0" as *const u8 as *const std::ffi::c_char,
            ],
            ends: [
                b"\0" as *const u8 as *const std::ffi::c_char,
                b"\0" as *const u8 as *const std::ffi::c_char,
                b"\0" as *const u8 as *const std::ffi::c_char,
                b"\0" as *const u8 as *const std::ffi::c_char,
                b"\0" as *const u8 as *const std::ffi::c_char,
            ],
            conversions: 0 as *const *const std::ffi::c_char,
        };
        init
    }
};

pub static mut aa_hp2_format: aa_format = unsafe {
    {
        let mut init = aa_format {
            width: 80 * 2,
            height: 64 - 1,
            pagewidth: 80,
            pageheight: 64 - 1,
            flags: 1,
            supported: 1,
            font: &aa_font16 as *const aa_font,
            formatname: b"HP laser jet - A4 big font\0" as *const u8 as *const std::ffi::c_char,
            extension: b".hp\0" as *const u8 as *const std::ffi::c_char,
            head: b"\x1B(s7B\x1B*p0Y@\0" as *const u8 as *const std::ffi::c_char,
            end: b"\x0C\0" as *const u8 as *const std::ffi::c_char,
            newline: b"\r\n\0" as *const u8 as *const std::ffi::c_char,
            prints: [
                b"%s\0" as *const u8 as *const std::ffi::c_char,
                b"%s\0" as *const u8 as *const std::ffi::c_char,
                b"%s\0" as *const u8 as *const std::ffi::c_char,
                b"%s\0" as *const u8 as *const std::ffi::c_char,
                b"%s\0" as *const u8 as *const std::ffi::c_char,
            ],
            begin: [
                b"\0" as *const u8 as *const std::ffi::c_char,
                b"\0" as *const u8 as *const std::ffi::c_char,
                b"\0" as *const u8 as *const std::ffi::c_char,
                b"\0" as *const u8 as *const std::ffi::c_char,
                b"\0" as *const u8 as *const std::ffi::c_char,
            ],
            ends: [
                b"\0" as *const u8 as *const std::ffi::c_char,
                b"\0" as *const u8 as *const std::ffi::c_char,
                b"\0" as *const u8 as *const std::ffi::c_char,
                b"\0" as *const u8 as *const std::ffi::c_char,
                b"\0" as *const u8 as *const std::ffi::c_char,
            ],
            conversions: 0 as *const *const std::ffi::c_char,
        };
        init
    }
};

pub static mut aa_irc_format: aa_format = unsafe {
    {
        let mut init = aa_format {
            width: 70,
            height: 25,
            pagewidth: 70,
            pageheight: 25,
            flags: 0,
            supported: 1 | 4 | 16,
            font: &aa_font16 as *const aa_font,
            formatname: b"For catting to an IRC channel\0" as *const u8 as *const std::ffi::c_char,
            extension: b".irc\0" as *const u8 as *const std::ffi::c_char,
            head: b"\0" as *const u8 as *const std::ffi::c_char,
            end: b"\0" as *const u8 as *const std::ffi::c_char,
            newline: b"\n\0" as *const u8 as *const std::ffi::c_char,
            prints: [
                b"%s\0" as *const u8 as *const std::ffi::c_char,
                b"%s\0" as *const u8 as *const std::ffi::c_char,
                b"%s\0" as *const u8 as *const std::ffi::c_char,
                b"%s\0" as *const u8 as *const std::ffi::c_char,
                b"%s\0" as *const u8 as *const std::ffi::c_char,
            ],
            begin: [
                b"\0" as *const u8 as *const std::ffi::c_char,
                b"\0" as *const u8 as *const std::ffi::c_char,
                b"\x02\0" as *const u8 as *const std::ffi::c_char,
                b"\0" as *const u8 as *const std::ffi::c_char,
                b"\x16\0" as *const u8 as *const std::ffi::c_char,
            ],
            ends: [
                b"\0" as *const u8 as *const std::ffi::c_char,
                b"\0" as *const u8 as *const std::ffi::c_char,
                b"\x02\0" as *const u8 as *const std::ffi::c_char,
                b"\0" as *const u8 as *const std::ffi::c_char,
                b"\x16\0" as *const u8 as *const std::ffi::c_char,
            ],
            conversions: 0 as *const *const std::ffi::c_char,
        };
        init
    }
};

pub static mut aa_zephyr_format: aa_format = unsafe {
    {
        let mut init = aa_format {
            width: 70,
            height: 25,
            pagewidth: 70,
            pageheight: 25,
            flags: 0,
            supported: 1 | 2 | 4,
            font: &aa_font16 as *const aa_font,
            formatname: b"For catting to an IRC channel II\0" as *const u8
                as *const std::ffi::c_char,
            extension: b".irc\0" as *const u8 as *const std::ffi::c_char,
            head: b"\0" as *const u8 as *const std::ffi::c_char,
            end: b"\0" as *const u8 as *const std::ffi::c_char,
            newline: b"\n\0" as *const u8 as *const std::ffi::c_char,
            prints: [
                b"%s\0" as *const u8 as *const std::ffi::c_char,
                b"%s\0" as *const u8 as *const std::ffi::c_char,
                b"%s\0" as *const u8 as *const std::ffi::c_char,
                b"%s\0" as *const u8 as *const std::ffi::c_char,
                b"%s\0" as *const u8 as *const std::ffi::c_char,
            ],
            begin: [
                b"\0" as *const u8 as *const std::ffi::c_char,
                b"@color(gray50)\0" as *const u8 as *const std::ffi::c_char,
                b"@b(\0" as *const u8 as *const std::ffi::c_char,
                b"\0" as *const u8 as *const std::ffi::c_char,
                b"\0" as *const u8 as *const std::ffi::c_char,
            ],
            ends: [
                b"\0" as *const u8 as *const std::ffi::c_char,
                b"@color(black)\0" as *const u8 as *const std::ffi::c_char,
                b")\0" as *const u8 as *const std::ffi::c_char,
                b"\0" as *const u8 as *const std::ffi::c_char,
                b"\0" as *const u8 as *const std::ffi::c_char,
            ],
            conversions: irc_escapes.as_ptr(),
        };
        init
    }
};

pub static mut aa_roff_format: aa_format = unsafe {
    {
        let mut init = aa_format {
            width: 70,
            height: 25,
            pagewidth: 70,
            pageheight: 25,
            flags: 0,
            supported: 1 | 4,
            font: &aa_font16 as *const aa_font,
            formatname: b"For including in a man page\0" as *const u8 as *const std::ffi::c_char,
            extension: b".man\0" as *const u8 as *const std::ffi::c_char,
            head: b"\0" as *const u8 as *const std::ffi::c_char,
            end: b"\0" as *const u8 as *const std::ffi::c_char,
            newline: b"\n.br\n\0" as *const u8 as *const std::ffi::c_char,
            prints: [
                b"%s\0" as *const u8 as *const std::ffi::c_char,
                b"%s\0" as *const u8 as *const std::ffi::c_char,
                b"%s\0" as *const u8 as *const std::ffi::c_char,
                b"%s\0" as *const u8 as *const std::ffi::c_char,
                b"%s\0" as *const u8 as *const std::ffi::c_char,
            ],
            begin: [
                b"\0" as *const u8 as *const std::ffi::c_char,
                b"\0" as *const u8 as *const std::ffi::c_char,
                b"\n.B \0" as *const u8 as *const std::ffi::c_char,
                b"\0" as *const u8 as *const std::ffi::c_char,
                b"\0" as *const u8 as *const std::ffi::c_char,
            ],
            ends: [
                b"\0" as *const u8 as *const std::ffi::c_char,
                b"\0" as *const u8 as *const std::ffi::c_char,
                b"\n\0" as *const u8 as *const std::ffi::c_char,
                b"\0" as *const u8 as *const std::ffi::c_char,
                b"\0" as *const u8 as *const std::ffi::c_char,
            ],
            conversions: irc_escapes.as_ptr(),
        };
        init
    }
};

pub static mut aa_html_alt_format: aa_format = unsafe {
    {
        let mut init = aa_format {
            width: 79,
            height: 25,
            pagewidth: 79,
            pageheight: 25,
            flags: 0,
            supported: 1,
            font: 0 as *const aa_font,
            formatname: b"HTML <IMG ALT= tag\0" as *const u8 as *const std::ffi::c_char,
            extension: b".html\0" as *const u8 as *const std::ffi::c_char,
            head: b"<PRE><IMG SRC=\"your image here\" ALT=\"\n\0" as *const u8
                as *const std::ffi::c_char,
            end: b"\"></PRE>\n\0" as *const u8 as *const std::ffi::c_char,
            newline: b"\n\0" as *const u8 as *const std::ffi::c_char,
            prints: [
                b"%s\0" as *const u8 as *const std::ffi::c_char,
                b"%s\0" as *const u8 as *const std::ffi::c_char,
                b"%s\0" as *const u8 as *const std::ffi::c_char,
                b"%s\0" as *const u8 as *const std::ffi::c_char,
                b"%s\0" as *const u8 as *const std::ffi::c_char,
            ],
            begin: [
                b"\0" as *const u8 as *const std::ffi::c_char,
                b"\0" as *const u8 as *const std::ffi::c_char,
                b"\0" as *const u8 as *const std::ffi::c_char,
                b"\0" as *const u8 as *const std::ffi::c_char,
                b"\0" as *const u8 as *const std::ffi::c_char,
            ],
            ends: [
                b"\0" as *const u8 as *const std::ffi::c_char,
                b"\0" as *const u8 as *const std::ffi::c_char,
                b"\0" as *const u8 as *const std::ffi::c_char,
                b"\0" as *const u8 as *const std::ffi::c_char,
                b"\0" as *const u8 as *const std::ffi::c_char,
            ],
            conversions: html_alt_escapes.as_ptr(),
        };
        init
    }
};
pub static mut aa_formats: [*const aa_format; 12] = unsafe {
    [
        &aa_text_format as *const aa_format,
        &aa_html_format as *const aa_format,
        &aa_nhtml_format as *const aa_format,
        &aa_html_alt_format as *const aa_format,
        &aa_more_format as *const aa_format,
        &aa_ansi_format as *const aa_format,
        &aa_hp_format as *const aa_format,
        &aa_hp2_format as *const aa_format,
        &aa_irc_format as *const aa_format,
        &aa_zephyr_format as *const aa_format,
        &aa_roff_format as *const aa_format,
        0 as *const aa_format,
    ]
};

pub static mut save_d: aa_driver = unsafe {
    {
        let mut init = aa_driver {
            shortname: b"save\0" as *const u8 as *const std::ffi::c_char,
            name: b"Special driver for saving to files\0" as *const u8 as *const std::ffi::c_char,
            init: Some(save_init),
            uninit: Some(save_uninit),
            getsize: Some(save_getsize),
            setattr: None,
            print: None,
            gotoxy: Some(save_gotoxy),
            flush: Some(save_flush),
            cursormode: None,
        };
        init
    }
};
unsafe extern "C" fn build_conversions(
    mut in_0: *const *const std::ffi::c_char,
    mut conv: *mut *const std::ffi::c_char,
) -> *mut *const std::ffi::c_char {
    let mut c_0: std::ffi::c_uchar = 0;
    memset(
        conv as *mut std::ffi::c_void,
        0 as std::ffi::c_int,
        (::core::mem::size_of::<*mut std::ffi::c_char>() as std::ffi::c_ulong)
            .wrapping_mul(256 as std::ffi::c_int as std::ffi::c_ulong),
    );
    if !in_0.is_null() {
        while !(*in_0).is_null() && !(*in_0.offset(1 as std::ffi::c_int as isize)).is_null() {
            c_0 = **in_0 as std::ffi::c_uchar;
            in_0 = in_0.offset(1);
            in_0;
            let ref mut fresh0 = *conv.offset(c_0 as isize);
            *fresh0 = *in_0;
            in_0 = in_0.offset(1);
            in_0;
        }
    }
    return conv;
}
unsafe fn save_init(
    mut p: *const aa_hardware_params,
    mut none: *const std::ffi::c_void,
    mut dest: *mut aa_hardware_params,
    mut data: *mut *mut std::ffi::c_void,
) -> i64 {
    let mut d: *const aa_savedata = none as *const aa_savedata;
    static mut def: aa_hardware_params = aa_hardware_params {
        font: 0 as *const aa_font,
        supported: 0,
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
    *data = malloc(::core::mem::size_of::<aa_savedata>() as std::ffi::c_ulong);
    memcpy(
        *data,
        d as *const std::ffi::c_void,
        ::core::mem::size_of::<aa_savedata>() as std::ffi::c_ulong,
    );
    *dest = def;
    if ((*p).font).is_null() {
        (*dest).font = (*(*d).format).font;
    }
    (*dest).width = (*(*d).format).width;
    (*dest).height = (*(*d).format).height;
    (*dest).supported = (*(*d).format).supported;
    return 1;
}
unsafe fn save_uninit(mut c_0: *mut aa_context) {}
unsafe fn save_getsize(mut c_0: *mut aa_context, mut width: &mut i64, mut height: &mut i64) {}
unsafe fn save_gotoxy(mut c_0: *mut aa_context, mut x: i64, mut y: i64) {}
static mut lastattr: std::ffi::c_int = 0;
static mut f: *mut FILE = 0 as *const FILE as *mut FILE;
static mut c: *mut aa_context = 0 as *const aa_context as *mut aa_context;
unsafe fn stop_tag() {
    if lastattr != -(1 as std::ffi::c_int) {
        fputs(
            (*(*((*c).driverdata as *mut aa_savedata)).format).ends[lastattr as usize],
            f,
        );
    }
    lastattr = -(1 as std::ffi::c_int);
}
unsafe fn start_tag(mut attr: std::ffi::c_int) {
    if attr > 5 as std::ffi::c_int {
        attr = 5 as std::ffi::c_int;
    }
    lastattr = attr;
    fputs(
        (*(*((*c).driverdata as *mut aa_savedata)).format).begin[lastattr as usize],
        f,
    );
}
unsafe fn encodechar(
    mut attr: std::ffi::c_uchar,
    mut ch: std::ffi::c_uchar,
    mut conversions: *mut *const std::ffi::c_char,
) {
    let mut chr: [std::ffi::c_char; 2] = [0; 2];
    if (*(*((*c).driverdata as *mut aa_savedata)).format).flags & 8 != 0
        && ch as std::ffi::c_int == ' ' as i32
        && attr as std::ffi::c_int != AA_REVERSE as std::ffi::c_int
    {
        attr = AA_NORMAL as std::ffi::c_int as std::ffi::c_uchar;
    }
    if attr as std::ffi::c_int != lastattr {
        stop_tag();
        start_tag(attr as std::ffi::c_int);
    }
    if (*conversions.offset(ch as isize)).is_null() {
        chr[0 as std::ffi::c_int as usize] = ch as std::ffi::c_char;
        chr[1 as std::ffi::c_int as usize] = 0 as std::ffi::c_int as std::ffi::c_char;
        fprintf(
            f,
            (*(*((*c).driverdata as *mut aa_savedata)).format).prints[attr as usize],
            chr.as_mut_ptr(),
            chr.as_mut_ptr(),
            chr.as_mut_ptr(),
            chr.as_mut_ptr(),
        );
    } else {
        fprintf(
            f,
            (*(*((*c).driverdata as *mut aa_savedata)).format).prints[attr as usize],
            *conversions.offset(ch as isize),
            *conversions.offset(ch as isize),
            *conversions.offset(ch as isize),
            *conversions.offset(ch as isize),
        );
    };
}
unsafe fn savearea(
    mut x1: i64,
    mut y1: i64,
    mut x2: i64,
    mut y2: i64,
    mut conversions: *mut *const std::ffi::c_char,
) {
    let mut x = 0;
    let mut y = 0;
    fputs((*(*((*c).driverdata as *mut aa_savedata)).format).head, f);
    lastattr = -1;
    y = y1;
    while y < y2 {
        x = x1;
        while x < x2 {
            if x < 0 || x >= (*c).params.width || y < 0 || y >= (*c).params.height {
                encodechar(
                    AA_NORMAL as std::ffi::c_uchar,
                    ' ' as i32 as std::ffi::c_uchar,
                    conversions,
                );
            } else {
                let mut pos = x + y * (*c).params.width;
                encodechar(
                    *((*c).attrbuffer).offset(pos as isize),
                    *((*c).textbuffer).offset(pos as isize),
                    conversions,
                );
            }
            x += 1;
            x;
        }
        stop_tag();
        fputs(
            (*(*((*c).driverdata as *mut aa_savedata)).format).newline,
            f,
        );
        y += 1;
        y;
    }
    fputs((*(*((*c).driverdata as *mut aa_savedata)).format).end, f);
    fflush(f);
}
unsafe fn save_flush(mut c1: *mut aa_context) {
    let mut fname: [std::ffi::c_char; 4096] = [0; 4096];
    let mut conversions: [*const std::ffi::c_char; 256] = [0 as *const std::ffi::c_char; 256];
    c = c1;
    build_conversions(
        (*(*((*c).driverdata as *mut aa_savedata)).format).conversions,
        conversions.as_mut_ptr(),
    );
    if (*(*((*c).driverdata as *mut aa_savedata)).format).flags & 1 != 0 {
        let mut xpages =
            ((*c1).params.width + (*(*((*c).driverdata as *mut aa_savedata)).format).pagewidth - 1)
                / (*(*((*c).driverdata as *mut aa_savedata)).format).pagewidth;
        let mut ypages = ((*c1).params.height
            + (*(*((*c).driverdata as *mut aa_savedata)).format).pageheight
            - 1)
            / (*(*((*c).driverdata as *mut aa_savedata)).format).pageheight;
        let mut x = 0;
        let mut y = 0;
        x = 0;
        while x < xpages {
            y = 0 as std::ffi::c_int;
            while (y as i64) < (ypages as i64) {
                if !((*((*c).driverdata as *mut aa_savedata)).name).is_null() {
                    generate_filename(
                        (*((*c).driverdata as *mut aa_savedata)).name,
                        fname.as_mut_ptr(),
                        x.into(),
                        y.into(),
                        1,
                        (*(*((*c).driverdata as *mut aa_savedata)).format).extension,
                    );
                    f = fopen(
                        fname.as_mut_ptr(),
                        b"w\0" as *const u8 as *const std::ffi::c_char,
                    );
                } else {
                    f = (*((*c).driverdata as *mut aa_savedata)).file;
                }
                if f.is_null() {
                    return;
                }
                savearea(
                    x as i64 * (*(*((*c).driverdata as *mut aa_savedata)).format).pagewidth as i64,
                    y as i64 * (*(*((*c).driverdata as *mut aa_savedata)).format).pageheight as i64,
                    (x + 1) as i64
                        * (*(*((*c).driverdata as *mut aa_savedata)).format).pagewidth as i64,
                    (y + 1) as i64
                        * (*(*((*c).driverdata as *mut aa_savedata)).format).pageheight as i64,
                    conversions.as_mut_ptr(),
                );
                if !((*((*c).driverdata as *mut aa_savedata)).name).is_null() {
                    fclose(f);
                }
                y += 1;
                y;
            }
            x += 1;
            x;
        }
    } else {
        if !((*((*c).driverdata as *mut aa_savedata)).name).is_null() {
            generate_filename(
                (*((*c).driverdata as *mut aa_savedata)).name,
                fname.as_mut_ptr(),
                0,
                0,
                0,
                (*(*((*c).driverdata as *mut aa_savedata)).format).extension,
            );
            f = fopen(
                fname.as_mut_ptr(),
                b"w\0" as *const u8 as *const std::ffi::c_char,
            );
        } else {
            f = (*((*c).driverdata as *mut aa_savedata)).file;
        }
        if f.is_null() {
            return;
        }
        savearea(
            0,
            0,
            (*c1).params.width,
            (*c1).params.height,
            conversions.as_mut_ptr(),
        );
        if !((*((*c).driverdata as *mut aa_savedata)).name).is_null() {
            fclose(f);
        }
    };
}
unsafe fn generate_filename(
    mut template: *const std::ffi::c_char,
    mut result: *mut std::ffi::c_char,
    mut x: i64,
    mut y: i64,
    mut pages: i64,
    mut extension: *const std::ffi::c_char,
) -> *mut std::ffi::c_char {
    let mut a: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
    let mut b: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
    let mut end: *mut std::ffi::c_char = result.offset(4090 as std::ffi::c_int as isize);
    b = template.offset(-1);
    a = result.offset(-1);
    loop {
        b = b.offset(1);
        a = a.offset(1);
        *a = *b;
        if !(*a != 0) {
            break;
        }
        if a >= end {
            break;
        }
        if !(*b as i32 == '%' as i32) {
            continue;
        }
        match *b.offset(1) {
            120 => {
                a = a.offset(-1);
                a;
                if pages != 0 {
                    let mut text: [std::ffi::c_char; 8] = [0; 8];
                    let mut e: *mut std::ffi::c_char = text.as_mut_ptr().offset(-1);
                    sprintf(
                        text.as_mut_ptr(),
                        b"%i\0" as *const u8 as *const std::ffi::c_char,
                        x,
                    );
                    loop {
                        e = e.offset(1);
                        a = a.offset(1);
                        *a = *e;
                        if !(*a != 0) {
                            break;
                        }
                        if a >= end {
                            break;
                        }
                    }
                    a = a.offset(-1);
                    a;
                }
                b = b.offset(1);
                b;
            }
            121 => {
                a = a.offset(-1);
                a;
                if pages != 0 {
                    let mut text_0: [std::ffi::c_char; 8] = [0; 8];
                    let mut e_0: *mut std::ffi::c_char = text_0.as_mut_ptr().offset(-1);
                    sprintf(
                        text_0.as_mut_ptr(),
                        b"%i\0" as *const u8 as *const std::ffi::c_char,
                        y,
                    );
                    loop {
                        e_0 = e_0.offset(1);
                        a = a.offset(1);
                        *a = *e_0;
                        if !(*a != 0) {
                            break;
                        }
                        if a >= end {
                            break;
                        }
                    }
                    a = a.offset(-1);
                    a;
                }
                b = b.offset(1);
                b;
            }
            99 => {
                a = a.offset(-1);
                a;
                if pages != 0 {
                    let mut text_1: [std::ffi::c_char; 8] = [0; 8];
                    let mut e_1: *mut std::ffi::c_char = text_1.as_mut_ptr().offset(-1);
                    sprintf(
                        text_1.as_mut_ptr(),
                        b"_%i_%i\0" as *const u8 as *const std::ffi::c_char,
                        x,
                        y,
                    );
                    loop {
                        e_1 = e_1.offset(1);
                        a = a.offset(1);
                        *a = *e_1;
                        if !(*a != 0) {
                            break;
                        }
                        if a >= end {
                            break;
                        }
                    }
                    a = a.offset(-1);
                    a;
                }
                b = b.offset(1);
                b;
            }
            101 => {
                a = a.offset(-1);
                a;
                let mut e_2: *const std::ffi::c_char = extension.offset(-1);
                loop {
                    e_2 = e_2.offset(1);
                    a = a.offset(1);
                    *a = *e_2;
                    if !(*a != 0) {
                        break;
                    }
                    if a >= end {
                        break;
                    }
                }
                a = a.offset(-1);
                a;
                b = b.offset(1);
                b;
            }
            37 => {
                a = a.offset(-1);
                a;
                b = b.offset(1);
                b;
            }
            _ => {}
        }
        if *b == 0 {
            break;
        }
    }
    *a = 0 as std::ffi::c_char;
    return result;
}
