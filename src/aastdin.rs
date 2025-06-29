use super::aagpm::__curses_usegpm;
use super::aarec::aa_mouserecommended;
use super::aarec::aa_recommendlow;
use super::aastructs::*;
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
static mut iswaiting: std::ffi::c_int = 0;
static mut __resized: std::ffi::c_int = 0;
static mut buf: jmp_buf = [__jmp_buf_tag {
    __jmpbuf: [0; 8],
    __mask_was_saved: 0,
    __saved_mask: __sigset_t { __val: [0; 16] },
}; 1];
unsafe extern "C" fn handler(i: std::ffi::c_int) { unsafe {
    __resized = 2 as std::ffi::c_int;
    signal(
        28 as std::ffi::c_int,
        Some(handler as unsafe extern "C" fn(std::ffi::c_int) -> ()),
    );
    if iswaiting != 0 {
        longjmp(buf.as_mut_ptr(), 1 as std::ffi::c_int);
    }
}}
unsafe fn stdin_init(context: *mut aa_context, mode: i64) -> i64 { unsafe {
    signal(
        28 as std::ffi::c_int,
        Some(handler as unsafe extern "C" fn(std::ffi::c_int) -> ()),
    );
    aa_recommendlow(
        &mut aa_mouserecommended,
        b"gpm\0" as *const u8 as *const std::ffi::c_char,
    );
    return 1;
}}
unsafe fn stdin_uninit(c: *mut aa_context) { unsafe {
    signal(
        28 as std::ffi::c_int,
        ::core::mem::transmute::<libc::intptr_t, __sighandler_t>(
            1 as std::ffi::c_int as libc::intptr_t,
        ),
    );
}}
unsafe fn stdin_getchar(c1: *mut aa_context, wait: i64) -> i64 { unsafe {
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
        return 258;
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
            return AA_NONE.try_into().unwrap();
        }
    }
    if __curses_usegpm != 0 {
        c = Gpm_Getc(stdin);
    } else {
        c = getc(stdin);
    }
    iswaiting = 0 as std::ffi::c_int;
    if c == 27 as std::ffi::c_int {
        return 305;
    }
    if c == 10 as std::ffi::c_int {
        return 13;
    }
    if c > 0 as std::ffi::c_int && c < 127 as std::ffi::c_int && c != 127 as std::ffi::c_int {
        return c.try_into().unwrap();
    }
    match c {
        127 => return 304,
        _ => {}
    }
    if feof(stdin) != 0 {
        return AA_NONE.try_into().unwrap();
    }
    return 400;
}}

#[unsafe(no_mangle)]
pub static mut kbd_stdin_d: aa_kbddriver = unsafe {
    {
        let init = aa_kbddriver {
            shortname: b"stdin\0" as *const u8 as *const std::ffi::c_char,
            name: b"Standard input keyboard driver 1.0\0" as *const u8 as *const std::ffi::c_char,
            flags: 0,
            init: Some(stdin_init),
            uninit: Some(stdin_uninit),
            getkey: Some(stdin_getchar),
        };
        init
    }
};
