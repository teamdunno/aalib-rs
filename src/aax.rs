use super::aafonts::aa_registerfont;
use super::aarec::{aa_kbdrecommended, aa_recommendlow};
use super::aastructs::*;
use super::fontx13b::aa_fontX13B;
use wrflib_x11_sys::_XPrivate;
use x11_dl::xlib::{_XDisplay, _XGC, _XrmHashBucketRec};

unsafe extern "C" {
    fn malloc(_: std::ffi::c_ulong) -> *mut std::ffi::c_void;
    fn calloc(_: std::ffi::c_ulong, _: std::ffi::c_ulong) -> *mut std::ffi::c_void;
    fn free(_: *mut std::ffi::c_void);
    fn getenv(__name: *const std::ffi::c_char) -> *mut std::ffi::c_char;
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
    fn XQueryFont(_: *mut Display, _: XID) -> *mut XFontStruct;
    fn XGetImage(
        _: *mut Display,
        _: Drawable,
        _: std::ffi::c_int,
        _: std::ffi::c_int,
        _: std::ffi::c_uint,
        _: std::ffi::c_uint,
        _: std::ffi::c_ulong,
        _: std::ffi::c_int,
    ) -> *mut XImage;
    fn XOpenDisplay(_: *const std::ffi::c_char) -> *mut Display;
    fn XLoadFont(_: *mut Display, _: *const std::ffi::c_char) -> Font;
    fn XCreateGC(_: *mut Display, _: Drawable, _: std::ffi::c_ulong, _: *mut XGCValues) -> GC;
    fn XCreatePixmap(
        _: *mut Display,
        _: Drawable,
        _: std::ffi::c_uint,
        _: std::ffi::c_uint,
        _: std::ffi::c_uint,
    ) -> Pixmap;
    fn XCreateWindow(
        _: *mut Display,
        _: Window,
        _: std::ffi::c_int,
        _: std::ffi::c_int,
        _: std::ffi::c_uint,
        _: std::ffi::c_uint,
        _: std::ffi::c_uint,
        _: std::ffi::c_int,
        _: std::ffi::c_uint,
        _: *mut Visual,
        _: std::ffi::c_ulong,
        _: *mut XSetWindowAttributes,
    ) -> Window;
    fn XAllocColor(_: *mut Display, _: Colormap, _: *mut XColor) -> std::ffi::c_int;
    fn XClearArea(
        _: *mut Display,
        _: Window,
        _: std::ffi::c_int,
        _: std::ffi::c_int,
        _: std::ffi::c_uint,
        _: std::ffi::c_uint,
        _: std::ffi::c_int,
    ) -> std::ffi::c_int;
    fn XClearWindow(_: *mut Display, _: Window) -> std::ffi::c_int;
    fn XCloseDisplay(_: *mut Display) -> std::ffi::c_int;
    fn XDrawLine(
        _: *mut Display,
        _: Drawable,
        _: GC,
        _: std::ffi::c_int,
        _: std::ffi::c_int,
        _: std::ffi::c_int,
        _: std::ffi::c_int,
    ) -> std::ffi::c_int;
    fn XDrawString(
        _: *mut Display,
        _: Drawable,
        _: GC,
        _: std::ffi::c_int,
        _: std::ffi::c_int,
        _: *const std::ffi::c_char,
        _: std::ffi::c_int,
    ) -> std::ffi::c_int;
    fn XDrawText(
        _: *mut Display,
        _: Drawable,
        _: GC,
        _: std::ffi::c_int,
        _: std::ffi::c_int,
        _: *mut XTextItem,
        _: std::ffi::c_int,
    ) -> std::ffi::c_int;
    fn XFillRectangle(
        _: *mut Display,
        _: Drawable,
        _: GC,
        _: std::ffi::c_int,
        _: std::ffi::c_int,
        _: std::ffi::c_uint,
        _: std::ffi::c_uint,
    ) -> std::ffi::c_int;
    fn XFillRectangles(
        _: *mut Display,
        _: Drawable,
        _: GC,
        _: *mut XRectangle,
        _: std::ffi::c_int,
    ) -> std::ffi::c_int;
    fn XFlush(_: *mut Display) -> std::ffi::c_int;
    fn XFreeGC(_: *mut Display, _: GC) -> std::ffi::c_int;
    fn XFreePixmap(_: *mut Display, _: Pixmap) -> std::ffi::c_int;
    fn XGetGeometry(
        _: *mut Display,
        _: Drawable,
        _: *mut Window,
        _: *mut std::ffi::c_int,
        _: *mut std::ffi::c_int,
        _: *mut std::ffi::c_uint,
        _: *mut std::ffi::c_uint,
        _: *mut std::ffi::c_uint,
        _: *mut std::ffi::c_uint,
    ) -> std::ffi::c_int;
    fn XMapWindow(_: *mut Display, _: Window) -> std::ffi::c_int;
    fn XSetBackground(_: *mut Display, _: GC, _: std::ffi::c_ulong) -> std::ffi::c_int;
    fn XSetFont(_: *mut Display, _: GC, _: Font) -> std::ffi::c_int;
    fn XSetForeground(_: *mut Display, _: GC, _: std::ffi::c_ulong) -> std::ffi::c_int;
    fn XSetWindowBackground(_: *mut Display, _: Window, _: std::ffi::c_ulong) -> std::ffi::c_int;
    fn XSetWindowBackgroundPixmap(_: *mut Display, _: Window, _: Pixmap) -> std::ffi::c_int;
    fn XStoreName(_: *mut Display, _: Window, _: *const std::ffi::c_char) -> std::ffi::c_int;
    fn XSync(_: *mut Display, _: std::ffi::c_int) -> std::ffi::c_int;
}
pub type XID = std::ffi::c_ulong;
pub type Atom = std::ffi::c_ulong;
pub type VisualID = std::ffi::c_ulong;
pub type Window = XID;
pub type Drawable = XID;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XGCValues {
    pub function: std::ffi::c_int,
    pub plane_mask: std::ffi::c_ulong,
    pub foreground: std::ffi::c_ulong,
    pub background: std::ffi::c_ulong,
    pub line_width: std::ffi::c_int,
    pub line_style: std::ffi::c_int,
    pub cap_style: std::ffi::c_int,
    pub join_style: std::ffi::c_int,
    pub fill_style: std::ffi::c_int,
    pub fill_rule: std::ffi::c_int,
    pub arc_mode: std::ffi::c_int,
    pub tile: Pixmap,
    pub stipple: Pixmap,
    pub ts_x_origin: std::ffi::c_int,
    pub ts_y_origin: std::ffi::c_int,
    pub font: Font,
    pub subwindow_mode: std::ffi::c_int,
    pub graphics_exposures: std::ffi::c_int,
    pub clip_x_origin: std::ffi::c_int,
    pub clip_y_origin: std::ffi::c_int,
    pub clip_mask: Pixmap,
    pub dash_offset: std::ffi::c_int,
    pub dashes: std::ffi::c_char,
}
pub type GC = *mut _XGC;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Visual {
    pub ext_data: *mut XExtData,
    pub visualid: VisualID,
    pub class: std::ffi::c_int,
    pub red_mask: std::ffi::c_ulong,
    pub green_mask: std::ffi::c_ulong,
    pub blue_mask: std::ffi::c_ulong,
    pub bits_per_rgb: std::ffi::c_int,
    pub map_entries: std::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Depth {
    pub depth: std::ffi::c_int,
    pub nvisuals: std::ffi::c_int,
    pub visuals: *mut Visual,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Screen {
    pub ext_data: *mut XExtData,
    pub display: *mut _XDisplay,
    pub root: Window,
    pub width: std::ffi::c_int,
    pub height: std::ffi::c_int,
    pub mwidth: std::ffi::c_int,
    pub mheight: std::ffi::c_int,
    pub ndepths: std::ffi::c_int,
    pub depths: *mut Depth,
    pub root_depth: std::ffi::c_int,
    pub root_visual: *mut Visual,
    pub default_gc: GC,
    pub cmap: Colormap,
    pub white_pixel: std::ffi::c_ulong,
    pub black_pixel: std::ffi::c_ulong,
    pub max_maps: std::ffi::c_int,
    pub min_maps: std::ffi::c_int,
    pub backing_store: std::ffi::c_int,
    pub save_unders: std::ffi::c_int,
    pub root_input_mask: std::ffi::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ScreenFormat {
    pub ext_data: *mut XExtData,
    pub depth: std::ffi::c_int,
    pub bits_per_pixel: std::ffi::c_int,
    pub scanline_pad: std::ffi::c_int,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XImage {
    pub width: std::ffi::c_int,
    pub height: std::ffi::c_int,
    pub xoffset: std::ffi::c_int,
    pub format: std::ffi::c_int,
    pub data: *mut std::ffi::c_char,
    pub byte_order: std::ffi::c_int,
    pub bitmap_unit: std::ffi::c_int,
    pub bitmap_bit_order: std::ffi::c_int,
    pub bitmap_pad: std::ffi::c_int,
    pub depth: std::ffi::c_int,
    pub bytes_per_line: std::ffi::c_int,
    pub bits_per_pixel: std::ffi::c_int,
    pub red_mask: std::ffi::c_ulong,
    pub green_mask: std::ffi::c_ulong,
    pub blue_mask: std::ffi::c_ulong,
    pub obdata: XPointer,
    pub f: funcs,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct funcs {
    pub create_image: Option<
        unsafe extern "C" fn(
            *mut _XDisplay,
            *mut Visual,
            std::ffi::c_uint,
            std::ffi::c_int,
            std::ffi::c_int,
            *mut std::ffi::c_char,
            std::ffi::c_uint,
            std::ffi::c_uint,
            std::ffi::c_int,
            std::ffi::c_int,
        ) -> *mut _XImage,
    >,
    pub destroy_image: Option<unsafe extern "C" fn(*mut _XImage) -> std::ffi::c_int>,
    pub get_pixel: Option<
        unsafe extern "C" fn(*mut _XImage, std::ffi::c_int, std::ffi::c_int) -> std::ffi::c_ulong,
    >,
    pub put_pixel: Option<
        unsafe extern "C" fn(
            *mut _XImage,
            std::ffi::c_int,
            std::ffi::c_int,
            std::ffi::c_ulong,
        ) -> std::ffi::c_int,
    >,
    pub sub_image: Option<
        unsafe extern "C" fn(
            *mut _XImage,
            std::ffi::c_int,
            std::ffi::c_int,
            std::ffi::c_uint,
            std::ffi::c_uint,
        ) -> *mut _XImage,
    >,
    pub add_pixel: Option<unsafe extern "C" fn(*mut _XImage, std::ffi::c_long) -> std::ffi::c_int>,
}
pub type XImage = _XImage;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XColor {
    pub pixel: std::ffi::c_ulong,
    pub red: std::ffi::c_ushort,
    pub green: std::ffi::c_ushort,
    pub blue: std::ffi::c_ushort,
    pub flags: std::ffi::c_char,
    pub pad: std::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XRectangle {
    pub x: std::ffi::c_short,
    pub y: std::ffi::c_short,
    pub width: std::ffi::c_ushort,
    pub height: std::ffi::c_ushort,
}
pub type Display = _XDisplay;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub ext_data: *mut XExtData,
    pub private1: *mut _XPrivate,
    pub fd: std::ffi::c_int,
    pub private2: std::ffi::c_int,
    pub proto_major_version: std::ffi::c_int,
    pub proto_minor_version: std::ffi::c_int,
    pub vendor: *mut std::ffi::c_char,
    pub private3: XID,
    pub private4: XID,
    pub private5: XID,
    pub private6: std::ffi::c_int,
    pub resource_alloc: Option<unsafe extern "C" fn(*mut _XDisplay) -> XID>,
    pub byte_order: std::ffi::c_int,
    pub bitmap_unit: std::ffi::c_int,
    pub bitmap_pad: std::ffi::c_int,
    pub bitmap_bit_order: std::ffi::c_int,
    pub nformats: std::ffi::c_int,
    pub pixmap_format: *mut ScreenFormat,
    pub private8: std::ffi::c_int,
    pub release: std::ffi::c_int,
    pub private9: *mut _XPrivate,
    pub private10: *mut _XPrivate,
    pub qlen: std::ffi::c_int,
    pub last_request_read: std::ffi::c_ulong,
    pub request: std::ffi::c_ulong,
    pub private11: XPointer,
    pub private12: XPointer,
    pub private13: XPointer,
    pub private14: XPointer,
    pub max_request_size: std::ffi::c_uint,
    pub db: *mut _XrmHashBucketRec,
    pub private15: Option<unsafe extern "C" fn(*mut _XDisplay) -> std::ffi::c_int>,
    pub display_name: *mut std::ffi::c_char,
    pub default_screen: std::ffi::c_int,
    pub nscreens: std::ffi::c_int,
    pub screens: *mut Screen,
    pub motion_buffer: std::ffi::c_ulong,
    pub private16: std::ffi::c_ulong,
    pub min_keycode: std::ffi::c_int,
    pub max_keycode: std::ffi::c_int,
    pub private17: XPointer,
    pub private18: XPointer,
    pub private19: std::ffi::c_int,
    pub xdefaults: *mut std::ffi::c_char,
}
pub type _XPrivDisplay = *mut C2RustUnnamed;
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
pub struct XTextItem {
    pub chars: *mut std::ffi::c_char,
    pub nchars: std::ffi::c_int,
    pub delta: std::ffi::c_int,
    pub font: Font,
}
pub type aa_attribute = std::ffi::c_uint;
pub const AA_SPECIAL: aa_attribute = 5;
pub const AA_REVERSE: aa_attribute = 4;
pub const AA_BOLDFONT: aa_attribute = 3;
pub const AA_BOLD: aa_attribute = 2;
pub const AA_DIM: aa_attribute = 1;
pub const AA_NORMAL: aa_attribute = 0;
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

#[unsafe(no_mangle)]
pub static mut X11_d: aa_driver = unsafe {
    {
        let mut init = aa_driver {
            shortname: b"X11\0" as *const u8 as *const std::ffi::c_char,
            name: b"X11 driver 1.1\0" as *const u8 as *const std::ffi::c_char,
            init: Some(
                X_init
                    as unsafe extern "C" fn(
                        *const aa_hardware_params,
                        *const std::ffi::c_void,
                        *mut aa_hardware_params,
                        *mut *mut std::ffi::c_void,
                    ) -> std::ffi::c_int,
            ),
            uninit: Some(X_uninit as unsafe extern "C" fn(*mut aa_context) -> ()),
            getsize: Some(
                X_getsize
                    as unsafe extern "C" fn(
                        *mut aa_context,
                        *mut std::ffi::c_int,
                        *mut std::ffi::c_int,
                    ) -> (),
            ),
            setattr: None,
            print: None,
            gotoxy: Some(
                X_gotoxy
                    as unsafe extern "C" fn(
                        *mut aa_context,
                        std::ffi::c_int,
                        std::ffi::c_int,
                    ) -> (),
            ),
            flush: Some(X_flush as unsafe extern "C" fn(*mut aa_context) -> ()),
            cursormode: Some(
                X_cursor as unsafe extern "C" fn(*mut aa_context, std::ffi::c_int) -> (),
            ),
        };
        init
    }
};
static mut font_error: std::ffi::c_int = 0;
unsafe extern "C" fn mygetpixel(
    mut image: *mut XImage,
    mut pos: std::ffi::c_int,
    mut y: std::ffi::c_int,
) -> std::ffi::c_int {
    let mut width: std::ffi::c_int = (*image).width;
    let mut i: std::ffi::c_int = 0;
    let mut sum: std::ffi::c_int = font_error;
    let mut start: std::ffi::c_int = (pos * width + 4 as std::ffi::c_int) / 8 as std::ffi::c_int;
    let mut end: std::ffi::c_int =
        ((pos + 1 as std::ffi::c_int) * width + 4 as std::ffi::c_int) / 8 as std::ffi::c_int;
    if start == end {
        if start == (*image).width - 1 as std::ffi::c_int {
            start -= 1;
            start;
        } else {
            end += 1;
            end;
        }
    }
    i = start;
    while i < end {
        sum += ((Some(((*image).f.get_pixel).expect("non-null function pointer")))
            .expect("non-null function pointer")(image, i, y)
            != 0 as std::ffi::c_int as std::ffi::c_ulong) as std::ffi::c_int;
        i += 1;
        i;
    }
    if sum <= (end - start) / 2 as std::ffi::c_int {
        font_error = sum;
        return 0 as std::ffi::c_int;
    } else {
        font_error = -(end - start - sum);
    }
    return 1 as std::ffi::c_int;
}
unsafe extern "C" fn X_AllocColors(mut d: *mut xdriverdata) {
    static mut c: XColor = XColor {
        pixel: 0,
        red: 0,
        green: 0,
        blue: 0,
        flags: 0,
        pad: 0,
    };
    (*d).invertedbold = (*((*((*d).dp as _XPrivDisplay)).screens).offset((*d).screen as isize))
        .black_pixel as std::ffi::c_long;
    (*d).attr.background_pixel = (*d).invertedbold as std::ffi::c_ulong;
    (*d).attr.border_pixel = (*d).attr.background_pixel;
    (*d).black = (*d).attr.border_pixel as std::ffi::c_long;
    (*d).invertedblack = (*((*((*d).dp as _XPrivDisplay)).screens).offset((*d).screen as isize))
        .white_pixel as std::ffi::c_long;
    (*d).bold = (*d).invertedblack;
    c.red = (0xb2 as std::ffi::c_int * 256 as std::ffi::c_int) as std::ffi::c_ushort;
    c.green = (0xb2 as std::ffi::c_int * 256 as std::ffi::c_int) as std::ffi::c_ushort;
    c.blue = (0xb2 as std::ffi::c_int * 256 as std::ffi::c_int) as std::ffi::c_ushort;
    if XAllocColor((*d).dp, (*d).cmap, &mut c) == 0 {
        (*d).normal = (*d).bold;
    } else {
        (*d).normal = c.pixel as std::ffi::c_long;
    }
    c.red = (65536 as u32 - c.red as u32) as std::ffi::c_ushort;
    c.green = (65536 as u32 - c.green as u32) as std::ffi::c_ushort;
    c.blue = (65536 as u32 - c.blue as u32) as std::ffi::c_ushort;
    if XAllocColor((*d).dp, (*d).cmap, &mut c) == 0 {
        (*d).invertednormal = (*d).invertedbold;
        (*d).normal = (*d).bold;
    } else {
        (*d).invertednormal = c.pixel as std::ffi::c_long;
    }
    c.red = (0x68 as std::ffi::c_int * 256 as std::ffi::c_int) as std::ffi::c_ushort;
    c.green = (0x68 as std::ffi::c_int * 256 as std::ffi::c_int) as std::ffi::c_ushort;
    c.blue = (0x68 as std::ffi::c_int * 256 as std::ffi::c_int) as std::ffi::c_ushort;
    if (*d).bold == (*d).dim && XAllocColor((*d).dp, (*d).cmap, &mut c) == 0 {
        (*d).dim = (*d).normal;
    } else {
        (*d).dim = c.pixel as std::ffi::c_long;
    }
    c.red = (65536 as u32 - c.red as u32) as std::ffi::c_ushort;
    c.green = (65536 as u32 - c.green as u32) as std::ffi::c_ushort;
    c.blue = (65536 as u32 - c.blue as u32) as std::ffi::c_ushort;
    if XAllocColor((*d).dp, (*d).cmap, &mut c) == 0 {
        (*d).inverteddim = (*d).invertednormal;
        (*d).dim = (*d).normal;
    } else {
        (*d).inverteddim = c.pixel as std::ffi::c_long;
    }
    c.red = 0 as std::ffi::c_int as std::ffi::c_ushort;
    c.green = 0 as std::ffi::c_int as std::ffi::c_ushort;
    c.blue = 65535 as std::ffi::c_ulong as std::ffi::c_ushort;
    if XAllocColor((*d).dp, (*d).cmap, &mut c) == 0 {
        (*d).special = (*d).black;
    } else {
        (*d).special = c.pixel as std::ffi::c_long;
    }
    c.red = (65535 as std::ffi::c_ulong).wrapping_div(2 as std::ffi::c_int as std::ffi::c_ulong)
        as std::ffi::c_ushort;
    c.green = (65535 as std::ffi::c_ulong).wrapping_div(2 as std::ffi::c_int as std::ffi::c_ulong)
        as std::ffi::c_ushort;
    c.blue = 65535 as std::ffi::c_ulong as std::ffi::c_ushort;
    if XAllocColor((*d).dp, (*d).cmap, &mut c) == 0 {
        (*d).invertedspecial = (*d).invertedblack;
    } else {
        (*d).invertedspecial = c.pixel as std::ffi::c_long;
    };
}
unsafe extern "C" fn X_setinversionmode(mut inverted: std::ffi::c_int, mut d: *mut xdriverdata) {
    (*d).inverted = inverted;
    if !((*d).specialGC).is_null() {
        XFreeGC((*d).dp, (*d).specialGC);
    }
    if !((*d).normalGC).is_null() {
        XFreeGC((*d).dp, (*d).normalGC);
    }
    if !((*d).boldGC).is_null() {
        XFreeGC((*d).dp, (*d).boldGC);
    }
    if !((*d).dimGC).is_null() {
        XFreeGC((*d).dp, (*d).dimGC);
    }
    (*d).specialGC = XCreateGC(
        (*d).dp,
        (*d).wi,
        0 as std::ffi::c_long as std::ffi::c_ulong,
        0 as *mut XGCValues,
    );
    XSetForeground(
        (*d).dp,
        (*d).specialGC,
        (if inverted != 0 {
            (*d).invertedspecial
        } else {
            (*d).special
        }) as std::ffi::c_ulong,
    );
    // XSetFont((*d).dp, (*d).specialGC, (*d).font as Font);
    (*d).normalGC = XCreateGC(
        (*d).dp,
        (*d).wi,
        0 as std::ffi::c_long as std::ffi::c_ulong,
        0 as *mut XGCValues,
    );
    XSetForeground(
        (*d).dp,
        (*d).normalGC,
        (if inverted != 0 {
            (*d).invertednormal
        } else {
            (*d).normal
        }) as std::ffi::c_ulong,
    );
    XSetBackground(
        (*d).dp,
        (*d).normalGC,
        (if inverted != 0 {
            (*d).invertedblack
        } else {
            (*d).black
        }) as std::ffi::c_ulong,
    );
    XSetFont((*d).dp, (*d).normalGC, (*d).font as Font);
    (*d).boldGC = XCreateGC(
        (*d).dp,
        (*d).wi,
        0 as std::ffi::c_long as std::ffi::c_ulong,
        0 as *mut XGCValues,
    );
    XSetForeground(
        (*d).dp,
        (*d).boldGC,
        (if inverted != 0 {
            (*d).invertedbold
        } else {
            (*d).bold
        }) as std::ffi::c_ulong,
    );
    XSetBackground(
        (*d).dp,
        (*d).boldGC,
        (if inverted != 0 {
            (*d).invertedblack
        } else {
            (*d).black
        }) as std::ffi::c_ulong,
    );
    XSetFont((*d).dp, (*d).boldGC, (*d).font as Font);
    (*d).dimGC = XCreateGC(
        (*d).dp,
        (*d).wi,
        0 as std::ffi::c_long as std::ffi::c_ulong,
        0 as *mut XGCValues,
    );
    XSetForeground(
        (*d).dp,
        (*d).dimGC,
        (if inverted != 0 {
            (*d).inverteddim
        } else {
            (*d).dim
        }) as std::ffi::c_ulong,
    );
    XSetBackground(
        (*d).dp,
        (*d).dimGC,
        (if inverted != 0 {
            (*d).invertedblack
        } else {
            (*d).black
        }) as std::ffi::c_ulong,
    );
    XSetFont((*d).dp, (*d).dimGC, (*d).font as Font);
    (*d).blackGC = XCreateGC(
        (*d).dp,
        (*d).wi,
        0 as std::ffi::c_long as std::ffi::c_ulong,
        0 as *mut XGCValues,
    );
    XSetForeground(
        (*d).dp,
        (*d).blackGC,
        (if inverted != 0 {
            (*d).invertedblack
        } else {
            (*d).black
        }) as std::ffi::c_ulong,
    );
    XSetBackground(
        (*d).dp,
        (*d).blackGC,
        (if inverted != 0 {
            (*d).invertedblack
        } else {
            (*d).black
        }) as std::ffi::c_ulong,
    );
    (*d).currGC = (*d).normalGC;
    if (*d).pixmapmode == 0 {
        XSetWindowBackground(
            (*d).dp,
            (*d).wi,
            (if inverted != 0 {
                (*d).invertedblack
            } else {
                (*d).black
            }) as std::ffi::c_ulong,
        );
    } else {
        XFillRectangle(
            (*d).dp,
            (*d).pi,
            (*d).blackGC,
            0 as std::ffi::c_int,
            0 as std::ffi::c_int,
            (*d).pixelwidth as std::ffi::c_uint,
            (*d).pixelheight as std::ffi::c_uint,
        );
    }
    XClearWindow((*d).dp, (*d).wi);
    if !((*d).previoust).is_null() {
        free((*d).previoust as *mut std::ffi::c_void);
        free((*d).previousa as *mut std::ffi::c_void);
    }
    (*d).previoust = 0 as *mut std::ffi::c_uchar;
    (*d).previousa = 0 as *mut std::ffi::c_uchar;
}
unsafe extern "C" fn X_init(
    mut p: *const aa_hardware_params,
    mut none: *const std::ffi::c_void,
    mut dest: *mut aa_hardware_params,
    mut driverdata: *mut *mut std::ffi::c_void,
) -> std::ffi::c_int {
    let mut font: *const std::ffi::c_char = b"fixed\0" as *const u8 as *const std::ffi::c_char;
    static mut registered: std::ffi::c_int = 0;
    static mut aafont: aa_font = aa_font {
        data: 0 as *const std::ffi::c_uchar,
        height: 0,
        name: 0 as *const std::ffi::c_char,
        shortname: 0 as *const std::ffi::c_char,
    };
    static mut def: aa_hardware_params = unsafe {
        {
            let mut init = aa_hardware_params {
                font: &aa_fontX13B as *const aa_font,
                supported: 2 as std::ffi::c_int
                    | 16 as std::ffi::c_int
                    | 1 as std::ffi::c_int
                    | 4 as std::ffi::c_int
                    | 8 as std::ffi::c_int
                    | (128 as std::ffi::c_int | 256 as std::ffi::c_int),
                minwidth: 0 as std::ffi::c_int,
                minheight: 0 as std::ffi::c_int,
                maxwidth: 0 as std::ffi::c_int,
                maxheight: 0 as std::ffi::c_int,
                recwidth: 80 as std::ffi::c_int,
                recheight: 32 as std::ffi::c_int,
                mmwidth: 0 as std::ffi::c_int,
                mmheight: 0 as std::ffi::c_int,
                width: 0,
                height: 0,
                dimmul: 0.,
                boldmul: 0.,
            };
            init
        }
    };
    let mut d: *mut xdriverdata = 0 as *mut xdriverdata;
    *dest = def;
    d = calloc(
        1 as std::ffi::c_int as std::ffi::c_ulong,
        ::core::mem::size_of::<xdriverdata>() as std::ffi::c_ulong,
    ) as *mut xdriverdata;
    *driverdata = d as *mut std::ffi::c_void;
    (*d).previoust = 0 as *mut std::ffi::c_uchar;
    (*d).previousa = 0 as *mut std::ffi::c_uchar;
    (*d).cvisible = 1 as std::ffi::c_int;
    (*d).width = 80 as std::ffi::c_int;
    (*d).height = 32 as std::ffi::c_int;
    (*d).dp = XOpenDisplay(0 as *const std::ffi::c_char);
    if ((*d).dp).is_null() {
        return 0 as std::ffi::c_int;
    }
    (*d).screen = (*((*d).dp as _XPrivDisplay)).default_screen;
    if !(getenv(b"AAFont\0" as *const u8 as *const std::ffi::c_char)).is_null() {
        font = getenv(b"AAFont\0" as *const u8 as *const std::ffi::c_char);
    }
    (*d).font = XLoadFont((*d).dp, font) as std::ffi::c_int;
    if (*d).font == 0 {
        XCloseDisplay((*d).dp);
        return 0 as std::ffi::c_int;
    }
    (*d).font_s = XQueryFont((*d).dp, (*d).font as XID);
    if ((*d).font_s).is_null() {
        XCloseDisplay((*d).dp);
        return 0 as std::ffi::c_int;
    }
    (*d).fontheight = (*(*d).font_s).max_bounds.ascent as std::ffi::c_int
        + (*(*d).font_s).max_bounds.descent as std::ffi::c_int;
    (*d).fontwidth = (*(*d).font_s).max_bounds.rbearing as std::ffi::c_int
        - (*(*d).font_s).min_bounds.lbearing as std::ffi::c_int;
    (*d).realfontwidth = (*(*d).font_s).max_bounds.width as std::ffi::c_int;
    (*d).cmap = (*((*((*d).dp as _XPrivDisplay)).screens).offset((*d).screen as isize)).cmap;
    X_AllocColors(d);
    if (*d).bold == (*d).normal {
        (*dest).supported &= !(4 as std::ffi::c_int);
    }
    if (*d).dim == (*d).normal {
        (*dest).supported &= !(2 as std::ffi::c_int);
    }
    (*d).attr.event_mask = (1 as std::ffi::c_long) << 15 as std::ffi::c_int;
    (*d).attr.override_redirect = 0 as std::ffi::c_int;
    if (*p).width != 0 {
        (*d).width = (*p).width;
    }
    if (*p).height != 0 {
        (*d).height = (*p).height;
    }
    if (*p).maxwidth != 0 && (*d).width > (*p).maxwidth {
        (*d).width = (*p).maxwidth;
    }
    if (*p).minwidth != 0 && (*d).width < (*p).minwidth {
        (*d).width = (*p).minwidth;
    }
    if (*p).maxheight != 0 && (*d).height > (*p).maxheight {
        (*d).height = (*p).maxheight;
    }
    if (*p).minheight != 0 && (*d).height < (*p).minheight {
        (*d).height = (*p).minheight;
    }
    (*d).wi = XCreateWindow(
        (*d).dp,
        (*((*((*d).dp as _XPrivDisplay)).screens).offset((*d).screen as isize)).root,
        0 as std::ffi::c_int,
        0 as std::ffi::c_int,
        ((*d).width * (*d).realfontwidth) as std::ffi::c_uint,
        ((*d).height * (*d).fontheight) as std::ffi::c_uint,
        0 as std::ffi::c_int as std::ffi::c_uint,
        (*((*((*d).dp as _XPrivDisplay)).screens).offset((*d).screen as isize)).root_depth,
        1 as std::ffi::c_int as std::ffi::c_uint,
        (*((*((*d).dp as _XPrivDisplay)).screens).offset((*d).screen as isize)).root_visual,
        ((1 as std::ffi::c_long) << 1 as std::ffi::c_int
            | (1 as std::ffi::c_long) << 3 as std::ffi::c_int
            | (1 as std::ffi::c_long) << 11 as std::ffi::c_int) as std::ffi::c_ulong,
        &mut (*d).attr,
    );
    if registered == 0 {
        (*d).pi = XCreatePixmap(
            (*d).dp,
            (*d).wi,
            (*d).fontwidth as std::ffi::c_uint,
            ((*d).fontheight * 256 as std::ffi::c_int) as std::ffi::c_uint,
            1 as std::ffi::c_int as std::ffi::c_uint,
        );
        if (*d).pi != 0 {
            let mut i: std::ffi::c_int = 0;
            let mut c: std::ffi::c_uchar = 0;
            let mut data: *mut std::ffi::c_uchar = 0 as *mut std::ffi::c_uchar;
            let mut image: *mut XImage = 0 as *mut XImage;
            registered = 1 as std::ffi::c_int;
            (*d).specialGC = XCreateGC(
                (*d).dp,
                (*d).pi,
                0 as std::ffi::c_long as std::ffi::c_ulong,
                0 as *mut XGCValues,
            );
            XSetForeground(
                (*d).dp,
                (*d).specialGC,
                0 as std::ffi::c_int as std::ffi::c_ulong,
            );
            XSetBackground(
                (*d).dp,
                (*d).specialGC,
                0 as std::ffi::c_int as std::ffi::c_ulong,
            );
            XFillRectangle(
                (*d).dp,
                (*d).pi,
                (*d).specialGC,
                0 as std::ffi::c_int,
                0 as std::ffi::c_int,
                (*d).fontwidth as std::ffi::c_uint,
                (256 as std::ffi::c_int * (*d).fontheight) as std::ffi::c_uint,
            );
            XSetForeground(
                (*d).dp,
                (*d).specialGC,
                1 as std::ffi::c_int as std::ffi::c_ulong,
            );
            XSetFont((*d).dp, (*d).specialGC, (*d).font as Font);
            i = 0 as std::ffi::c_int;
            while i < 256 as std::ffi::c_int {
                c = i as std::ffi::c_uchar;
                XDrawString(
                    (*d).dp,
                    (*d).pi,
                    (*d).specialGC,
                    0 as std::ffi::c_int,
                    (i + 1 as std::ffi::c_int) * (*d).fontheight - (*(*d).font_s).descent,
                    &mut c as *mut std::ffi::c_uchar as *mut std::ffi::c_char,
                    1 as std::ffi::c_int,
                );
                i += 1;
                i;
            }
            image = XGetImage(
                (*d).dp,
                (*d).pi,
                0 as std::ffi::c_int,
                0 as std::ffi::c_int,
                (*d).fontwidth as std::ffi::c_uint,
                (256 as std::ffi::c_int * (*d).fontheight) as std::ffi::c_uint,
                1 as std::ffi::c_int as std::ffi::c_ulong,
                1 as std::ffi::c_int,
            );
            if !image.is_null() {
                data = malloc((256 as std::ffi::c_int * (*d).fontheight) as std::ffi::c_ulong)
                    as *mut std::ffi::c_uchar;
                i = 0 as std::ffi::c_int;
                while i < 256 as std::ffi::c_int {
                    let mut y: std::ffi::c_int = 0;
                    font_error = 0 as std::ffi::c_int;
                    y = 0 as std::ffi::c_int;
                    while y < (*d).fontheight {
                        let mut o: std::ffi::c_int = 0;
                        o = (((mygetpixel(image, 0 as std::ffi::c_int, i * (*d).fontheight + y)
                            != 0 as std::ffi::c_int)
                            as std::ffi::c_int)
                            << 7 as std::ffi::c_int)
                            + (((mygetpixel(image, 1 as std::ffi::c_int, i * (*d).fontheight + y)
                                != 0 as std::ffi::c_int)
                                as std::ffi::c_int)
                                << 6 as std::ffi::c_int)
                            + (((mygetpixel(image, 2 as std::ffi::c_int, i * (*d).fontheight + y)
                                != 0 as std::ffi::c_int)
                                as std::ffi::c_int)
                                << 5 as std::ffi::c_int)
                            + (((mygetpixel(image, 3 as std::ffi::c_int, i * (*d).fontheight + y)
                                != 0 as std::ffi::c_int)
                                as std::ffi::c_int)
                                << 4 as std::ffi::c_int)
                            + (((mygetpixel(image, 4 as std::ffi::c_int, i * (*d).fontheight + y)
                                != 0 as std::ffi::c_int)
                                as std::ffi::c_int)
                                << 3 as std::ffi::c_int)
                            + (((mygetpixel(image, 5 as std::ffi::c_int, i * (*d).fontheight + y)
                                != 0 as std::ffi::c_int)
                                as std::ffi::c_int)
                                << 2 as std::ffi::c_int)
                            + (((mygetpixel(image, 6 as std::ffi::c_int, i * (*d).fontheight + y)
                                != 0 as std::ffi::c_int)
                                as std::ffi::c_int)
                                << 1 as std::ffi::c_int)
                            + (((mygetpixel(image, 7 as std::ffi::c_int, i * (*d).fontheight + y)
                                != 0 as std::ffi::c_int)
                                as std::ffi::c_int)
                                << 0 as std::ffi::c_int);
                        *data.offset((i * (*d).fontheight + y) as isize) = o as std::ffi::c_uchar;
                        y += 1;
                        y;
                    }
                    i += 1;
                    i;
                }
                aafont.name = b"Font used by X server\0" as *const u8 as *const std::ffi::c_char;
                aafont.shortname = b"current\0" as *const u8 as *const std::ffi::c_char;
                aafont.height = (*d).fontheight;
                aafont.data = data;
                aa_registerfont(&mut aafont);
                (*dest).font = &mut aafont;
            }
        }
    }
    XStoreName(
        (*d).dp,
        (*d).wi,
        b"aa for X\0" as *const u8 as *const std::ffi::c_char,
    );
    XMapWindow((*d).dp, (*d).wi);
    X_setinversionmode(
        (getenv(b"AAInverted\0" as *const u8 as *const std::ffi::c_char)
            != 0 as *mut std::ffi::c_void as *mut std::ffi::c_char) as std::ffi::c_int,
        d,
    );
    (*d).pixelwidth = -(1 as std::ffi::c_int);
    (*d).pixelheight = -(1 as std::ffi::c_int);
    XSync((*d).dp, 0 as std::ffi::c_int);
    aa_recommendlow(
        &mut aa_kbdrecommended,
        b"X11\0" as *const u8 as *const std::ffi::c_char,
    );
    return 1 as std::ffi::c_int;
}

pub unsafe extern "C" fn __aa_X_getsize(
    mut c: *mut aa_context,
    mut d: *mut xdriverdata,
) -> std::ffi::c_int {
    let mut px: std::ffi::c_uint = 0;
    let mut py: std::ffi::c_uint = 0;
    let mut tmp: std::ffi::c_int = 0;
    let mut wtmp: Window = 0;
    XSync((*d).dp, 0 as std::ffi::c_int);
    XGetGeometry(
        (*d).dp,
        (*d).wi,
        &mut wtmp,
        &mut tmp,
        &mut tmp,
        &mut px,
        &mut py,
        &mut tmp as *mut std::ffi::c_int as *mut std::ffi::c_uint,
        &mut tmp as *mut std::ffi::c_int as *mut std::ffi::c_uint,
    );
    tmp = 0 as std::ffi::c_int;
    if px != (*d).pixelwidth as std::ffi::c_uint || py != (*d).pixelheight as std::ffi::c_uint {
        tmp = 1 as std::ffi::c_int;
    }
    (*d).pixelwidth = px as std::ffi::c_int;
    (*d).pixelheight = py as std::ffi::c_int;
    if tmp != 0 {
        if (*d).pixmapmode != 0 {
            XFreePixmap((*d).dp, (*d).pi);
        }
        if (getenv(b"AABlink\0" as *const u8 as *const std::ffi::c_char)).is_null() {
            (*d).pi = XCreatePixmap(
                (*d).dp,
                (*d).wi,
                (*d).pixelwidth as std::ffi::c_uint,
                (*d).pixelheight as std::ffi::c_uint,
                (*((*((*d).dp as _XPrivDisplay)).screens).offset((*d).screen as isize)).root_depth
                    as std::ffi::c_uint,
            );
        } else {
            (*d).pi = 11 as std::ffi::c_int as Pixmap;
        }
        if (*d).pi == 11 as std::ffi::c_int as Pixmap {
            (*d).pixmapmode = 0 as std::ffi::c_int;
            XSetWindowBackground(
                (*d).dp,
                (*d).wi,
                (if (*d).inverted != 0 {
                    (*d).invertedblack
                } else {
                    (*d).black
                }) as std::ffi::c_ulong,
            );
        } else {
            (*d).pixmapmode = 1 as std::ffi::c_int;
            XFillRectangle(
                (*d).dp,
                (*d).pi,
                (*d).blackGC,
                0 as std::ffi::c_int,
                0 as std::ffi::c_int,
                (*d).pixelwidth as std::ffi::c_uint,
                (*d).pixelheight as std::ffi::c_uint,
            );
            XSetWindowBackgroundPixmap((*d).dp, (*d).wi, (*d).pi);
        }
        (*c).driverparams.mmwidth =
            (*((*((*d).dp as _XPrivDisplay)).screens).offset((*d).screen as isize)).mwidth
                * (*d).pixelwidth
                / (*((*((*d).dp as _XPrivDisplay)).screens).offset((*d).screen as isize)).width;
        (*c).driverparams.mmheight =
            (*((*((*d).dp as _XPrivDisplay)).screens).offset((*d).screen as isize)).mheight
                * (*d).pixelheight
                / (*((*((*d).dp as _XPrivDisplay)).screens).offset((*d).screen as isize)).height;
        if !((*d).previoust).is_null() {
            free((*d).previoust as *mut std::ffi::c_void);
            free((*d).previousa as *mut std::ffi::c_void);
        }
        (*d).previoust = 0 as *mut std::ffi::c_uchar;
        (*d).previousa = 0 as *mut std::ffi::c_uchar;
        X_flush(c);
        XFlush((*d).dp);
    }
    XSync((*d).dp, 0 as std::ffi::c_int);
    return tmp;
}
unsafe extern "C" fn X_uninit(mut c: *mut aa_context) {
    let mut d: *mut xdriverdata = (*c).driverdata as *mut xdriverdata;
    if !((*d).previoust).is_null() {
        free((*d).previoust as *mut std::ffi::c_void);
        free((*d).previousa as *mut std::ffi::c_void);
    }
    if (*d).pixmapmode != 0 {
        XFreePixmap((*d).dp, (*d).pi);
    }
    XCloseDisplay((*d).dp);
}
unsafe extern "C" fn X_getsize(
    mut c: *mut aa_context,
    mut width1: *mut std::ffi::c_int,
    mut height1: *mut std::ffi::c_int,
) {
    let mut d: *mut xdriverdata = (*c).driverdata as *mut xdriverdata;
    __aa_X_getsize(c, d);
    (*d).width = (*d).pixelwidth / (*d).realfontwidth;
    *width1 = (*d).width;
    (*d).height = (*d).pixelheight / (*d).fontheight;
    *height1 = (*d).height;
}
unsafe extern "C" fn X_setattr(mut d: *mut xdriverdata, mut attr: std::ffi::c_int) {
    match attr {
        0 | 4 => {
            (*d).currGC = (*d).normalGC;
        }
        1 => {
            (*d).currGC = (*d).dimGC;
        }
        2 => {
            (*d).currGC = (*d).boldGC;
        }
        3 => {
            (*d).currGC = (*d).blackGC;
        }
        _ => {}
    };
}
static mut _texty: *mut XTextItem = 0 as *const XTextItem as *mut XTextItem;
static mut nitem: *mut [std::ffi::c_int; 5] =
    0 as *const [std::ffi::c_int; 5] as *mut [std::ffi::c_int; 5];
static mut startitem: *mut [std::ffi::c_int; 5] =
    0 as *const [std::ffi::c_int; 5] as *mut [std::ffi::c_int; 5];
static mut _rectangles: *mut XRectangle = 0 as *const XRectangle as *mut XRectangle;
static mut nrectangles: [std::ffi::c_int; 4] = [0; 4];
static mut drawed: std::ffi::c_int = 0;
static mut area: std::ffi::c_int = 0;
unsafe extern "C" fn alloctables(mut d: *mut xdriverdata) {
    _texty = malloc(
        (::core::mem::size_of::<XTextItem>() as std::ffi::c_ulong)
            .wrapping_mul((*d).width as std::ffi::c_ulong)
            .wrapping_mul(5 as std::ffi::c_int as std::ffi::c_ulong)
            .wrapping_mul((*d).height as std::ffi::c_ulong),
    ) as *mut XTextItem;
    nitem = calloc(
        (::core::mem::size_of::<[std::ffi::c_int; 5]>() as std::ffi::c_ulong)
            .wrapping_mul((*d).height as std::ffi::c_ulong),
        1 as std::ffi::c_int as std::ffi::c_ulong,
    ) as *mut [std::ffi::c_int; 5];
    startitem = calloc(
        (::core::mem::size_of::<[std::ffi::c_int; 5]>() as std::ffi::c_ulong)
            .wrapping_mul((*d).height as std::ffi::c_ulong),
        1 as std::ffi::c_int as std::ffi::c_ulong,
    ) as *mut [std::ffi::c_int; 5];
    _rectangles = malloc(
        (::core::mem::size_of::<XRectangle>() as std::ffi::c_ulong)
            .wrapping_mul((*d).width as std::ffi::c_ulong)
            .wrapping_mul((*d).height as std::ffi::c_ulong)
            .wrapping_mul(4 as std::ffi::c_int as std::ffi::c_ulong),
    ) as *mut XRectangle;
}
unsafe extern "C" fn freetables() {
    free(_texty as *mut std::ffi::c_void);
    free(nitem as *mut std::ffi::c_void);
    free(startitem as *mut std::ffi::c_void);
    free(_rectangles as *mut std::ffi::c_void);
}
unsafe extern "C" fn MyDrawString(
    mut d: *mut xdriverdata,
    mut attr: std::ffi::c_int,
    mut x: std::ffi::c_int,
    mut y: std::ffi::c_int,
    mut c: *mut std::ffi::c_uchar,
    mut i: std::ffi::c_int,
) {
    let mut it: *mut XTextItem = 0 as *mut XTextItem;
    let mut rect: *mut XRectangle = 0 as *mut XRectangle;
    let mut n: std::ffi::c_int = 0;
    let mut a: std::ffi::c_int = 0;
    match attr {
        4 => {
            n = 1 as std::ffi::c_int;
        }
        5 => {
            n = 2 as std::ffi::c_int;
        }
        0 | 1 | 2 | 3 | _ => {
            n = 0 as std::ffi::c_int;
        }
    }
    match attr {
        1 => {
            a = 1 as std::ffi::c_int;
        }
        2 => {
            a = 2 as std::ffi::c_int;
        }
        4 => {
            a = 3 as std::ffi::c_int;
        }
        3 => {
            a = 4 as std::ffi::c_int;
        }
        5 | 0 | _ => {
            a = 0 as std::ffi::c_int;
        }
    }
    it = &mut *_texty.offset(
        (y as isize * 5 + a as isize) * (*d).width as isize
            + (*nitem.offset(y as isize))[a as usize] as isize,
    ) as *mut XTextItem;
    (*it).delta = x * (*d).realfontwidth - (*startitem.offset(y as isize))[a as usize];
    if (*it).delta == 0 && x != 0 {
        it = it.offset(-1);
        it;
        (*it).nchars += i;
    } else {
        let ref mut fresh0 = (*nitem.offset(y as isize))[a as usize];
        *fresh0 += 1;
        *fresh0;
        (*it).chars = c as *mut std::ffi::c_char;
        (*it).nchars = i;
        (*it).font = (*d).font as Font;
        drawed = 1 as std::ffi::c_int;
    }
    (*startitem.offset(y as isize))[a as usize] = (x + i) * (*d).realfontwidth;
    rect = &mut *_rectangles.offset(
        (n * (*d).height * (*d).width + *nrectangles.as_mut_ptr().offset(n as isize)) as isize,
    ) as *mut XRectangle;
    (*rect).x = (x * (*d).realfontwidth) as std::ffi::c_short;
    (*rect).y = (y * (*d).fontheight + 1 as std::ffi::c_int) as std::ffi::c_short;
    (*rect).width = (i * (*d).realfontwidth) as std::ffi::c_ushort;
    if nrectangles[n as usize] != 0
        && (*rect.offset(-(1 as std::ffi::c_int as isize))).y as std::ffi::c_int
            == (*rect).y as std::ffi::c_int
        && (*rect.offset(-(1 as std::ffi::c_int as isize))).x as std::ffi::c_int
            + (*rect.offset(-(1 as std::ffi::c_int as isize))).width as std::ffi::c_int
            == (*rect).x as std::ffi::c_int
    {
        nrectangles[n as usize] -= 1;
        nrectangles[n as usize];
        rect = rect.offset(-1);
        (*rect).width =
            ((*rect).width as std::ffi::c_int + i * (*d).realfontwidth) as std::ffi::c_ushort;
    }
    (*rect).height = (*d).fontheight as std::ffi::c_ushort;
    nrectangles[n as usize] += 1;
    nrectangles[n as usize];
    rect = &mut *_rectangles.offset(
        (n * (*d).height * (*d).width
            + *nrectangles
                .as_mut_ptr()
                .offset(3 as std::ffi::c_int as isize)) as isize,
    ) as *mut XRectangle;
    (*rect).x = (x * (*d).realfontwidth) as std::ffi::c_short;
    (*rect).y = (y * (*d).fontheight + 1 as std::ffi::c_int) as std::ffi::c_short;
    (*rect).width = (i * (*d).realfontwidth) as std::ffi::c_ushort;
    if nrectangles[3 as std::ffi::c_int as usize] != 0
        && (*rect.offset(-(1 as std::ffi::c_int as isize))).y as std::ffi::c_int
            == (*rect).y as std::ffi::c_int
        && (*rect.offset(-(1 as std::ffi::c_int as isize))).x as std::ffi::c_int
            + (*rect.offset(-(1 as std::ffi::c_int as isize))).width as std::ffi::c_int
            == (*rect).x as std::ffi::c_int
    {
        nrectangles[3 as std::ffi::c_int as usize] -= 1;
        nrectangles[3 as std::ffi::c_int as usize];
        rect = rect.offset(-1);
        (*rect).width =
            ((*rect).width as std::ffi::c_int + i * (*d).realfontwidth) as std::ffi::c_ushort;
    }
    (*rect).height = (*d).fontheight as std::ffi::c_ushort;
    nrectangles[3 as std::ffi::c_int as usize] += 1;
    nrectangles[3 as std::ffi::c_int as usize];
    area += i;
}
static mut Black: [std::ffi::c_int; 6] = [
    0 as std::ffi::c_int,
    0 as std::ffi::c_int,
    0 as std::ffi::c_int,
    0 as std::ffi::c_int,
    1 as std::ffi::c_int,
    1 as std::ffi::c_int,
];
unsafe extern "C" fn X_flush(mut c: *mut aa_context) {
    let mut d: *mut xdriverdata = (*c).driverdata as *mut xdriverdata;
    let mut x: std::ffi::c_int = 0;
    let mut y: std::ffi::c_int = 0;
    let mut attr: std::ffi::c_int = 0;
    let mut xs: std::ffi::c_int = 0 as std::ffi::c_int;
    let mut ys: std::ffi::c_int = 0 as std::ffi::c_int;
    let mut l: std::ffi::c_int = 0;
    let mut same: std::ffi::c_int = 0;
    let mut s: std::ffi::c_int = 0 as std::ffi::c_int;
    let mut pos: std::ffi::c_int = 0;
    attr = AA_NORMAL as std::ffi::c_int;
    alloctables(d);
    drawed = 0 as std::ffi::c_int;
    area = 0 as std::ffi::c_int;
    if (*c).imgwidth != (*d).imgwidth || (*c).imgheight != (*d).imgheight {
        if !((*d).previoust).is_null() {
            free((*d).previoust as *mut std::ffi::c_void);
            free((*d).previousa as *mut std::ffi::c_void);
        }
        (*d).previoust = 0 as *mut std::ffi::c_uchar;
        (*d).previousa = 0 as *mut std::ffi::c_uchar;
        (*d).imgwidth = (*c).imgwidth;
        (*d).imgheight = (*c).imgheight;
        if (*d).pixmapmode == 0 {
            XSetWindowBackground(
                (*d).dp,
                (*d).wi,
                (if (*d).inverted != 0 {
                    (*d).invertedblack
                } else {
                    (*d).black
                }) as std::ffi::c_ulong,
            );
        } else {
            XFillRectangle(
                (*d).dp,
                (*d).pi,
                (*d).blackGC,
                0 as std::ffi::c_int,
                0 as std::ffi::c_int,
                (*d).pixelwidth as std::ffi::c_uint,
                (*d).pixelheight as std::ffi::c_uint,
            );
            XSetWindowBackgroundPixmap((*d).dp, (*d).wi, (*d).pi);
            XClearWindow((*d).dp, (*d).wi);
        }
    }
    nrectangles[0 as std::ffi::c_int as usize] = 0 as std::ffi::c_int;
    nrectangles[1 as std::ffi::c_int as usize] = 0 as std::ffi::c_int;
    nrectangles[2 as std::ffi::c_int as usize] = 0 as std::ffi::c_int;
    nrectangles[3 as std::ffi::c_int as usize] = 0 as std::ffi::c_int;
    if ((*d).previoust).is_null() {
        (*d).previoust =
            malloc(((*d).width * (*d).height) as std::ffi::c_ulong) as *mut std::ffi::c_uchar;
        (*d).previousa = calloc(
            ((*d).width * (*d).height) as std::ffi::c_ulong,
            1 as std::ffi::c_int as std::ffi::c_ulong,
        ) as *mut std::ffi::c_uchar;
        memset(
            (*d).previoust as *mut std::ffi::c_void,
            ' ' as i32,
            ((*d).width * (*d).height) as std::ffi::c_ulong,
        );
    }
    y = 0 as std::ffi::c_int;
    while y < (*c).params.height {
        l = 0 as std::ffi::c_int;
        s = l;
        xs = 0 as std::ffi::c_int;
        ys = y;
        x = 0 as std::ffi::c_int;
        while x < (*c).params.width {
            pos = x + y * (*c).params.width;
            if s > 5 as std::ffi::c_int
                || *((*c).attrbuffer).offset(pos as isize) as std::ffi::c_int != attr
                    && (*((*c).textbuffer).offset(pos as isize) as std::ffi::c_int != ' ' as i32
                        || Black[*((*c).attrbuffer).offset(pos as isize) as usize] != 0
                        || Black[attr as usize] != 0)
            {
                if l - s != 0 {
                    MyDrawString(
                        d,
                        attr,
                        xs,
                        ys,
                        &mut *((*c).textbuffer).offset((xs + ys * (*c).params.width) as isize),
                        l - s,
                    );
                }
                attr = *((*c).attrbuffer).offset(pos as isize) as std::ffi::c_int;
                l = 0 as std::ffi::c_int;
                s = l;
                xs = x;
                ys = y;
            }
            if *((*d).previoust).offset(pos as isize) as std::ffi::c_int
                == *((*c).textbuffer).offset(pos as isize) as std::ffi::c_int
                && *((*d).previousa).offset(pos as isize) as std::ffi::c_int
                    == *((*c).attrbuffer).offset(pos as isize) as std::ffi::c_int
                || Black[attr as usize] == 0
                    && *((*d).previoust).offset(pos as isize) as std::ffi::c_int == ' ' as i32
                    && *((*c).textbuffer).offset(pos as isize) as std::ffi::c_int == ' ' as i32
                    && Black[*((*d).previousa).offset(pos as isize) as usize] == 0
            {
                same = 1 as std::ffi::c_int;
            } else {
                same = 0 as std::ffi::c_int;
            }
            if xs == x && same != 0 {
                xs += 1;
                xs;
            } else {
                if same != 0 {
                    s += 1;
                    s;
                } else {
                    s = 0 as std::ffi::c_int;
                }
                l += 1;
                l;
            }
            x += 1;
            x;
        }
        if l - s != 0 {
            MyDrawString(
                d,
                attr,
                xs,
                ys,
                &mut *((*c).textbuffer).offset((xs + ys * (*c).params.width) as isize),
                l - s,
            );
        }
        y += 1;
        y;
    }
    if drawed != 0 {
        memcpy(
            (*d).previousa as *mut std::ffi::c_void,
            (*c).attrbuffer as *const std::ffi::c_void,
            ((*d).width * (*d).height) as std::ffi::c_ulong,
        );
        memcpy(
            (*d).previoust as *mut std::ffi::c_void,
            (*c).textbuffer as *const std::ffi::c_void,
            ((*d).width * (*d).height) as std::ffi::c_ulong,
        );
        if nrectangles[0 as std::ffi::c_int as usize] != 0 {
            XFillRectangles(
                (*d).dp,
                if (*d).pixmapmode != 0 {
                    (*d).pi
                } else {
                    (*d).wi
                },
                (*d).blackGC,
                &mut *_rectangles.offset(
                    (0 as std::ffi::c_int * (*d).height * (*d).width + 0 as std::ffi::c_int)
                        as isize,
                ),
                nrectangles[0 as std::ffi::c_int as usize],
            );
        }
        if nrectangles[1 as std::ffi::c_int as usize] != 0 {
            XFillRectangles(
                (*d).dp,
                if (*d).pixmapmode != 0 {
                    (*d).pi
                } else {
                    (*d).wi
                },
                (*d).normalGC,
                &mut *_rectangles.offset(
                    (1 as std::ffi::c_int * (*d).height * (*d).width + 0 as std::ffi::c_int)
                        as isize,
                ),
                nrectangles[1 as std::ffi::c_int as usize],
            );
        }
        if nrectangles[2 as std::ffi::c_int as usize] != 0 {
            XFillRectangles(
                (*d).dp,
                if (*d).pixmapmode != 0 {
                    (*d).pi
                } else {
                    (*d).wi
                },
                (*d).specialGC,
                &mut *_rectangles.offset(
                    (2 as std::ffi::c_int * (*d).height * (*d).width + 0 as std::ffi::c_int)
                        as isize,
                ),
                nrectangles[2 as std::ffi::c_int as usize],
            );
        }
        if (*d).cvisible != 0 {
            XDrawLine(
                (*d).dp,
                if (*d).pixmapmode != 0 {
                    (*d).pi
                } else {
                    (*d).wi
                },
                (*d).normalGC,
                (*d).Xpos * (*d).realfontwidth,
                ((*d).Ypos + 1 as std::ffi::c_int) * (*d).fontheight - 1 as std::ffi::c_int,
                ((*d).Xpos + 1 as std::ffi::c_int) * (*d).realfontwidth - 1 as std::ffi::c_int,
                ((*d).Ypos + 1 as std::ffi::c_int) * (*d).fontheight - 1 as std::ffi::c_int,
            );
        }
        y = 0 as std::ffi::c_int;
        while y < (*d).height {
            x = 0 as std::ffi::c_int;
            while x < 5 as std::ffi::c_int {
                if (*nitem.offset(y as isize))[x as usize] != 0 {
                    X_setattr(d, x);
                    XDrawText(
                        (*d).dp,
                        if (*d).pixmapmode != 0 {
                            (*d).pi
                        } else {
                            (*d).wi
                        },
                        (*d).currGC,
                        0 as std::ffi::c_int,
                        (y + 1 as std::ffi::c_int) * (*d).fontheight - (*(*d).font_s).descent,
                        &mut *_texty.offset(
                            ((y * 5 as std::ffi::c_int + x) * (*d).width + 0 as std::ffi::c_int)
                                as isize,
                        ),
                        (*nitem.offset(y as isize))[x as usize],
                    );
                    if x == 4 as std::ffi::c_int {
                        XDrawText(
                            (*d).dp,
                            if (*d).pixmapmode != 0 {
                                (*d).pi
                            } else {
                                (*d).wi
                            },
                            (*d).currGC,
                            1 as std::ffi::c_int,
                            (y + 1 as std::ffi::c_int) * (*d).fontheight - (*(*d).font_s).descent,
                            &mut *_texty.offset(
                                ((y * 5 as std::ffi::c_int + x) * (*d).width + 0 as std::ffi::c_int)
                                    as isize,
                            ),
                            (*nitem.offset(y as isize))[x as usize],
                        );
                    }
                }
                x += 1;
                x;
            }
            y += 1;
            y;
        }
        if (*d).pixmapmode != 0 {
            if nrectangles[3 as std::ffi::c_int as usize] != 0
                && area < (*d).width * (*d).height / 2 as std::ffi::c_int
                && nrectangles[3 as std::ffi::c_int as usize] < 5 as std::ffi::c_int
            {
                let mut i: std::ffi::c_int = 0;
                i = 0 as std::ffi::c_int;
                while i < nrectangles[3 as std::ffi::c_int as usize] {
                    XClearArea(
                        (*d).dp,
                        (*d).wi,
                        (*_rectangles
                            .offset((3 as std::ffi::c_int * (*d).height * (*d).width + i) as isize))
                        .x as std::ffi::c_int,
                        (*_rectangles
                            .offset((3 as std::ffi::c_int * (*d).height * (*d).width + i) as isize))
                        .y as std::ffi::c_int,
                        (*_rectangles
                            .offset((3 as std::ffi::c_int * (*d).height * (*d).width + i) as isize))
                        .width as std::ffi::c_uint,
                        (*_rectangles
                            .offset((3 as std::ffi::c_int * (*d).height * (*d).width + i) as isize))
                        .height as std::ffi::c_uint,
                        0 as std::ffi::c_int,
                    );
                    i += 1;
                    i;
                }
            } else {
                XClearWindow((*d).dp, (*d).wi);
            }
        }
        XSync((*d).dp, 0 as std::ffi::c_int);
    }
    freetables();
}

pub unsafe extern "C" fn __aa_X_redraw(mut c: *mut aa_context) {
    let mut d: *mut xdriverdata = (*c).driverdata as *mut xdriverdata;
    if (*d).pixmapmode != 0 && !((*d).previoust).is_null() {
        XFlush((*d).dp);
        return;
    }
    if !((*d).previoust).is_null() {
        free((*d).previoust as *mut std::ffi::c_void);
        free((*d).previousa as *mut std::ffi::c_void);
    }
    (*d).previoust = 0 as *mut std::ffi::c_uchar;
    (*d).previousa = 0 as *mut std::ffi::c_uchar;
    X_flush(c);
    XFlush((*d).dp);
}
unsafe extern "C" fn X_gotoxy(
    mut c: *mut aa_context,
    mut x: std::ffi::c_int,
    mut y: std::ffi::c_int,
) {
    let mut d: *mut xdriverdata = (*c).driverdata as *mut xdriverdata;
    if (*d).Xpos != x || (*d).Ypos != y {
        if !((*d).previoust).is_null() {
            *((*d).previoust).offset(((*d).Ypos * (*d).width + (*d).Xpos) as isize) =
                255 as std::ffi::c_int as std::ffi::c_uchar;
        }
        (*d).Xpos = x;
        (*d).Ypos = y;
        X_flush(c);
    }
}
unsafe extern "C" fn X_cursor(mut c: *mut aa_context, mut mode: std::ffi::c_int) {
    let mut d: *mut xdriverdata = (*c).driverdata as *mut xdriverdata;
    (*d).cvisible = mode;
}
