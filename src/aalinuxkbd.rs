use super::aaattributes::*;
use super::aagpm::{__curses_usegpm, __gpm_user_handler, Gpm_Etype, Gpm_Event, Gpm_Margin};
use super::aarec::{aa_mouserecommended, aa_recommendlow};
use super::aastructs::*;
use ::c2rust_bitfields;
use ::libc;

unsafe extern "C" {
    fn select(
        __nfds: std::ffi::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *mut timeval,
    ) -> std::ffi::c_int;
    fn read(__fd: std::ffi::c_int, __buf: *mut std::ffi::c_void, __nbytes: size_t) -> ssize_t;
    fn usleep(__useconds: __useconds_t) -> std::ffi::c_int;
    fn getpid() -> __pid_t;
    static mut stdin: *mut FILE;
    fn printf(_: *const std::ffi::c_char, _: ...) -> std::ffi::c_int;
    fn fileno(__stream: *mut FILE) -> std::ffi::c_int;
    fn fcntl(__fd: std::ffi::c_int, __cmd: std::ffi::c_int, _: ...) -> std::ffi::c_int;
    fn signal(__sig: std::ffi::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn raise(__sig: std::ffi::c_int) -> std::ffi::c_int;
    fn sigaction(
        __sig: std::ffi::c_int,
        __act: *const sigaction,
        __oact: *mut sigaction,
    ) -> std::ffi::c_int;
    fn tcgetattr(__fd: std::ffi::c_int, __termios_p: *mut termios) -> std::ffi::c_int;
    fn tcsetattr(
        __fd: std::ffi::c_int,
        __optional_actions: std::ffi::c_int,
        __termios_p: *const termios,
    ) -> std::ffi::c_int;
    fn ioctl(__fd: std::ffi::c_int, __request: std::ffi::c_ulong, _: ...) -> std::ffi::c_int;
    fn memset(
        _: *mut std::ffi::c_void,
        _: std::ffi::c_int,
        _: std::ffi::c_ulong,
    ) -> *mut std::ffi::c_void;
    fn longjmp(_: *mut __jmp_buf_tag, _: std::ffi::c_int) -> !;
    static mut gpm_fd: std::ffi::c_int;
    static mut gpm_zerobased: std::ffi::c_int;
    static mut gpm_visiblepointer: std::ffi::c_int;
    static mut _gpm_buf: [std::ffi::c_uchar; 0];
    static mut _gpm_arg: *mut std::ffi::c_ushort;
    fn Gpm_GetEvent(_: *mut Gpm_Event) -> std::ffi::c_int;
    static mut gpm_consolefd: std::ffi::c_int;
    fn atexit(__func: Option<unsafe extern "C" fn() -> ()>) -> std::ffi::c_int;
}
pub type __uint32_t = std::ffi::c_uint;
pub type __uid_t = std::ffi::c_uint;
pub type __off_t = std::ffi::c_long;
pub type __off64_t = std::ffi::c_long;
pub type __pid_t = std::ffi::c_int;
pub type __clock_t = std::ffi::c_long;
pub type __time_t = std::ffi::c_long;
pub type __useconds_t = std::ffi::c_uint;
pub type __suseconds_t = std::ffi::c_long;
pub type __ssize_t = std::ffi::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [std::ffi::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
pub type __fd_mask = std::ffi::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fd_set {
    pub __fds_bits: [__fd_mask; 16],
}
pub type ssize_t = __ssize_t;
pub type size_t = std::ffi::c_ulong;
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
pub union sigval {
    pub sival_int: std::ffi::c_int,
    pub sival_ptr: *mut std::ffi::c_void,
}
pub type __sigval_t = sigval;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct siginfo_t {
    pub si_signo: std::ffi::c_int,
    pub si_errno: std::ffi::c_int,
    pub si_code: std::ffi::c_int,
    pub __pad0: std::ffi::c_int,
    pub _sifields: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub _pad: [std::ffi::c_int; 28],
    pub _kill: C2RustUnnamed_8,
    pub _timer: C2RustUnnamed_7,
    pub _rt: C2RustUnnamed_6,
    pub _sigchld: C2RustUnnamed_5,
    pub _sigfault: C2RustUnnamed_2,
    pub _sigpoll: C2RustUnnamed_1,
    pub _sigsys: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub _call_addr: *mut std::ffi::c_void,
    pub _syscall: std::ffi::c_int,
    pub _arch: std::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub si_band: std::ffi::c_long,
    pub si_fd: std::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub si_addr: *mut std::ffi::c_void,
    pub si_addr_lsb: std::ffi::c_short,
    pub _bounds: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub _addr_bnd: C2RustUnnamed_4,
    pub _pkey: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub _lower: *mut std::ffi::c_void,
    pub _upper: *mut std::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_status: std::ffi::c_int,
    pub si_utime: __clock_t,
    pub si_stime: __clock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub si_tid: std::ffi::c_int,
    pub si_overrun: std::ffi::c_int,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
}
pub type __sighandler_t = Option<unsafe extern "C" fn(std::ffi::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigaction {
    pub __sigaction_handler: C2RustUnnamed_9,
    pub sa_mask: __sigset_t,
    pub sa_flags: std::ffi::c_int,
    pub sa_restorer: Option<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_9 {
    pub sa_handler: __sighandler_t,
    pub sa_sigaction:
        Option<unsafe extern "C" fn(std::ffi::c_int, *mut siginfo_t, *mut std::ffi::c_void) -> ()>,
}
pub type cc_t = std::ffi::c_uchar;
pub type speed_t = std::ffi::c_uint;
pub type tcflag_t = std::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct termios {
    pub c_iflag: tcflag_t,
    pub c_oflag: tcflag_t,
    pub c_cflag: tcflag_t,
    pub c_lflag: tcflag_t,
    pub c_line: cc_t,
    pub c_cc: [cc_t; 32],
    pub c_ispeed: speed_t,
    pub c_ospeed: speed_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kbentry {
    pub kb_table: std::ffi::c_uchar,
    pub kb_index: std::ffi::c_uchar,
    pub kb_value: std::ffi::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vt_mode {
    pub mode: std::ffi::c_char,
    pub waitv: std::ffi::c_char,
    pub relsig: std::ffi::c_short,
    pub acqsig: std::ffi::c_short,
    pub frsig: std::ffi::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vt_stat {
    pub v_active: std::ffi::c_ushort,
    pub v_signal: std::ffi::c_ushort,
    pub v_state: std::ffi::c_ushort,
}
pub type __jmp_buf = [std::ffi::c_long; 8];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: std::ffi::c_int,
    pub __saved_mask: __sigset_t,
}
pub type jmp_buf = [__jmp_buf_tag; 1];
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
pub const GPM_RGT: Gpm_Margin = 8;
pub const GPM_LFT: Gpm_Margin = 4;
pub const GPM_BOT: Gpm_Margin = 2;
pub const GPM_TOP: Gpm_Margin = 1;
static mut oldios: termios = termios {
    c_iflag: 0,
    c_oflag: 0,
    c_cflag: 0,
    c_lflag: 0,
    c_line: 0,
    c_cc: [0; 32],
    c_ispeed: 0,
    c_ospeed: 0,
};
static mut keymap: [[std::ffi::c_int; 256]; 2] = [[0; 256]; 2];
static mut tty_fd: std::ffi::c_int = -(1 as std::ffi::c_int);
static mut restart_con: std::ffi::c_int = 0;
static mut alt_pressed: std::ffi::c_int = 0;
static mut new_termio: termios = termios {
    c_iflag: 0,
    c_oflag: 0,
    c_cflag: 0,
    c_lflag: 0,
    c_line: 0,
    c_cc: [0; 32],
    c_ispeed: 0,
    c_ospeed: 0,
};
static mut old_termio: termios = termios {
    c_iflag: 0,
    c_oflag: 0,
    c_cflag: 0,
    c_lflag: 0,
    c_line: 0,
    c_cc: [0; 32],
    c_ispeed: 0,
    c_ospeed: 0,
};
static mut vtswitch_allowed: std::ffi::c_int = 0;
static mut key_down: [std::ffi::c_char; 128] = [0; 128];
static mut closed: std::ffi::c_int = 1 as std::ffi::c_int;
static mut mypid: std::ffi::c_int = 0;
unsafe fn get_keyb_map() -> std::ffi::c_int {
    unsafe {
        static mut keyb_ent: kbentry = kbentry {
            kb_table: 0,
            kb_index: 0,
            kb_value: 0,
        };
        let mut f = 0;
        keyb_ent.kb_table = 0 as std::ffi::c_uchar;
        f = 0;
        while f < 256 {
            keyb_ent.kb_index = f as std::ffi::c_uchar;
            if ioctl(
                tty_fd,
                0x4b46 as std::ffi::c_ulong,
                &mut keyb_ent as *mut kbentry as std::ffi::c_uint,
            ) != 0
            {
                return 0;
            }
            keymap[0][f as usize] = keyb_ent.kb_value as i32;
            f += 1;
            f;
        }
        keyb_ent.kb_table = 1 as std::ffi::c_int as std::ffi::c_uchar;
        f = 0 as std::ffi::c_int;
        while f < 256 as std::ffi::c_int {
            keyb_ent.kb_index = f as std::ffi::c_uchar;
            if ioctl(
                tty_fd,
                0x4b46 as std::ffi::c_int as std::ffi::c_ulong,
                &mut keyb_ent as *mut kbentry as std::ffi::c_uint,
            ) != 0
            {
                return 0 as std::ffi::c_int;
            }
            keymap[1 as std::ffi::c_int as usize][f as usize] =
                keyb_ent.kb_value as std::ffi::c_int;
            f += 1;
            f;
        }
        return 1 as std::ffi::c_int;
    }
}
unsafe extern "C" fn allow_switch(on: std::ffi::c_int) {
    unsafe {
        vtswitch_allowed = on;
    }
}
unsafe extern "C" fn raw_mode(tty_fd_0: std::ffi::c_int, on: std::ffi::c_int) {
    unsafe {
        ioctl(
            tty_fd_0,
            0x4b45 as std::ffi::c_int as std::ffi::c_ulong,
            if on != 0 {
                0x2 as std::ffi::c_int
            } else {
                0x1 as std::ffi::c_int
            },
        );
    }
}
unsafe extern "C" fn blank_key_down() {
    unsafe {
        let mut f: std::ffi::c_int = 0;
        f = 0 as std::ffi::c_int;
        while f < 256 as std::ffi::c_int {
            key_down[f as usize] = 0 as std::ffi::c_int as std::ffi::c_char;
            f += 1;
            f;
        }
    }
}
unsafe extern "C" fn vt_from_here(num: std::ffi::c_int) {
    unsafe {
        ioctl(
            tty_fd,
            0x5403 as std::ffi::c_int as std::ffi::c_ulong,
            &mut old_termio as *mut termios,
        );
        raw_mode(tty_fd, 0 as std::ffi::c_int);
        ioctl(
            tty_fd,
            0x5605 as std::ffi::c_int as std::ffi::c_ulong,
            0x2 as std::ffi::c_int,
        );
        signal(
            10 as std::ffi::c_int,
            Some(vt_from_here as unsafe extern "C" fn(std::ffi::c_int) -> ()),
        );
    }
}
unsafe extern "C" fn vt_to_here(num: std::ffi::c_int) {
    unsafe {
        let mut ios: termios = termios {
            c_iflag: 0,
            c_oflag: 0,
            c_cflag: 0,
            c_lflag: 0,
            c_line: 0,
            c_cc: [0; 32],
            c_ispeed: 0,
            c_ospeed: 0,
        };
        ioctl(
            tty_fd,
            0x5403 as std::ffi::c_int as std::ffi::c_ulong,
            &mut new_termio as *mut termios,
        );
        restart_con = 1 as std::ffi::c_int;
        alt_pressed = 0 as std::ffi::c_int;
        raw_mode(tty_fd, 1 as std::ffi::c_int);
        ios = oldios;
        ios.c_lflag &= !(0o10 as std::ffi::c_int) as tcflag_t;
        tcsetattr(tty_fd, 0 as std::ffi::c_int, &mut ios);
        blank_key_down();
        signal(
            12 as std::ffi::c_int,
            Some(vt_to_here as unsafe extern "C" fn(std::ffi::c_int) -> ()),
        );
    }
}
unsafe extern "C" fn rawmode_init() -> std::ffi::c_int {
    unsafe {
        if closed == 0 {
            return 0 as std::ffi::c_int;
        }
        mypid = getpid();
        if tty_fd == -(1 as std::ffi::c_int) {
            tty_fd = fileno(stdin);
            fcntl(tty_fd, 4 as std::ffi::c_int, 0o4000 as std::ffi::c_int);
        }
        ioctl(
            tty_fd,
            0x5401 as std::ffi::c_int as std::ffi::c_ulong,
            &mut old_termio as *mut termios,
        );
        new_termio = old_termio;
        new_termio.c_lflag &= !(0o1 as std::ffi::c_int | 0o2 as std::ffi::c_int) as tcflag_t;
        ioctl(
            tty_fd,
            0x5403 as std::ffi::c_int as std::ffi::c_ulong,
            &mut new_termio as *mut termios,
        );
        if get_keyb_map() != 0 {
            let mut vtm: vt_mode = vt_mode {
                mode: 0,
                waitv: 0,
                relsig: 0,
                acqsig: 0,
                frsig: 0,
            };
            let mut ios: termios = termios {
                c_iflag: 0,
                c_oflag: 0,
                c_cflag: 0,
                c_lflag: 0,
                c_line: 0,
                c_cc: [0; 32],
                c_ispeed: 0,
                c_ospeed: 0,
            };
            blank_key_down();
            raw_mode(tty_fd, 1 as std::ffi::c_int);
            signal(
                10 as std::ffi::c_int,
                Some(vt_from_here as unsafe extern "C" fn(std::ffi::c_int) -> ()),
            );
            signal(
                12 as std::ffi::c_int,
                Some(vt_to_here as unsafe extern "C" fn(std::ffi::c_int) -> ()),
            );
            ioctl(
                tty_fd,
                0x5601 as std::ffi::c_int as std::ffi::c_ulong,
                &mut vtm as *mut vt_mode,
            );
            vtm.mode = 0x1 as std::ffi::c_int as std::ffi::c_char;
            vtm.relsig = 10 as std::ffi::c_int as std::ffi::c_short;
            vtm.acqsig = 12 as std::ffi::c_int as std::ffi::c_short;
            ioctl(
                tty_fd,
                0x5602 as std::ffi::c_int as std::ffi::c_ulong,
                &mut vtm as *mut vt_mode,
            );
            tcgetattr(tty_fd, &mut oldios);
            ios = oldios;
            ios.c_lflag &= !(0o10 as std::ffi::c_int) as tcflag_t;
            tcsetattr(tty_fd, 0 as std::ffi::c_int, &mut ios);
            closed = 0 as std::ffi::c_int;
            return 1 as std::ffi::c_int;
        } else {
            return 0 as std::ffi::c_int;
        };
    }
}
unsafe extern "C" fn rawmode_exit() {
    unsafe {
        let mut vtm: vt_mode = vt_mode {
            mode: 0,
            waitv: 0,
            relsig: 0,
            acqsig: 0,
            frsig: 0,
        };
        if mypid != getpid() {
            return;
        }
        if closed != 0 {
            return;
        }
        closed = 1 as std::ffi::c_int;
        raw_mode(tty_fd, 0 as std::ffi::c_int);
        ioctl(
            tty_fd,
            0x5601 as std::ffi::c_int as std::ffi::c_ulong,
            &mut vtm as *mut vt_mode,
        );
        vtm.mode = 0 as std::ffi::c_int as std::ffi::c_char;
        ioctl(
            tty_fd,
            0x5602 as std::ffi::c_int as std::ffi::c_ulong,
            &mut vtm as *mut vt_mode,
        );
        ioctl(
            tty_fd,
            0x5403 as std::ffi::c_int as std::ffi::c_ulong,
            &mut old_termio as *mut termios,
        );
        fcntl(tty_fd, 4 as std::ffi::c_int, 0 as std::ffi::c_int);
        tty_fd = -(1 as std::ffi::c_int);
        tcsetattr(tty_fd, 0 as std::ffi::c_int, &mut oldios);
    }
}
unsafe extern "C" fn get_scancode() -> std::ffi::c_int {
    unsafe {
        let mut c: std::ffi::c_uchar = 0;
        if read(
            tty_fd,
            &mut c as *mut std::ffi::c_uchar as *mut std::ffi::c_void,
            1 as std::ffi::c_int as size_t,
        ) <= 0 as std::ffi::c_int as ssize_t
        {
            return -(1 as std::ffi::c_int);
        }
        return c as std::ffi::c_int;
    }
}
unsafe extern "C" fn scan_keyboard() -> std::ffi::c_int {
    unsafe {
        let mut c: std::ffi::c_int = 0;
        let mut key: std::ffi::c_int = 0;
        let mut flag: std::ffi::c_int = 0;
        loop {
            c = get_scancode();
            if !(c == 0xe0 as std::ffi::c_int) {
                break;
            }
        }
        if c == 0xe1 as std::ffi::c_int {
            c = get_scancode();
        }
        if c == -(1 as std::ffi::c_int) {
            return -(1 as std::ffi::c_int);
        }
        key = c & 127 as std::ffi::c_int;
        flag = if c & 128 as std::ffi::c_int != 0 {
            0 as std::ffi::c_int
        } else {
            1 as std::ffi::c_int
        };
        if flag != 0 || key_down[key as usize] as std::ffi::c_int != flag {
            key_down[key as usize] = flag as std::ffi::c_char;
        } else {
            return scan_keyboard();
        }
        if key == 0x38 as std::ffi::c_int {
            alt_pressed = flag;
        }
        if alt_pressed != 0
            && flag != 0
            && key
                >= 0x3a as std::ffi::c_int
                    + 1 as std::ffi::c_int
                    + (if 1 as std::ffi::c_int > 10 as std::ffi::c_int {
                        18 as std::ffi::c_int
                    } else {
                        0 as std::ffi::c_int
                    })
            && key
                <= 0x3a as std::ffi::c_int
                    + 10 as std::ffi::c_int
                    + (if 10 as std::ffi::c_int > 10 as std::ffi::c_int {
                        18 as std::ffi::c_int
                    } else {
                        0 as std::ffi::c_int
                    })
        {
            let mut vts: vt_stat = vt_stat {
                v_active: 0,
                v_signal: 0,
                v_state: 0,
            };
            let mut newvt: std::ffi::c_int = 0;
            ioctl(
                tty_fd,
                0x5603 as std::ffi::c_int as std::ffi::c_ulong,
                &mut vts as *mut vt_stat,
            );
            newvt = c
                - (0x3a as std::ffi::c_int
                    + 1 as std::ffi::c_int
                    + (if 1 as std::ffi::c_int > 10 as std::ffi::c_int {
                        18 as std::ffi::c_int
                    } else {
                        0 as std::ffi::c_int
                    }))
                + 1 as std::ffi::c_int;
            if vts.v_active as std::ffi::c_int != newvt && vtswitch_allowed != 0 {
                ioctl(
                    tty_fd,
                    0x5606 as std::ffi::c_int as std::ffi::c_ulong,
                    newvt,
                );
                restart_con = 0 as std::ffi::c_int;
                while restart_con == 0 as std::ffi::c_int {
                    usleep(50000 as std::ffi::c_int as __useconds_t);
                }
            }
            return -(1 as std::ffi::c_int);
        }
        if flag != 0
            && key == 46 as std::ffi::c_int
            && key_down[0x1d as std::ffi::c_int as usize] as std::ffi::c_int != 0
        {
            raise(2 as std::ffi::c_int);
        }
        return key;
    }
}
unsafe extern "C" fn keymap_trans(sc: std::ffi::c_int) -> std::ffi::c_int {
    unsafe {
        if sc < 0 as std::ffi::c_int || sc > 127 as std::ffi::c_int {
            return -(1 as std::ffi::c_int);
        }
        return keymap[(key_down[0x2a as std::ffi::c_int as usize] as std::ffi::c_int != 0
            || key_down[0x36 as std::ffi::c_int as usize] as std::ffi::c_int != 0)
            as std::ffi::c_int as usize][sc as usize];
    }
}
static mut iswaiting: std::ffi::c_int = 0;
static mut __resized: std::ffi::c_int = 0;
static mut buf: jmp_buf = [__jmp_buf_tag {
    __jmpbuf: [0; 8],
    __mask_was_saved: 0,
    __saved_mask: __sigset_t { __val: [0; 16] },
}; 1];
unsafe extern "C" fn handler(i: std::ffi::c_int) {
    unsafe {
        __resized = 2 as std::ffi::c_int;
        signal(
            28 as std::ffi::c_int,
            Some(handler as unsafe extern "C" fn(std::ffi::c_int) -> ()),
        );
        if iswaiting != 0 {
            longjmp(buf.as_mut_ptr(), 1 as std::ffi::c_int);
        }
    }
}
static mut sig2catch: [std::ffi::c_char; 16] = [
    1 as std::ffi::c_int as std::ffi::c_char,
    2 as std::ffi::c_int as std::ffi::c_char,
    3 as std::ffi::c_int as std::ffi::c_char,
    4 as std::ffi::c_int as std::ffi::c_char,
    5 as std::ffi::c_int as std::ffi::c_char,
    6 as std::ffi::c_int as std::ffi::c_char,
    7 as std::ffi::c_int as std::ffi::c_char,
    8 as std::ffi::c_int as std::ffi::c_char,
    11 as std::ffi::c_int as std::ffi::c_char,
    13 as std::ffi::c_int as std::ffi::c_char,
    14 as std::ffi::c_int as std::ffi::c_char,
    15 as std::ffi::c_int as std::ffi::c_char,
    24 as std::ffi::c_int as std::ffi::c_char,
    25 as std::ffi::c_int as std::ffi::c_char,
    26 as std::ffi::c_int as std::ffi::c_char,
    30 as std::ffi::c_int as std::ffi::c_char,
];
static mut old_signal_handler: [sigaction; 16] = [sigaction {
    __sigaction_handler: C2RustUnnamed_9 { sa_handler: None },
    sa_mask: __sigset_t { __val: [0; 16] },
    sa_flags: 0,
    sa_restorer: None,
}; 16];
unsafe extern "C" fn exithandler(v: std::ffi::c_int) {
    unsafe {
        let mut i: std::ffi::c_int = 0;
        printf(
            b"AAlib: signal %i received\n\0" as *const u8 as *const std::ffi::c_char,
            v,
        );
        rawmode_exit();
        i = 0 as std::ffi::c_int;
        while i < ::core::mem::size_of::<[std::ffi::c_char; 16]>() as std::ffi::c_ulong
            as std::ffi::c_int
        {
            if sig2catch[i as usize] as std::ffi::c_int == v {
                sigaction(
                    v,
                    old_signal_handler.as_mut_ptr().offset(i as isize),
                    0 as *mut sigaction,
                );
                raise(v);
                break;
            } else {
                i += 1;
                i;
            }
        }
        if i >= ::core::mem::size_of::<[std::ffi::c_char; 16]>() as std::ffi::c_ulong
            as std::ffi::c_int
        {
            printf(
                b"AA-lib: Aieeee! Illegal call to signal_handler, raising segfault.\n\0"
                    as *const u8 as *const std::ffi::c_char,
            );
            raise(11 as std::ffi::c_int);
        }
    }
}
unsafe fn linux_init(context: *mut aa_context, mode: i64) -> i64 {
    unsafe {
        let mut i: std::ffi::c_int = 0;
        let mut siga: sigaction = sigaction {
            __sigaction_handler: C2RustUnnamed_9 { sa_handler: None },
            sa_mask: __sigset_t { __val: [0; 16] },
            sa_flags: 0,
            sa_restorer: None,
        };
        if mode & 1 == 0 {
            return 0;
        }
        if rawmode_init() == 0 {
            return 0;
        }
        signal(
            28 as std::ffi::c_int,
            Some(handler as unsafe extern "C" fn(std::ffi::c_int) -> ()),
        );
        aa_recommendlow(
            &mut aa_mouserecommended,
            b"gpm\0" as *const u8 as *const std::ffi::c_char,
        );
        allow_switch(1 as std::ffi::c_int);
        atexit(Some(rawmode_exit as unsafe extern "C" fn() -> ()));
        i = 0 as std::ffi::c_int;
        while i < ::core::mem::size_of::<[std::ffi::c_char; 16]>() as std::ffi::c_ulong
            as std::ffi::c_int
        {
            siga.__sigaction_handler.sa_handler =
                Some(exithandler as unsafe extern "C" fn(std::ffi::c_int) -> ());
            siga.sa_flags = 0 as std::ffi::c_int;
            memset(
                &mut siga.sa_mask as *mut __sigset_t as *mut std::ffi::c_void,
                0 as std::ffi::c_int,
                ::core::mem::size_of::<sigset_t>() as std::ffi::c_ulong,
            );
            sigaction(
                sig2catch[i as usize] as std::ffi::c_int,
                &mut siga,
                old_signal_handler.as_mut_ptr().offset(i as isize),
            );
            i += 1;
            i;
        }
        return 1;
    }
}
unsafe fn linux_uninit(c: *mut aa_context) {
    unsafe {
        signal(
            28 as std::ffi::c_int,
            ::core::mem::transmute::<libc::intptr_t, __sighandler_t>(
                1 as std::ffi::c_int as libc::intptr_t,
            ),
        );
        rawmode_exit();
    }
}
unsafe fn linux_getchar(c1: *mut aa_context, wait: i64) -> i64 {
    unsafe {
        static mut e: Gpm_Event = Gpm_Event {
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
        let mut c: std::ffi::c_int = 0;
        let mut key: std::ffi::c_int = 0;
        let mut tv: timeval = timeval {
            tv_sec: 0,
            tv_usec: 0,
        };
        loop {
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
            readfds.__fds_bits[(tty_fd
                / (8 as std::ffi::c_int
                    * ::core::mem::size_of::<__fd_mask>() as std::ffi::c_ulong as std::ffi::c_int))
                as usize] |= ((1 as std::ffi::c_ulong)
                << tty_fd
                    % (8 as std::ffi::c_int
                        * ::core::mem::size_of::<__fd_mask>() as std::ffi::c_ulong
                            as std::ffi::c_int)) as __fd_mask;
            if gpm_visiblepointer != 0 {
                *_gpm_buf.as_mut_ptr().offset(
                    (::core::mem::size_of::<std::ffi::c_short>() as std::ffi::c_ulong)
                        .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong)
                        as isize,
                ) = 2 as std::ffi::c_int as std::ffi::c_uchar;
                let ref mut fresh0 = *_gpm_arg.offset(2 as std::ffi::c_int as isize);
                *fresh0 = (e.x as std::ffi::c_ushort as std::ffi::c_int + gpm_zerobased)
                    as std::ffi::c_ushort;
                *_gpm_arg.offset(0 as std::ffi::c_int as isize) = *fresh0;
                let ref mut fresh1 = *_gpm_arg.offset(3 as std::ffi::c_int as isize);
                *fresh1 = (e.y as std::ffi::c_ushort as std::ffi::c_int + gpm_zerobased)
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
                            ::core::mem::size_of::<std::ffi::c_short>() as std::ffi::c_ulong
                                as isize,
                        )
                        .offset(-(1 as std::ffi::c_int as isize)),
                );
            }
            if __curses_usegpm != 0 {
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
            select(
                (if __curses_usegpm != 0 {
                    gpm_fd
                } else {
                    0 as std::ffi::c_int
                }) + 1 as std::ffi::c_int,
                &mut readfds,
                0 as *mut fd_set,
                0 as *mut fd_set,
                if wait != 0 {
                    0 as *mut timeval
                } else {
                    &mut tv
                },
            );
            if __curses_usegpm != 0
                && readfds.__fds_bits[(gpm_fd
                    / (8 as std::ffi::c_int
                        * ::core::mem::size_of::<__fd_mask>() as std::ffi::c_ulong
                            as std::ffi::c_int)) as usize]
                    & ((1 as std::ffi::c_ulong)
                        << gpm_fd
                            % (8 as std::ffi::c_int
                                * ::core::mem::size_of::<__fd_mask>() as std::ffi::c_ulong
                                    as std::ffi::c_int)) as __fd_mask
                    != 0 as std::ffi::c_int as __fd_mask
            {
                if Gpm_GetEvent(&mut e) == 1 as std::ffi::c_int {
                    let e_ptr: *mut Gpm_Event = &mut e;
                    __gpm_user_handler(e_ptr, 0 as *mut std::ffi::c_void);
                    return 259;
                }
            }
            c = scan_keyboard();
            if c != -(1 as std::ffi::c_int) {
                match c {
                    1 => {
                        key = 305 as std::ffi::c_int;
                    }
                    28 => {
                        key = 13 as std::ffi::c_int;
                    }
                    14 => {
                        key = 304 as std::ffi::c_int;
                    }
                    75 => {
                        key = 302 as std::ffi::c_int;
                    }
                    77 => {
                        key = 303 as std::ffi::c_int;
                    }
                    72 => {
                        key = 300 as std::ffi::c_int;
                    }
                    80 => {
                        key = 301 as std::ffi::c_int;
                    }
                    105 => {
                        key = 302 as std::ffi::c_int;
                    }
                    106 => {
                        key = 303 as std::ffi::c_int;
                    }
                    103 => {
                        key = 300 as std::ffi::c_int;
                    }
                    108 => {
                        key = 301 as std::ffi::c_int;
                    }
                    _ => {
                        key = keymap_trans(c) & 255 as std::ffi::c_int;
                    }
                }
                if key_down[c as usize] == 0 {
                    key |= 65536 as std::ffi::c_int;
                }
                return key.try_into().unwrap();
            } else {
                key = AA_NONE as std::ffi::c_int;
            }
            if !(wait != 0) {
                break;
            }
        }
        return AA_NONE.try_into().unwrap();
    }
}

#[unsafe(no_mangle)]
pub static mut kbd_linux_d: aa_kbddriver = unsafe {
    {
        let init = aa_kbddriver {
            shortname: b"linux\0" as *const u8 as *const std::ffi::c_char,
            name: b"Linux console raw keyboard driver 1.0\0" as *const u8
                as *const std::ffi::c_char,
            flags: 1,
            init: Some(linux_init),
            uninit: Some(linux_uninit),
            getkey: Some(linux_getchar),
        };
        init
    }
};
