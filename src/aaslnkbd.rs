use super::aarec::aa_recommendlow;
use super::aastructs::*;
use ::c2rust_bitfields;

unsafe extern "C" {
    fn _setjmp(_: *mut __jmp_buf_tag) -> std::ffi::c_int;
    fn longjmp(_: *mut __jmp_buf_tag, _: std::ffi::c_int) -> !;
    fn signal(__sig: std::ffi::c_int, __handler: __sighandler_t) -> __sighandler_t;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    fn fflush(__stream: *mut FILE) -> std::ffi::c_int;
    fn fopen(_: *const std::ffi::c_char, _: *const std::ffi::c_char) -> *mut FILE;
    fn printf(_: *const std::ffi::c_char, _: ...) -> std::ffi::c_int;
    fn select(
        __nfds: std::ffi::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *mut timeval,
    ) -> std::ffi::c_int;
    fn SLang_init_tty(
        _: std::ffi::c_int,
        _: std::ffi::c_int,
        _: std::ffi::c_int,
    ) -> std::ffi::c_int;
    fn SLang_reset_tty();
    fn SLang_input_pending(_: std::ffi::c_int) -> std::ffi::c_int;
    fn SLtt_get_terminfo();
    fn SLkp_init() -> std::ffi::c_int;
    fn SLkp_getkey() -> std::ffi::c_int;
    static mut gpm_fd: std::ffi::c_int;
    static mut gpm_hflag: std::ffi::c_int;
    static mut gpm_zerobased: std::ffi::c_int;
    static mut _gpm_buf: [std::ffi::c_uchar; 0];
    static mut _gpm_arg: *mut std::ffi::c_ushort;
    static mut gpm_handler: Option<Gpm_Handler>;
    static mut gpm_data: *mut std::ffi::c_void;
    fn Gpm_GetEvent(_: *mut Gpm_Event) -> std::ffi::c_int;
    fn Gpm_Getc(_: *mut FILE) -> std::ffi::c_int;
    static mut gpm_consolefd: std::ffi::c_int;
    fn ioctl(__fd: std::ffi::c_int, __request: std::ffi::c_ulong, _: ...) -> std::ffi::c_int;
    static mut aa_mouserecommended: *mut aa_linkedlist;
    static mut __slang_is_up: std::ffi::c_int;
    static mut __resized_slang: std::ffi::c_int;
}
pub type __jmp_buf = [std::ffi::c_long; 8];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [std::ffi::c_ulong; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: std::ffi::c_int,
    pub __saved_mask: __sigset_t,
}
pub type jmp_buf = [__jmp_buf_tag; 1];
pub type __off_t = std::ffi::c_long;
pub type __off64_t = std::ffi::c_long;
pub type __time_t = std::ffi::c_long;
pub type __suseconds_t = std::ffi::c_long;
pub type __sighandler_t = Option<unsafe extern "C" fn(std::ffi::c_int) -> ()>;
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
pub type Gpm_Etype = std::ffi::c_uint;
pub const GPM_LEAVE: Gpm_Etype = 1024;
pub const GPM_ENTER: Gpm_Etype = 512;
pub const GPM_HARD: Gpm_Etype = 256;
pub const GPM_MFLAG: Gpm_Etype = 128;
pub const GPM_TRIPLE: Gpm_Etype = 64;
pub const GPM_DOUBLE: Gpm_Etype = 32;
pub const GPM_SINGLE: Gpm_Etype = 16;
pub const GPM_UP: Gpm_Etype = 8;
pub const GPM_DOWN: Gpm_Etype = 4;
pub const GPM_DRAG: Gpm_Etype = 2;
pub const GPM_MOVE: Gpm_Etype = 1;
pub type Gpm_Margin = std::ffi::c_uint;
pub const GPM_RGT: Gpm_Margin = 8;
pub const GPM_LFT: Gpm_Margin = 4;
pub const GPM_BOT: Gpm_Margin = 2;
pub const GPM_TOP: Gpm_Margin = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Gpm_Event {
    pub buttons: std::ffi::c_uchar,
    pub modifiers: std::ffi::c_uchar,
    pub vc: std::ffi::c_ushort,
    pub dx: std::ffi::c_short,
    pub dy: std::ffi::c_short,
    pub x: std::ffi::c_short,
    pub y: std::ffi::c_short,
    pub type_0: Gpm_Etype,
    pub clicks: std::ffi::c_int,
    pub margin: Gpm_Margin,
    pub wdx: std::ffi::c_short,
    pub wdy: std::ffi::c_short,
}
pub type Gpm_Handler =
    unsafe extern "C" fn(*mut Gpm_Event, *mut std::ffi::c_void) -> std::ffi::c_int;
pub type aa_dithering_mode = std::ffi::c_uint;
pub const AA_DITHERTYPES: aa_dithering_mode = 3;
pub const AA_FLOYD_S: aa_dithering_mode = 2;
pub const AA_ERRORDISTRIB: aa_dithering_mode = 1;
pub const AA_NONE: aa_dithering_mode = 0;
static mut iswaiting: std::ffi::c_int = 0;
static mut f: *mut FILE = 0 as *const FILE as *mut FILE;
static mut buf: jmp_buf = [__jmp_buf_tag {
    __jmpbuf: [0; 8],
    __mask_was_saved: 0,
    __saved_mask: __sigset_t { __val: [0; 16] },
}; 1];

pub static mut __slang_keyboard: std::ffi::c_int = 0;
static mut uninitslang: std::ffi::c_int = 0;
unsafe extern "C" fn handler(mut i: std::ffi::c_int) {
    ::core::ptr::write_volatile(
        &mut __resized_slang as *mut std::ffi::c_int,
        2 as std::ffi::c_int,
    );
    signal(
        28 as std::ffi::c_int,
        Some(handler as unsafe extern "C" fn(std::ffi::c_int) -> ()),
    );
    if iswaiting != 0 {
        longjmp(buf.as_mut_ptr(), 1 as std::ffi::c_int);
    }
}
unsafe extern "C" fn slang_init(
    mut context: *mut aa_context,
    mut mode: std::ffi::c_int,
) -> std::ffi::c_int {
    if __slang_is_up == 0 {
        fflush(stdout);
        SLtt_get_terminfo();
        __slang_is_up = 1 as std::ffi::c_int;
        uninitslang = 1 as std::ffi::c_int;
    }
    f = fopen(
        b"/dev/null\0" as *const u8 as *const std::ffi::c_char,
        b"r\0" as *const u8 as *const std::ffi::c_char,
    );
    if SLang_init_tty(
        -(1 as std::ffi::c_int),
        0 as std::ffi::c_int,
        0 as std::ffi::c_int,
    ) == -(1 as std::ffi::c_int)
    {
        return 0 as std::ffi::c_int;
    }
    if -(1 as std::ffi::c_int) == SLkp_init() {
        return 0 as std::ffi::c_int;
    }
    __slang_keyboard = 1 as std::ffi::c_int;
    aa_recommendlow(
        &mut aa_mouserecommended,
        b"gpm\0" as *const u8 as *const std::ffi::c_char,
    );
    signal(
        28 as std::ffi::c_int,
        Some(handler as unsafe extern "C" fn(std::ffi::c_int) -> ()),
    );
    return 1 as std::ffi::c_int;
}
unsafe extern "C" fn slang_uninit(mut c: *mut aa_context) {
    if uninitslang != 0 {
        uninitslang = 0 as std::ffi::c_int;
        __slang_is_up = 0 as std::ffi::c_int;
    }
    SLang_reset_tty();
}
unsafe extern "C" fn slang_getchar(
    mut c1: *mut aa_context,
    mut wait: std::ffi::c_int,
) -> std::ffi::c_int {
    let mut c: std::ffi::c_int = 0;
    let mut flag: std::ffi::c_int = 0 as std::ffi::c_int;
    static mut ev: Gpm_Event = Gpm_Event {
        buttons: 0,
        modifiers: 0,
        vc: 0,
        dx: 0,
        dy: 0,
        x: 0,
        y: 0,
        type_0: 0 as Gpm_Etype,
        clicks: 0,
        margin: 0 as Gpm_Margin,
        wdx: 0,
        wdy: 0,
    };
    let mut tv: timeval = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut readfds: fd_set = fd_set {
        __fds_bits: [0; 16],
    };
    if wait != 0 {
        _setjmp(buf.as_mut_ptr());
        iswaiting = 1 as std::ffi::c_int;
    } else {
        iswaiting = 0 as std::ffi::c_int;
    }
    if __resized_slang == 2 as std::ffi::c_int {
        iswaiting = 0 as std::ffi::c_int;
        ::core::ptr::write_volatile(
            &mut __resized_slang as *mut std::ffi::c_int,
            1 as std::ffi::c_int,
        );
        return 258 as std::ffi::c_int;
    }
    if wait == 0 {
        if gpm_fd == -(1 as std::ffi::c_int) {
            if SLang_input_pending(0 as std::ffi::c_int) == 0 {
                return AA_NONE as std::ffi::c_int;
            }
        } else {
            *_gpm_buf.as_mut_ptr().offset(
                (::core::mem::size_of::<std::ffi::c_short>() as std::ffi::c_ulong)
                    .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong)
                    as isize,
            ) = 2 as std::ffi::c_int as std::ffi::c_uchar;
            let ref mut fresh0 = *_gpm_arg.offset(2 as std::ffi::c_int as isize);
            *fresh0 = (ev.x as std::ffi::c_ushort as std::ffi::c_int + gpm_zerobased)
                as std::ffi::c_ushort;
            *_gpm_arg.offset(0 as std::ffi::c_int as isize) = *fresh0;
            let ref mut fresh1 = *_gpm_arg.offset(3 as std::ffi::c_int as isize);
            *fresh1 = (ev.y as std::ffi::c_ushort as std::ffi::c_int + gpm_zerobased)
                as std::ffi::c_ushort;
            *_gpm_arg.offset(1 as std::ffi::c_int as isize) = *fresh1;
            *_gpm_arg.offset(4 as std::ffi::c_int as isize) =
                3 as std::ffi::c_int as std::ffi::c_ushort;
            ioctl(
                gpm_consolefd,
                0x541c as std::ffi::c_int as std::ffi::c_ulong,
                _gpm_buf
                    .as_mut_ptr()
                    .offset(
                        ::core::mem::size_of::<std::ffi::c_short>() as std::ffi::c_ulong as isize,
                    )
                    .offset(-(1 as std::ffi::c_int as isize)),
            );
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
            if gpm_fd != -(2 as std::ffi::c_int) {
                readfds.__fds_bits[(gpm_fd
                    / (8 as std::ffi::c_int
                        * ::core::mem::size_of::<__fd_mask>() as std::ffi::c_ulong
                            as std::ffi::c_int)) as usize] |= ((1 as std::ffi::c_ulong)
                    << gpm_fd
                        % (8 as std::ffi::c_int
                            * ::core::mem::size_of::<__fd_mask>() as std::ffi::c_ulong
                                as std::ffi::c_int))
                    as __fd_mask;
            }
            let fd: usize = 0;
            readfds.__fds_bits[fd / (8 * std::mem::size_of::<__fd_mask>())] |=
                (1 as __fd_mask) << (fd % (8 * std::mem::size_of::<__fd_mask>()));
            ::core::ptr::write_volatile(
                &mut flag as *mut std::ffi::c_int,
                select(
                    (if gpm_fd == -(2 as std::ffi::c_int) {
                        0 as std::ffi::c_int
                    } else {
                        gpm_fd
                    }) + 1 as std::ffi::c_int,
                    &mut readfds,
                    0 as *mut fd_set,
                    0 as *mut fd_set,
                    &mut tv,
                ),
            );
            if ::core::ptr::read_volatile::<std::ffi::c_int>(&flag as *const std::ffi::c_int) == 0 {
                return AA_NONE as std::ffi::c_int;
            }
        }
    }
    if gpm_fd != -(1 as std::ffi::c_int) {
        *_gpm_buf.as_mut_ptr().offset(
            (::core::mem::size_of::<std::ffi::c_short>() as std::ffi::c_ulong)
                .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong) as isize,
        ) = 2 as std::ffi::c_int as std::ffi::c_uchar;
        let ref mut fresh2 = *_gpm_arg.offset(2 as std::ffi::c_int as isize);
        *fresh2 =
            (ev.x as std::ffi::c_ushort as std::ffi::c_int + gpm_zerobased) as std::ffi::c_ushort;
        *_gpm_arg.offset(0 as std::ffi::c_int as isize) = *fresh2;
        let ref mut fresh3 = *_gpm_arg.offset(3 as std::ffi::c_int as isize);
        *fresh3 =
            (ev.y as std::ffi::c_ushort as std::ffi::c_int + gpm_zerobased) as std::ffi::c_ushort;
        *_gpm_arg.offset(1 as std::ffi::c_int as isize) = *fresh3;
        *_gpm_arg.offset(4 as std::ffi::c_int as isize) =
            3 as std::ffi::c_int as std::ffi::c_ushort;
        ioctl(
            gpm_consolefd,
            0x541c as std::ffi::c_int as std::ffi::c_ulong,
            _gpm_buf
                .as_mut_ptr()
                .offset(::core::mem::size_of::<std::ffi::c_short>() as std::ffi::c_ulong as isize)
                .offset(-(1 as std::ffi::c_int as isize)),
        );
        while flag == 0 {
            let mut __i_0: std::ffi::c_uint = 0;
            let mut __arr_0: *mut fd_set = &mut readfds;
            __i_0 = 0 as std::ffi::c_int as std::ffi::c_uint;
            while (__i_0 as std::ffi::c_ulong)
                < (::core::mem::size_of::<fd_set>() as std::ffi::c_ulong)
                    .wrapping_div(::core::mem::size_of::<__fd_mask>() as std::ffi::c_ulong)
            {
                (*__arr_0).__fds_bits[__i_0 as usize] = 0 as std::ffi::c_int as __fd_mask;
                __i_0 = __i_0.wrapping_add(1);
                __i_0;
            }
            if gpm_fd != -(2 as std::ffi::c_int) {
                readfds.__fds_bits[(gpm_fd
                    / (8 as std::ffi::c_int
                        * ::core::mem::size_of::<__fd_mask>() as std::ffi::c_ulong
                            as std::ffi::c_int)) as usize] |= ((1 as std::ffi::c_ulong)
                    << gpm_fd
                        % (8 as std::ffi::c_int
                            * ::core::mem::size_of::<__fd_mask>() as std::ffi::c_ulong
                                as std::ffi::c_int))
                    as __fd_mask;
            }
            let fd: usize = 0;
            readfds.__fds_bits[fd / (8 * std::mem::size_of::<__fd_mask>())] |=
                (1 as __fd_mask) << (fd % (8 * std::mem::size_of::<__fd_mask>()));
            tv.tv_sec = 60 as std::ffi::c_int as __time_t;
            ::core::ptr::write_volatile(
                &mut flag as *mut std::ffi::c_int,
                select(
                    (if gpm_fd == -(2 as std::ffi::c_int) {
                        0 as std::ffi::c_int
                    } else {
                        gpm_fd
                    }) + 1 as std::ffi::c_int,
                    &mut readfds,
                    0 as *mut fd_set,
                    0 as *mut fd_set,
                    &mut tv,
                ),
            );
        }
        if flag == -(1 as std::ffi::c_int) {
            printf(b"error!\n\0" as *const u8 as *const std::ffi::c_char);
            return AA_NONE as std::ffi::c_int;
        }
        if gpm_fd > -(1 as std::ffi::c_int)
            && readfds.__fds_bits[(gpm_fd
                / (8 as std::ffi::c_int
                    * ::core::mem::size_of::<__fd_mask>() as std::ffi::c_ulong as std::ffi::c_int))
                as usize]
                & ((1 as std::ffi::c_ulong)
                    << gpm_fd
                        % (8 as std::ffi::c_int
                            * ::core::mem::size_of::<__fd_mask>() as std::ffi::c_ulong
                                as std::ffi::c_int)) as __fd_mask
                != 0 as std::ffi::c_int as __fd_mask
        {
            if Gpm_GetEvent(&mut ev) != 0
                && gpm_handler.is_some()
                && (Some(gpm_handler.expect("non-null function pointer")))
                    .expect("non-null function pointer")(&mut ev, gpm_data)
                    != 0
            {
                gpm_hflag = 1 as std::ffi::c_int;
                return 259 as std::ffi::c_int;
            }
        }
    }
    if gpm_fd == -(2 as std::ffi::c_int) {
        c = Gpm_Getc(stdin);
    } else {
        c = SLkp_getkey();
    }
    iswaiting = 0 as std::ffi::c_int;
    if __resized_slang == 2 as std::ffi::c_int {
        ::core::ptr::write_volatile(
            &mut __resized_slang as *mut std::ffi::c_int,
            1 as std::ffi::c_int,
        );
        return 258 as std::ffi::c_int;
    }
    if c == 27 as std::ffi::c_int {
        return 305 as std::ffi::c_int;
    }
    if c > 0 as std::ffi::c_int && c < 128 as std::ffi::c_int && c != 127 as std::ffi::c_int {
        return c;
    }
    match c {
        65535 => return AA_NONE as std::ffi::c_int,
        259 => return 302 as std::ffi::c_int,
        260 => return 303 as std::ffi::c_int,
        257 => return 300 as std::ffi::c_int,
        258 => return 301 as std::ffi::c_int,
        272 | 127 => return 304 as std::ffi::c_int,
        _ => {}
    }
    return 400 as std::ffi::c_int;
}

pub static mut kbd_slang_d: aa_kbddriver = unsafe {
    {
        let mut init = aa_kbddriver {
            shortname: b"slang\0" as *const u8 as *const std::ffi::c_char,
            name: b"Slang keyboard driver 1.0\0" as *const u8 as *const std::ffi::c_char,
            flags: 0 as std::ffi::c_int,
            init: Some(
                slang_init
                    as unsafe extern "C" fn(*mut aa_context, std::ffi::c_int) -> std::ffi::c_int,
            ),
            uninit: Some(slang_uninit as unsafe extern "C" fn(*mut aa_context) -> ()),
            getkey: Some(
                slang_getchar
                    as unsafe extern "C" fn(*mut aa_context, std::ffi::c_int) -> std::ffi::c_int,
            ),
        };
        init
    }
};
