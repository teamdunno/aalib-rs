use super::aastructs::*;
use x11_dl::xlib::{_XDisplay, _XGC};
unsafe extern "C" {
    fn XSelectInput(_: *mut Display, _: Window, _: std::ffi::c_long) -> std::ffi::c_int;
    static X11_d: aa_driver;
}
pub type XID = std::ffi::c_ulong;
pub type Atom = std::ffi::c_ulong;
pub type Window = XID;
pub type Font = XID;
pub type Pixmap = XID;
pub type Cursor = XID;
pub type Colormap = XID;
pub type XPointer = *mut std::ffi::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XExtData {
    pub number: std::ffi::c_int,
    pub next: *mut _XExtData,
    pub free_private: Option<unsafe extern "C" fn(*mut _XExtData) -> std::ffi::c_int>,
    pub private_data: XPointer,
}
pub type XExtData = _XExtData;
pub type GC = *mut _XGC;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XSetWindowAttributes {
    pub background_pixmap: Pixmap,
    pub background_pixel: std::ffi::c_ulong,
    pub border_pixmap: Pixmap,
    pub border_pixel: std::ffi::c_ulong,
    pub bit_gravity: std::ffi::c_int,
    pub win_gravity: std::ffi::c_int,
    pub backing_store: std::ffi::c_int,
    pub backing_planes: std::ffi::c_ulong,
    pub backing_pixel: std::ffi::c_ulong,
    pub save_under: std::ffi::c_int,
    pub event_mask: std::ffi::c_long,
    pub do_not_propagate_mask: std::ffi::c_long,
    pub override_redirect: std::ffi::c_int,
    pub colormap: Colormap,
    pub cursor: Cursor,
}
pub type Display = _XDisplay;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XCharStruct {
    pub lbearing: std::ffi::c_short,
    pub rbearing: std::ffi::c_short,
    pub width: std::ffi::c_short,
    pub ascent: std::ffi::c_short,
    pub descent: std::ffi::c_short,
    pub attributes: std::ffi::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XFontProp {
    pub name: Atom,
    pub card32: std::ffi::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XFontStruct {
    pub ext_data: *mut XExtData,
    pub fid: Font,
    pub direction: std::ffi::c_uint,
    pub min_char_or_byte2: std::ffi::c_uint,
    pub max_char_or_byte2: std::ffi::c_uint,
    pub min_byte1: std::ffi::c_uint,
    pub max_byte1: std::ffi::c_uint,
    pub all_chars_exist: std::ffi::c_int,
    pub default_char: std::ffi::c_uint,
    pub n_properties: std::ffi::c_int,
    pub properties: *mut XFontProp,
    pub min_bounds: XCharStruct,
    pub max_bounds: XCharStruct,
    pub per_char: *mut XCharStruct,
    pub ascent: std::ffi::c_int,
    pub descent: std::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xdriverdata {
    pub dp: *mut Display,
    pub wi: Window,
    pub pi: Pixmap,
    pub imgwidth: std::ffi::c_int,
    pub imgheight: std::ffi::c_int,
    pub attr: XSetWindowAttributes,
    pub fontheight: std::ffi::c_int,
    pub fontwidth: std::ffi::c_int,
    pub realfontwidth: std::ffi::c_int,
    pub normalGC: GC,
    pub dimGC: GC,
    pub boldGC: GC,
    pub currGC: GC,
    pub specialGC: GC,
    pub cvisible: std::ffi::c_int,
    pub blackGC: GC,
    pub cmap: Colormap,
    pub screen: std::ffi::c_int,
    pub bold: std::ffi::c_long,
    pub normal: std::ffi::c_long,
    pub dim: std::ffi::c_long,
    pub black: std::ffi::c_long,
    pub special: std::ffi::c_long,
    pub invertedbold: std::ffi::c_long,
    pub invertednormal: std::ffi::c_long,
    pub inverteddim: std::ffi::c_long,
    pub invertedblack: std::ffi::c_long,
    pub invertedspecial: std::ffi::c_long,
    pub width: std::ffi::c_int,
    pub height: std::ffi::c_int,
    pub font_s: *mut XFontStruct,
    pub Xpos: std::ffi::c_int,
    pub pixmapmode: std::ffi::c_int,
    pub Ypos: std::ffi::c_int,
    pub mmwidth: std::ffi::c_int,
    pub mmheight: std::ffi::c_int,
    pub previoust: *mut std::ffi::c_uchar,
    pub previousa: *mut std::ffi::c_uchar,
    pub font: std::ffi::c_int,
    pub pixelwidth: std::ffi::c_int,
    pub pixelheight: std::ffi::c_int,
    pub inverted: std::ffi::c_int,
}
unsafe extern "C" fn X_init(mut c: *mut aa_context, mut mode: std::ffi::c_int) -> std::ffi::c_int {
    let mut d: *mut xdriverdata = (*c).driverdata as *mut xdriverdata;
    if (*c).driver != &X11_d as *const aa_driver {
        return 0 as std::ffi::c_int;
    }
    (*d).attr.event_mask |= (1 as std::ffi::c_long) << 2 as std::ffi::c_int
        | (1 as std::ffi::c_long) << 3 as std::ffi::c_int
        | (if mode & 1 as std::ffi::c_int != 0 {
            (1 as std::ffi::c_long) << 6 as std::ffi::c_int
        } else {
            0 as std::ffi::c_int as std::ffi::c_long
        });
    XSelectInput((*d).dp, (*d).wi, (*d).attr.event_mask);
    return 1 as std::ffi::c_int;
}
unsafe extern "C" fn X_uninit(mut c: *mut aa_context) {
    let mut d: *mut xdriverdata = (*c).driverdata as *mut xdriverdata;
    (*d).attr.event_mask &= !((1 as std::ffi::c_long) << 2 as std::ffi::c_int
        | (1 as std::ffi::c_long) << 3 as std::ffi::c_int
        | (1 as std::ffi::c_long) << 6 as std::ffi::c_int
        | (1 as std::ffi::c_long) << 13 as std::ffi::c_int);
    XSelectInput((*d).dp, (*d).wi, (*d).attr.event_mask);
}

pub static mut __X_mousex: std::ffi::c_int = 0;
pub static mut __X_mousey: std::ffi::c_int = 0;
pub static mut __X_buttons: std::ffi::c_int = 0;
unsafe extern "C" fn X_getmouse(
    mut c: *mut aa_context,
    mut x: *mut std::ffi::c_int,
    mut y: *mut std::ffi::c_int,
    mut b: *mut std::ffi::c_int,
) {
    let mut d: *mut xdriverdata = (*c).driverdata as *mut xdriverdata;
    *x = __X_mousex / (*d).fontwidth;
    *y = __X_mousey / (*d).fontheight;
    *b = 0 as std::ffi::c_int;
    if __X_buttons & (1 as std::ffi::c_int) << 8 as std::ffi::c_int != 0 {
        *b |= 1 as std::ffi::c_int;
    }
    if __X_buttons & (1 as std::ffi::c_int) << 9 as std::ffi::c_int != 0 {
        *b |= 2 as std::ffi::c_int;
    }
    if __X_buttons & (1 as std::ffi::c_int) << 10 as std::ffi::c_int != 0 {
        *b |= 4 as std::ffi::c_int;
    }
}

pub static mut mouse_X11_d: aa_mousedriver = unsafe {
    {
        let mut init = aa_mousedriver {
            shortname: b"X11\0" as *const u8 as *const std::ffi::c_char,
            name: b"X11 mouse driver 1.0\0" as *const u8 as *const std::ffi::c_char,
            flags: 7 as std::ffi::c_int,
            init: Some(
                X_init as unsafe extern "C" fn(*mut aa_context, std::ffi::c_int) -> std::ffi::c_int,
            ),
            uninit: Some(X_uninit as unsafe extern "C" fn(*mut aa_context) -> ()),
            getmouse: Some(
                X_getmouse
                    as unsafe extern "C" fn(
                        *mut aa_context,
                        *mut std::ffi::c_int,
                        *mut std::ffi::c_int,
                        *mut std::ffi::c_int,
                    ) -> (),
            ),
            cursormode: None,
        };
        init
    }
};
