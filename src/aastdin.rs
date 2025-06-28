use super::aarec::{aa_linkedlist, aa_recommendlow};
use ::c2rust_bitfields;
use ::libc;
unsafe extern "C" {
    static mut stdin: *mut FILE;
    fn getc(__stream: *mut FILE) -> std::ffi::c_int;
    fn feof(__stream: *mut FILE) -> std::ffi::c_int;
    fn signal(__sig: std::ffi::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn _setjmp(_: *mut __jmp_buf_tag) -> std::ffi::c_int;
    fn longjmp(_: *mut __jmp_buf_tag, _: std::ffi::c_int) -> !;
    fn select(
        __nfds: std::ffi::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *mut timeval,
    ) -> std::ffi::c_int;
    static mut gpm_fd: std::ffi::c_int;
    fn Gpm_Getc(_: *mut FILE) -> std::ffi::c_int;
    static mut aa_mouserecommended: *mut aa_linkedlist;
    static mut __curses_usegpm: std::ffi::c_int;
}
pub type __off_t = std::ffi::c_long;
pub type __off64_t = std::ffi::c_long;
pub type __time_t = std::ffi::c_long;
pub type __suseconds_t = std::ffi::c_long;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [std::ffi::c_ulong; 16],
}
pub type __sighandler_t = Option<unsafe extern "C" fn(std::ffi::c_int) -> ()>;
pub type __jmp_buf = [std::ffi::c_long; 8];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: std::ffi::c_int,
    pub __saved_mask: __sigset_t,
}
pub type jmp_buf = [__jmp_buf_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type __fd_mask = std::ffi::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fd_set {
    pub __fds_bits: [__fd_mask; 16],
}
pub type aa_dithering_mode = std::ffi::c_uint;
pub const AA_DITHERTYPES: aa_dithering_mode = 3;
pub const AA_FLOYD_S: aa_dithering_mode = 2;
pub const AA_ERRORDISTRIB: aa_dithering_mode = 1;
pub const AA_NONE: aa_dithering_mode = 0;
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
static mut iswaiting: std::ffi::c_int = 0;
static mut __resized: std::ffi::c_int = 0;
static mut buf: jmp_buf = [__jmp_buf_tag {
    __jmpbuf: [0; 8],
    __mask_was_saved: 0,
    __saved_mask: __sigset_t { __val: [0; 16] },
}; 1];
unsafe extern "C" fn handler(mut i: std::ffi::c_int) {
    __resized = 2 as std::ffi::c_int;
    signal(
        28 as std::ffi::c_int,
        Some(handler as unsafe extern "C" fn(std::ffi::c_int) -> ()),
    );
    if iswaiting != 0 {
        longjmp(buf.as_mut_ptr(), 1 as std::ffi::c_int);
    }
}
unsafe extern "C" fn stdin_init(
    mut context: *mut aa_context,
    mut mode: std::ffi::c_int,
) -> std::ffi::c_int {
    signal(
        28 as std::ffi::c_int,
        Some(handler as unsafe extern "C" fn(std::ffi::c_int) -> ()),
    );
    aa_recommendlow(
        &mut aa_mouserecommended,
        b"gpm\0" as *const u8 as *const std::ffi::c_char,
    );
    return 1 as std::ffi::c_int;
}
unsafe extern "C" fn stdin_uninit(mut c: *mut aa_context) {
    signal(
        28 as std::ffi::c_int,
        ::core::mem::transmute::<libc::intptr_t, __sighandler_t>(
            1 as std::ffi::c_int as libc::intptr_t,
        ),
    );
}
unsafe extern "C" fn stdin_getchar(
    mut c1: *mut aa_context,
    mut wait: std::ffi::c_int,
) -> std::ffi::c_int {
    let mut c: std::ffi::c_int = 0;
    let mut flag: std::ffi::c_int = 0;
    let mut tv: timeval = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    if wait != 0 {
        _setjmp(buf.as_mut_ptr());
        iswaiting = 1 as std::ffi::c_int;
    }
    if __resized == 2 as std::ffi::c_int {
        __resized = 1 as std::ffi::c_int;
        return 258 as std::ffi::c_int;
    }
    if wait == 0 {
        let mut readfds: fd_set = fd_set {
            __fds_bits: [0; 16],
        };
        tv.tv_sec = 0 as std::ffi::c_int as __time_t;
        tv.tv_usec = 0 as std::ffi::c_int as __suseconds_t;
        let mut __i: std::ffi::c_uint = 0;
        let mut __arr: *mut fd_set = &mut readfds;
        __i = 0 as std::ffi::c_int as std::ffi::c_uint;
        while (__i as std::ffi::c_ulong)
            < (::core::mem::size_of::<fd_set>() as std::ffi::c_ulong)
                .wrapping_div(::core::mem::size_of::<__fd_mask>() as std::ffi::c_ulong)
        {
            (*__arr).__fds_bits[__i as usize] = 0 as std::ffi::c_int as __fd_mask;
            __i = __i.wrapping_add(1);
            __i;
        }
        let fd: usize = 0;
        readfds.__fds_bits[fd / (8 * std::mem::size_of::<__fd_mask>())] |=
            (1 as __fd_mask) << (fd % (8 * std::mem::size_of::<__fd_mask>()));

        if __curses_usegpm != 0 {
            readfds.__fds_bits[(gpm_fd
                / (8 as std::ffi::c_int
                    * ::core::mem::size_of::<__fd_mask>() as std::ffi::c_ulong as std::ffi::c_int))
                as usize] |= ((1 as std::ffi::c_ulong)
                << gpm_fd
                    % (8 as std::ffi::c_int
                        * ::core::mem::size_of::<__fd_mask>() as std::ffi::c_ulong
                            as std::ffi::c_int)) as __fd_mask;
        }
        flag = select(
            (if __curses_usegpm != 0 {
                gpm_fd
            } else {
                0 as std::ffi::c_int
            }) + 1 as std::ffi::c_int,
            &mut readfds,
            0 as *mut fd_set,
            0 as *mut fd_set,
            &mut tv,
        );
        if flag == 0 {
            return AA_NONE as std::ffi::c_int;
        }
    }
    if __curses_usegpm != 0 {
        c = Gpm_Getc(stdin);
    } else {
        c = getc(stdin);
    }
    iswaiting = 0 as std::ffi::c_int;
    if c == 27 as std::ffi::c_int {
        return 305 as std::ffi::c_int;
    }
    if c == 10 as std::ffi::c_int {
        return 13 as std::ffi::c_int;
    }
    if c > 0 as std::ffi::c_int && c < 127 as std::ffi::c_int && c != 127 as std::ffi::c_int {
        return c;
    }
    match c {
        127 => return 304 as std::ffi::c_int,
        _ => {}
    }
    if feof(stdin) != 0 {
        return AA_NONE as std::ffi::c_int;
    }
    return 400 as std::ffi::c_int;
}

pub static mut kbd_stdin_d: aa_kbddriver = unsafe {
    {
        let mut init = aa_kbddriver {
            shortname: b"stdin\0" as *const u8 as *const std::ffi::c_char,
            name: b"Standard input keyboard driver 1.0\0" as *const u8 as *const std::ffi::c_char,
            flags: 0 as std::ffi::c_int,
            init: Some(
                stdin_init
                    as unsafe extern "C" fn(*mut aa_context, std::ffi::c_int) -> std::ffi::c_int,
            ),
            uninit: Some(stdin_uninit as unsafe extern "C" fn(*mut aa_context) -> ()),
            getkey: Some(
                stdin_getchar
                    as unsafe extern "C" fn(*mut aa_context, std::ffi::c_int) -> std::ffi::c_int,
            ),
        };
        init
    }
};
