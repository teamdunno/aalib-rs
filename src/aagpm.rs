unsafe extern "C" {
    static mut gpm_fd: std::ffi::c_int;
    static mut gpm_hflag: std::ffi::c_int;
    static mut gpm_visiblepointer: std::ffi::c_int;
    static mut gpm_handler: Option<Gpm_Handler>;
    fn Gpm_Open(_: *mut Gpm_Connect, _: std::ffi::c_int) -> std::ffi::c_int;
    fn Gpm_Close() -> std::ffi::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Gpm_Connect {
    pub eventMask: std::ffi::c_ushort,
    pub defaultMask: std::ffi::c_ushort,
    pub minMod: std::ffi::c_ushort,
    pub maxMod: std::ffi::c_ushort,
    pub pid: std::ffi::c_int,
    pub vc: std::ffi::c_int,
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
static mut conn: Gpm_Connect = Gpm_Connect {
    eventMask: 0,
    defaultMask: 0,
    minMod: 0,
    maxMod: 0,
    pid: 0,
    vc: 0,
};

pub static mut __curses_usegpm: std::ffi::c_int = 0;
static mut mousex: std::ffi::c_int = 0;
static mut mousey: std::ffi::c_int = 0;
static mut mousebuttons: std::ffi::c_int = 0;

pub unsafe extern "C" fn __gpm_user_handler(
    mut event: *mut Gpm_Event,
    mut data: *mut std::ffi::c_void,
) -> std::ffi::c_int {
    mousex = (*event).x as std::ffi::c_int;
    mousey = (*event).y as std::ffi::c_int;
    mousebuttons = (*event).buttons as std::ffi::c_int;
    (*event).type_0 = ::core::mem::transmute::<std::ffi::c_uint, Gpm_Etype>(
        (*event).type_0 as std::ffi::c_uint & GPM_UP as std::ffi::c_int as std::ffi::c_uint,
    );
    if (*event).type_0 as u64 != 0 {
        mousebuttons = 0 as std::ffi::c_int;
    }
    return 0o631 as std::ffi::c_int;
}
unsafe extern "C" fn gpm_init(
    mut context: *mut aa_context,
    mut mode: std::ffi::c_int,
) -> std::ffi::c_int {
    conn.eventMask = ((if mode & 1 as std::ffi::c_int != 0 {
        GPM_MOVE as std::ffi::c_int | GPM_DRAG as std::ffi::c_int
    } else {
        0 as std::ffi::c_int
    }) | GPM_DOWN as std::ffi::c_int
        | GPM_UP as std::ffi::c_int) as std::ffi::c_ushort;
    conn.defaultMask = 0 as std::ffi::c_int as std::ffi::c_ushort;
    conn.maxMod = !(0 as std::ffi::c_int) as std::ffi::c_ushort;
    conn.minMod = 0 as std::ffi::c_int as std::ffi::c_ushort;
    if Gpm_Open(&mut conn, 0 as std::ffi::c_int) == -(1 as std::ffi::c_int) {
        return 0 as std::ffi::c_int;
    }
    if gpm_fd < 0 as std::ffi::c_int {
        return 0 as std::ffi::c_int;
    }
    __curses_usegpm = 1 as std::ffi::c_int;
    gpm_handler = Some(
        __gpm_user_handler
            as unsafe extern "C" fn(*mut Gpm_Event, *mut std::ffi::c_void) -> std::ffi::c_int,
    );
    gpm_visiblepointer = 1 as std::ffi::c_int;
    gpm_hflag = 1 as std::ffi::c_int;
    return 1 as std::ffi::c_int;
}
unsafe extern "C" fn gpm_uninit(mut c: *mut aa_context) {
    __curses_usegpm = 0 as std::ffi::c_int;
    Gpm_Close();
}
unsafe extern "C" fn gpm_mouse(
    mut c: *mut aa_context,
    mut x: *mut std::ffi::c_int,
    mut y: *mut std::ffi::c_int,
    mut b: *mut std::ffi::c_int,
) {
    *x = mousex;
    *y = mousey;
    *b = 0 as std::ffi::c_int;
    if mousebuttons & 4 as std::ffi::c_int != 0 {
        *b |= 1 as std::ffi::c_int;
    }
    if mousebuttons & 2 as std::ffi::c_int != 0 {
        *b |= 2 as std::ffi::c_int;
    }
    if mousebuttons & 1 as std::ffi::c_int != 0 {
        *b |= 4 as std::ffi::c_int;
    }
}
unsafe extern "C" fn gpm_mousemode(mut c: *mut aa_context, mut m: std::ffi::c_int) {
    gpm_visiblepointer = m;
}

pub static mut mouse_gpm_d: aa_mousedriver = unsafe {
    {
        let mut init = aa_mousedriver {
            shortname: b"gpm\0" as *const u8 as *const std::ffi::c_char,
            name: b"Gpm mouse driver 1.0\0" as *const u8 as *const std::ffi::c_char,
            flags: 7 as std::ffi::c_int | 8 as std::ffi::c_int,
            init: Some(
                gpm_init
                    as unsafe extern "C" fn(*mut aa_context, std::ffi::c_int) -> std::ffi::c_int,
            ),
            uninit: Some(gpm_uninit as unsafe extern "C" fn(*mut aa_context) -> ()),
            getmouse: Some(
                gpm_mouse
                    as unsafe extern "C" fn(
                        *mut aa_context,
                        *mut std::ffi::c_int,
                        *mut std::ffi::c_int,
                        *mut std::ffi::c_int,
                    ) -> (),
            ),
            cursormode: Some(
                gpm_mousemode as unsafe extern "C" fn(*mut aa_context, std::ffi::c_int) -> (),
            ),
        };
        init
    }
};
