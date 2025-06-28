use super::aafonts::aa_registerfont;
use super::aain::aa_font;
use super::aarec::{aa_kbdrecommended, aa_linkedlist, aa_mouserecommended, aa_recommendlow};
use super::font14::aa_font14;
use ::c2rust_bitfields;

unsafe extern "C" {
    fn fstat(__fd: std::ffi::c_int, __buf: *mut stat) -> std::ffi::c_int;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> std::ffi::c_int;
    fn fflush(__stream: *mut FILE) -> std::ffi::c_int;
    fn fopen(_: *const std::ffi::c_char, _: *const std::ffi::c_char) -> *mut FILE;
    fn printf(_: *const std::ffi::c_char, _: ...) -> std::ffi::c_int;
    fn sprintf(_: *mut std::ffi::c_char, _: *const std::ffi::c_char, _: ...) -> std::ffi::c_int;
    fn putc(__c: std::ffi::c_int, __stream: *mut FILE) -> std::ffi::c_int;
    fn fread(
        _: *mut std::ffi::c_void,
        _: std::ffi::c_ulong,
        _: std::ffi::c_ulong,
        _: *mut FILE,
    ) -> std::ffi::c_ulong;
    fn fwrite(
        _: *const std::ffi::c_void,
        _: std::ffi::c_ulong,
        _: std::ffi::c_ulong,
        _: *mut FILE,
    ) -> std::ffi::c_ulong;
    fn fseek(
        __stream: *mut FILE,
        __off: std::ffi::c_long,
        __whence: std::ffi::c_int,
    ) -> std::ffi::c_int;
    fn fileno(__stream: *mut FILE) -> std::ffi::c_int;
    fn malloc(_: std::ffi::c_ulong) -> *mut std::ffi::c_void;
    fn ioctl(__fd: std::ffi::c_int, __request: std::ffi::c_ulong, _: ...) -> std::ffi::c_int;
    static mut gpm_mx: std::ffi::c_int;
    static mut gpm_my: std::ffi::c_int;
    fn close(__fd: std::ffi::c_int) -> std::ffi::c_int;
    fn dup(__fd: std::ffi::c_int) -> std::ffi::c_int;
    fn open(__file: *const std::ffi::c_char, __oflag: std::ffi::c_int, _: ...) -> std::ffi::c_int;
    fn getenv(__name: *const std::ffi::c_char) -> *mut std::ffi::c_char;
}

pub type __dev_t = std::ffi::c_ulong;
pub type __uid_t = std::ffi::c_uint;
pub type __gid_t = std::ffi::c_uint;
pub type __ino_t = std::ffi::c_ulong;
pub type __mode_t = std::ffi::c_uint;
pub type __nlink_t = std::ffi::c_ulong;
pub type __off_t = std::ffi::c_long;
pub type __off64_t = std::ffi::c_long;
pub type __time_t = std::ffi::c_long;
pub type __blksize_t = std::ffi::c_long;
pub type __blkcnt_t = std::ffi::c_long;
pub type __syscall_slong_t = std::ffi::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: std::ffi::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
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
pub struct winsize {
    pub ws_row: std::ffi::c_ushort,
    pub ws_col: std::ffi::c_ushort,
    pub ws_xpixel: std::ffi::c_ushort,
    pub ws_ypixel: std::ffi::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct consolefontdesc {
    pub charcount: std::ffi::c_ushort,
    pub charheight: std::ffi::c_ushort,
    pub chardata: *mut std::ffi::c_char,
}
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

#[unsafe(no_mangle)]
pub static mut linux_d: aa_driver = unsafe {
    {
        let mut init = aa_driver {
            shortname: b"linux\0" as *const u8 as *const std::ffi::c_char,
            name: b"Linux pc console driver 1.0\0" as *const u8 as *const std::ffi::c_char,
            init: Some(
                linux_init
                    as unsafe extern "C" fn(
                        *const aa_hardware_params,
                        *const std::ffi::c_void,
                        *mut aa_hardware_params,
                        *mut *mut std::ffi::c_void,
                    ) -> std::ffi::c_int,
            ),
            uninit: Some(linux_uninit as unsafe extern "C" fn(*mut aa_context) -> ()),
            getsize: Some(
                linux_getsize
                    as unsafe extern "C" fn(
                        *mut aa_context,
                        *mut std::ffi::c_int,
                        *mut std::ffi::c_int,
                    ) -> (),
            ),
            setattr: None,
            print: None,
            gotoxy: Some(
                linux_gotoxy
                    as unsafe extern "C" fn(
                        *mut aa_context,
                        std::ffi::c_int,
                        std::ffi::c_int,
                    ) -> (),
            ),
            flush: Some(linux_flush as unsafe extern "C" fn(*mut aa_context) -> ()),
            cursormode: Some(
                linux_cursor as unsafe extern "C" fn(*mut aa_context, std::ffi::c_int) -> (),
            ),
        };
        init
    }
};

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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub lines: std::ffi::c_uchar,
    pub cols: std::ffi::c_uchar,
    pub x: std::ffi::c_uchar,
    pub y: std::ffi::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub lines: std::ffi::c_uchar,
    pub cols: std::ffi::c_uchar,
    pub x: std::ffi::c_uchar,
    pub y: std::ffi::c_uchar,
}
static mut readonly: std::ffi::c_int = 1 as std::ffi::c_int;
static mut cursorx: std::ffi::c_int = 0;
static mut cursory: std::ffi::c_int = 0;
static mut vc: [*mut FILE; 10] = [0 as *const FILE as *mut FILE; 10];
static mut nvcs: std::ffi::c_int = 0;
static mut sizes: [[std::ffi::c_int; 10]; 2] = [[0; 10]; 2];
static mut cursor_visible: std::ffi::c_int = 1 as std::ffi::c_int;
unsafe extern "C" fn linux_cursor(mut c: *mut aa_context, mut mode: std::ffi::c_int) {
    cursor_visible = mode;
    linux_gotoxy(c, cursorx, cursory);
    if mode != 0 {
        printf(b"\x1B[?25h\0" as *const u8 as *const std::ffi::c_char);
    } else {
        printf(b"\x1B[?25l\0" as *const u8 as *const std::ffi::c_char);
    }
    fflush(stdout);
}
unsafe extern "C" fn linux_init(
    mut p: *const aa_hardware_params,
    mut none: *const std::ffi::c_void,
    mut dest: *mut aa_hardware_params,
    mut params: *mut *mut std::ffi::c_void,
) -> std::ffi::c_int {
    static mut registered: std::ffi::c_int = 0;
    static mut font: aa_font = aa_font {
        data: 0 as *const std::ffi::c_uchar,
        height: 0,
        name: 0 as *const std::ffi::c_char,
        shortname: 0 as *const std::ffi::c_char,
    };
    static mut def: aa_hardware_params = {
        let mut init = aa_hardware_params {
            font: 0 as *const aa_font,
            supported: 2 as std::ffi::c_int
                | 16 as std::ffi::c_int
                | 1 as std::ffi::c_int
                | 4 as std::ffi::c_int
                | (128 as std::ffi::c_int | 256 as std::ffi::c_int),
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
    let mut sbuf: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    let mut major: std::ffi::c_int = 0;
    let mut minor: std::ffi::c_int = 0;
    let mut fname: [std::ffi::c_char; 20] = [0; 20];
    let mut tmp: [std::ffi::c_char; 256] = [0; 256];
    let mut env: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
    let mut vt: std::ffi::c_int = 0;
    let mut i: std::ffi::c_int = 0;
    let mut y: std::ffi::c_int = 0;
    let mut fd: std::ffi::c_int = 0;
    *dest = def;
    fflush(stdout);
    fd = dup(fileno(stderr));
    fstat(fd, &mut sbuf);
    major = (sbuf.st_rdev >> 8 as std::ffi::c_int) as std::ffi::c_int;
    minor = (sbuf.st_rdev & 0xff as std::ffi::c_int as __dev_t) as std::ffi::c_int;
    vt = minor;
    close(fd);
    if major != 4 as std::ffi::c_int || minor >= 64 as std::ffi::c_int {
        return 0 as std::ffi::c_int;
    }
    readonly = 0 as std::ffi::c_int;
    env = getenv(b"AAVCS\0" as *const u8 as *const std::ffi::c_char);
    if !env.is_null() {
        let mut p1: std::ffi::c_int = 0 as std::ffi::c_int;
        let mut p2: std::ffi::c_int = 0;
        nvcs = 0 as std::ffi::c_int;
        while *env.offset(p1 as isize) != 0 {
            while *env.offset(p1 as isize) as std::ffi::c_int != 0
                && *env.offset(p1 as isize) as std::ffi::c_int == ' ' as i32
            {
                p1 += 1;
                p1;
            }
            if *env.offset(p1 as isize) == 0 {
                break;
            }
            p2 = 0 as std::ffi::c_int;
            while *env.offset(p1 as isize) as std::ffi::c_int != 0
                && *env.offset(p1 as isize) as std::ffi::c_int != ' ' as i32
            {
                let fresh0 = p1;
                p1 = p1 + 1;
                let fresh1 = p2;
                p2 = p2 + 1;
                tmp[fresh1 as usize] = *env.offset(fresh0 as isize);
            }
            tmp[p2 as usize] = 0 as std::ffi::c_int as std::ffi::c_char;
            vc[nvcs as usize] = fopen(
                tmp.as_mut_ptr(),
                b"w+\0" as *const u8 as *const std::ffi::c_char,
            );
            if (vc[nvcs as usize]).is_null() {
                vc[nvcs as usize] = fopen(
                    tmp.as_mut_ptr(),
                    b"w\0" as *const u8 as *const std::ffi::c_char,
                );
                readonly = 1 as std::ffi::c_int;
            }
            if (vc[nvcs as usize]).is_null() {
                return 0 as std::ffi::c_int;
            }
            nvcs += 1;
            nvcs;
        }
    } else {
        sprintf(
            fname.as_mut_ptr(),
            b"/dev/vcsa%i\0" as *const u8 as *const std::ffi::c_char,
            vt,
        );
        vc[0 as std::ffi::c_int as usize] = fopen(
            fname.as_mut_ptr(),
            b"w+\0" as *const u8 as *const std::ffi::c_char,
        );
        if (vc[0 as std::ffi::c_int as usize]).is_null() {
            vc[0 as std::ffi::c_int as usize] = fopen(
                tmp.as_mut_ptr(),
                b"w\0" as *const u8 as *const std::ffi::c_char,
            );
            readonly = 1 as std::ffi::c_int;
        }
        nvcs = 1 as std::ffi::c_int;
    }
    if (vc[0 as std::ffi::c_int as usize]).is_null() {
        return 0 as std::ffi::c_int;
    }
    if registered == 0 {
        let mut data: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
        fd = open(
            b"/dev/console\0" as *const u8 as *const std::ffi::c_char,
            0 as std::ffi::c_int,
        );
        if fd >= 0 as std::ffi::c_int {
            let mut buf: [std::ffi::c_char; 32768] = [0; 32768];
            let mut desc: consolefontdesc = consolefontdesc {
                charcount: 0,
                charheight: 0,
                chardata: 0 as *mut std::ffi::c_char,
            };
            desc.chardata = buf.as_mut_ptr();
            desc.charcount = 1024 as std::ffi::c_int as std::ffi::c_ushort;
            i = ioctl(
                fd,
                0x4b6b as std::ffi::c_int as std::ffi::c_ulong,
                &mut desc as *mut consolefontdesc,
            );
            close(fd);
            if i != 0 {
                (*dest).font = &aa_font14;
                (*dest).supported &= !(2 as std::ffi::c_int);
            } else {
                font.name = b"Font used by your console\0" as *const u8 as *const std::ffi::c_char;
                font.shortname = b"current\0" as *const u8 as *const std::ffi::c_char;
                font.height = desc.charheight as std::ffi::c_int;
                data = malloc(
                    (desc.charheight as std::ffi::c_int * 256 as std::ffi::c_int)
                        as std::ffi::c_ulong,
                ) as *mut std::ffi::c_char;
                font.data = data as *const std::ffi::c_uchar;
                if !(font.data).is_null() {
                    y = 0 as std::ffi::c_int;
                    i = 0 as std::ffi::c_int;
                    while i < 8192 as std::ffi::c_int {
                        if (i % 32 as std::ffi::c_int) < font.height {
                            *data.offset(y as isize) = *(desc.chardata).offset(i as isize);
                            y += 1;
                            y;
                        }
                        i += 1;
                        i;
                    }
                    aa_registerfont(&mut font);
                    (*dest).font = &mut font;
                }
            }
        }
    }
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
    aa_recommendlow(
        &mut aa_kbdrecommended,
        b"curses\0" as *const u8 as *const std::ffi::c_char,
    );
    return 1 as std::ffi::c_int;
}
unsafe extern "C" fn linux_uninit(mut c: *mut aa_context) {
    let mut i: std::ffi::c_int = 0;
    i = 0 as std::ffi::c_int;
    while i < nvcs {
        fclose(vc[i as usize]);
        i += 1;
        i;
    }
}
unsafe extern "C" fn linux_getsize(
    mut c: *mut aa_context,
    mut width: *mut std::ffi::c_int,
    mut height: *mut std::ffi::c_int,
) {
    let mut i: std::ffi::c_int = 0;
    let mut scrn: C2RustUnnamed_0 = {
        let mut init = C2RustUnnamed_0 {
            lines: 0 as std::ffi::c_int as std::ffi::c_uchar,
            cols: 0 as std::ffi::c_int as std::ffi::c_uchar,
            x: 0 as std::ffi::c_int as std::ffi::c_uchar,
            y: 0 as std::ffi::c_int as std::ffi::c_uchar,
        };
        init
    };
    *width = 0 as std::ffi::c_int;
    *height = 65536 as std::ffi::c_int;
    if readonly == 0 {
        i = 0 as std::ffi::c_int;
        while i < nvcs {
            fseek(
                vc[i as usize],
                0 as std::ffi::c_int as std::ffi::c_long,
                0 as std::ffi::c_int,
            );
            fread(
                &mut scrn as *mut C2RustUnnamed_0 as *mut std::ffi::c_void,
                4 as std::ffi::c_int as std::ffi::c_ulong,
                1 as std::ffi::c_int as std::ffi::c_ulong,
                vc[i as usize],
            );
            sizes[0 as std::ffi::c_int as usize][i as usize] = scrn.cols as std::ffi::c_int;
            sizes[1 as std::ffi::c_int as usize][i as usize] = scrn.lines as std::ffi::c_int;
            *width = *width + scrn.cols as std::ffi::c_int;
            if *height > scrn.lines as std::ffi::c_int {
                *height = scrn.lines as std::ffi::c_int;
            }
            i += 1;
            i;
        }
    } else {
        let mut ws: winsize = winsize {
            ws_row: 0,
            ws_col: 0,
            ws_xpixel: 0,
            ws_ypixel: 0,
        };
        if ioctl(
            2 as std::ffi::c_int,
            0x5413 as std::ffi::c_int as std::ffi::c_ulong,
            &mut ws as *mut winsize,
        ) == 0 as std::ffi::c_int
        {
            *width = ws.ws_col as std::ffi::c_int * nvcs;
            *height = ws.ws_row as std::ffi::c_int;
        } else {
            *width = 80 as std::ffi::c_int;
            *height = 25 as std::ffi::c_int;
        }
    }
    gpm_mx = *width - 1 as std::ffi::c_int;
    gpm_my = *height - 1 as std::ffi::c_int;
}
unsafe extern "C" fn linux_flush(mut c: *mut aa_context) {
    let mut i: std::ffi::c_int = 0;
    let mut x: std::ffi::c_int = 0;
    let mut y: std::ffi::c_int = 0;
    let mut xstart: std::ffi::c_int = 0 as std::ffi::c_int;
    let mut xend: std::ffi::c_int = 0;
    let mut end: std::ffi::c_int = (*c).params.width * (*c).params.height;
    let mut data: [std::ffi::c_uchar; 6] = [
        0x7 as std::ffi::c_int as std::ffi::c_uchar,
        0x8 as std::ffi::c_int as std::ffi::c_uchar,
        0xf as std::ffi::c_int as std::ffi::c_uchar,
        0xf as std::ffi::c_int as std::ffi::c_uchar,
        0x70 as std::ffi::c_int as std::ffi::c_uchar,
        0x17 as std::ffi::c_int as std::ffi::c_uchar,
    ];
    i = 0 as std::ffi::c_int;
    while i < nvcs {
        fseek(
            vc[i as usize],
            4 as std::ffi::c_int as std::ffi::c_long,
            0 as std::ffi::c_int,
        );
        y = 0 as std::ffi::c_int;
        while y < (*c).params.height {
            let mut start: std::ffi::c_int = y * (*c).params.width;
            x = xstart;
            while x < xstart + sizes[0 as std::ffi::c_int as usize][i as usize] {
                putc(
                    *((*c).textbuffer).offset((x + start) as isize) as std::ffi::c_int,
                    vc[i as usize],
                );
                if (*((*c).attrbuffer).offset((x + start) as isize) as std::ffi::c_int)
                    < 7 as std::ffi::c_int
                {
                    putc(
                        data[*((*c).attrbuffer).offset((x + start) as isize) as usize]
                            as std::ffi::c_int,
                        vc[i as usize],
                    );
                } else {
                    putc(0x27 as std::ffi::c_int, vc[i as usize]);
                }
                x += 1;
                x;
            }
            y += 1;
            y;
        }
        xstart += sizes[0 as std::ffi::c_int as usize][i as usize];
        fflush(vc[i as usize]);
        i += 1;
        i;
    }
}
unsafe extern "C" fn linux_gotoxy(
    mut c: *mut aa_context,
    mut x: std::ffi::c_int,
    mut y: std::ffi::c_int,
) {
    let mut n: std::ffi::c_int = 0 as std::ffi::c_int;
    let mut i: std::ffi::c_int = 0;
    let mut scrn: C2RustUnnamed = C2RustUnnamed {
        lines: 0,
        cols: 0,
        x: 0,
        y: 0,
    };
    cursorx = x;
    cursory = y;
    i = 0 as std::ffi::c_int;
    while i < nvcs {
        fseek(
            vc[i as usize],
            0 as std::ffi::c_int as std::ffi::c_long,
            0 as std::ffi::c_int,
        );
        if x >= n && x < n + sizes[0 as std::ffi::c_int as usize][i as usize] && cursor_visible != 0
        {
            scrn.x = (x - n) as std::ffi::c_uchar;
            scrn.y = y as std::ffi::c_uchar;
            scrn.lines = sizes[0 as std::ffi::c_int as usize][i as usize] as std::ffi::c_uchar;
            scrn.cols = sizes[1 as std::ffi::c_int as usize][i as usize] as std::ffi::c_uchar;
        } else {
            scrn.x = 0 as std::ffi::c_int as std::ffi::c_uchar;
            scrn.y = 0 as std::ffi::c_int as std::ffi::c_uchar;
            scrn.lines = sizes[0 as std::ffi::c_int as usize][i as usize] as std::ffi::c_uchar;
            scrn.cols = sizes[1 as std::ffi::c_int as usize][i as usize] as std::ffi::c_uchar;
        }
        fwrite(
            &mut scrn as *mut C2RustUnnamed as *const std::ffi::c_void,
            4 as std::ffi::c_int as std::ffi::c_ulong,
            1 as std::ffi::c_int as std::ffi::c_ulong,
            vc[i as usize],
        );
        n += sizes[0 as std::ffi::c_int as usize][i as usize];
        i += 1;
        i;
    }
    fflush(vc[i as usize]);
}
