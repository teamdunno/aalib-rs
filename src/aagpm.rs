use super::aastructs::*;

unsafe extern "C" {
    static mut gpm_fd: std::ffi::c_int;
    static mut gpm_hflag: std::ffi::c_int;
    static mut gpm_visiblepointer: std::ffi::c_int;
    static mut gpm_handler: Option<Gpm_Handler>;
    fn Gpm_Open(_: *mut Gpm_Connect, _: std::ffi::c_int) -> std::ffi::c_int;
    fn Gpm_Close() -> i32;
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
    pub clicks: i32,
    pub margin: Gpm_Margin,
    pub wdx: std::ffi::c_short,
    pub wdy: std::ffi::c_short,
}
pub type Gpm_Handler = unsafe extern "C" fn(*mut Gpm_Event, *mut std::ffi::c_void) -> i32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Gpm_Connect {
    pub eventMask: std::ffi::c_ushort,
    pub defaultMask: std::ffi::c_ushort,
    pub minMod: std::ffi::c_ushort,
    pub maxMod: std::ffi::c_ushort,
    pub pid: i32,
    pub vc: i32,
}
static mut conn: Gpm_Connect = Gpm_Connect {
    eventMask: 0,
    defaultMask: 0,
    minMod: 0,
    maxMod: 0,
    pid: 0,
    vc: 0,
};

pub static mut __curses_usegpm: i32 = 0;
static mut mousex: i64 = 0;
static mut mousey: i64 = 0;
static mut mousebuttons: i32 = 0;

pub unsafe extern "C" fn __gpm_user_handler(
    event: *mut Gpm_Event,
    data: *mut std::ffi::c_void,
) -> i32 { unsafe {
    mousex = (*event).x as i64;
    mousey = (*event).y as i64;
    mousebuttons = (*event).buttons as i32;
    (*event).type_0 = ::core::mem::transmute::<std::ffi::c_uint, Gpm_Etype>(
        (*event).type_0 as std::ffi::c_uint & GPM_UP as i32 as std::ffi::c_uint,
    );
    if (*event).type_0 as u64 != 0 {
        mousebuttons = 0 as i32;
    }
    return 0o631 as i32;
}}
unsafe fn gpm_init(context: *mut aa_context, mode: i64) -> i64 { unsafe {
    conn.eventMask = ((if mode & 1 != 0 {
        GPM_MOVE | GPM_DRAG
    } else {
        0
    }) | GPM_DOWN
        | GPM_UP) as std::ffi::c_ushort;
    conn.defaultMask = 0 as i32 as std::ffi::c_ushort;
    conn.maxMod = !(0 as i32) as std::ffi::c_ushort;
    conn.minMod = 0 as i32 as std::ffi::c_ushort;
    if Gpm_Open(&mut conn, 0 as i32) == -(1 as std::ffi::c_int) {
        return 0;
    }
    if gpm_fd < 0 as i32 {
        return 0;
    }
    __curses_usegpm = 1 as i32;
    gpm_handler = Some(
        __gpm_user_handler as unsafe extern "C" fn(*mut Gpm_Event, *mut std::ffi::c_void) -> i32,
    );
    gpm_visiblepointer = 1 as i32;
    gpm_hflag = 1 as i32;
    return 1;
}}
unsafe fn gpm_uninit(c: *mut aa_context) { unsafe {
    __curses_usegpm = 0 as i32;
    Gpm_Close();
}}
unsafe fn gpm_mouse(c: *mut aa_context, x: *mut i64, y: *mut i64, b: *mut i64) { unsafe {
    *x = mousex;
    *y = mousey;
    *b = 0;
    if mousebuttons & 4 != 0 {
        *b |= 1;
    }
    if mousebuttons & 2 != 0 {
        *b |= 2;
    }
    if mousebuttons & 1 != 0 {
        *b |= 4;
    }
}}
unsafe fn gpm_mousemode(c: *mut aa_context, m: i64) { unsafe {
    gpm_visiblepointer = m as i32;
}}

pub static mut mouse_gpm_d: aa_mousedriver = unsafe {
    {
        let init = aa_mousedriver {
            shortname: b"gpm\0" as *const u8 as *const std::ffi::c_char,
            name: b"Gpm mouse driver 1.0\0" as *const u8 as *const std::ffi::c_char,
            flags: 7 | 8,
            init: Some(gpm_init),
            uninit: Some(gpm_uninit),
            getmouse: Some(gpm_mouse),
            cursormode: Some(gpm_mousemode),
        };
        init
    }
};
