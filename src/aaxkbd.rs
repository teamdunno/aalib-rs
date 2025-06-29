use super::aaattributes::*;
use super::aarec::aa_mouserecommended;
use super::aarec::aa_recommendlow;
use super::aastructs::*;
use super::aax::{__aa_X_getsize, __aa_X_redraw, xdriverdata};
use super::aaxmouse::{__X_buttons, __X_mousex, __X_mousey};

use x11_dl::xlib::{_XDisplay, _XGC};

unsafe extern "C" {
    fn strlen(_: *const std::ffi::c_char) -> std::ffi::c_ulong;
    fn XLookupKeysym(_: *mut XKeyEvent, _: std::ffi::c_int) -> KeySym;
    fn XNextEvent(_: *mut Display, _: *mut XEvent) -> std::ffi::c_int;
    fn XPending(_: *mut Display) -> std::ffi::c_int;
    fn XSelectInput(_: *mut Display, _: Window, _: std::ffi::c_long) -> std::ffi::c_int;
    fn XSync(_: *mut Display, _: std::ffi::c_int) -> std::ffi::c_int;
    fn XLookupString(
        _: *mut XKeyEvent,
        _: *mut std::ffi::c_char,
        _: std::ffi::c_int,
        _: *mut KeySym,
        _: *mut XComposeStatus,
    ) -> std::ffi::c_int;
    static X11_d: aa_driver;
}
pub type XID = std::ffi::c_ulong;
pub type Atom = std::ffi::c_ulong;
pub type Time = std::ffi::c_ulong;
pub type Window = XID;
pub type Drawable = XID;
pub type Font = XID;
pub type Pixmap = XID;
pub type Cursor = XID;
pub type Colormap = XID;
pub type KeySym = XID;
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
pub struct XKeyEvent {
    pub type_0: std::ffi::c_int,
    pub serial: std::ffi::c_ulong,
    pub send_event: std::ffi::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: std::ffi::c_int,
    pub y: std::ffi::c_int,
    pub x_root: std::ffi::c_int,
    pub y_root: std::ffi::c_int,
    pub state: std::ffi::c_uint,
    pub keycode: std::ffi::c_uint,
    pub same_screen: std::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XButtonEvent {
    pub type_0: std::ffi::c_int,
    pub serial: std::ffi::c_ulong,
    pub send_event: std::ffi::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: std::ffi::c_int,
    pub y: std::ffi::c_int,
    pub x_root: std::ffi::c_int,
    pub y_root: std::ffi::c_int,
    pub state: std::ffi::c_uint,
    pub button: std::ffi::c_uint,
    pub same_screen: std::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XMotionEvent {
    pub type_0: std::ffi::c_int,
    pub serial: std::ffi::c_ulong,
    pub send_event: std::ffi::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: std::ffi::c_int,
    pub y: std::ffi::c_int,
    pub x_root: std::ffi::c_int,
    pub y_root: std::ffi::c_int,
    pub state: std::ffi::c_uint,
    pub is_hint: std::ffi::c_char,
    pub same_screen: std::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XCrossingEvent {
    pub type_0: std::ffi::c_int,
    pub serial: std::ffi::c_ulong,
    pub send_event: std::ffi::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: std::ffi::c_int,
    pub y: std::ffi::c_int,
    pub x_root: std::ffi::c_int,
    pub y_root: std::ffi::c_int,
    pub mode: std::ffi::c_int,
    pub detail: std::ffi::c_int,
    pub same_screen: std::ffi::c_int,
    pub focus: std::ffi::c_int,
    pub state: std::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XFocusChangeEvent {
    pub type_0: std::ffi::c_int,
    pub serial: std::ffi::c_ulong,
    pub send_event: std::ffi::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub mode: std::ffi::c_int,
    pub detail: std::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XKeymapEvent {
    pub type_0: std::ffi::c_int,
    pub serial: std::ffi::c_ulong,
    pub send_event: std::ffi::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub key_vector: [std::ffi::c_char; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XExposeEvent {
    pub type_0: std::ffi::c_int,
    pub serial: std::ffi::c_ulong,
    pub send_event: std::ffi::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub x: std::ffi::c_int,
    pub y: std::ffi::c_int,
    pub width: std::ffi::c_int,
    pub height: std::ffi::c_int,
    pub count: std::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XGraphicsExposeEvent {
    pub type_0: std::ffi::c_int,
    pub serial: std::ffi::c_ulong,
    pub send_event: std::ffi::c_int,
    pub display: *mut Display,
    pub drawable: Drawable,
    pub x: std::ffi::c_int,
    pub y: std::ffi::c_int,
    pub width: std::ffi::c_int,
    pub height: std::ffi::c_int,
    pub count: std::ffi::c_int,
    pub major_code: std::ffi::c_int,
    pub minor_code: std::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XNoExposeEvent {
    pub type_0: std::ffi::c_int,
    pub serial: std::ffi::c_ulong,
    pub send_event: std::ffi::c_int,
    pub display: *mut Display,
    pub drawable: Drawable,
    pub major_code: std::ffi::c_int,
    pub minor_code: std::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XVisibilityEvent {
    pub type_0: std::ffi::c_int,
    pub serial: std::ffi::c_ulong,
    pub send_event: std::ffi::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub state: std::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XCreateWindowEvent {
    pub type_0: std::ffi::c_int,
    pub serial: std::ffi::c_ulong,
    pub send_event: std::ffi::c_int,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
    pub x: std::ffi::c_int,
    pub y: std::ffi::c_int,
    pub width: std::ffi::c_int,
    pub height: std::ffi::c_int,
    pub border_width: std::ffi::c_int,
    pub override_redirect: std::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XDestroyWindowEvent {
    pub type_0: std::ffi::c_int,
    pub serial: std::ffi::c_ulong,
    pub send_event: std::ffi::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XUnmapEvent {
    pub type_0: std::ffi::c_int,
    pub serial: std::ffi::c_ulong,
    pub send_event: std::ffi::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub from_configure: std::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XMapEvent {
    pub type_0: std::ffi::c_int,
    pub serial: std::ffi::c_ulong,
    pub send_event: std::ffi::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub override_redirect: std::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XMapRequestEvent {
    pub type_0: std::ffi::c_int,
    pub serial: std::ffi::c_ulong,
    pub send_event: std::ffi::c_int,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XReparentEvent {
    pub type_0: std::ffi::c_int,
    pub serial: std::ffi::c_ulong,
    pub send_event: std::ffi::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub parent: Window,
    pub x: std::ffi::c_int,
    pub y: std::ffi::c_int,
    pub override_redirect: std::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XConfigureEvent {
    pub type_0: std::ffi::c_int,
    pub serial: std::ffi::c_ulong,
    pub send_event: std::ffi::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub x: std::ffi::c_int,
    pub y: std::ffi::c_int,
    pub width: std::ffi::c_int,
    pub height: std::ffi::c_int,
    pub border_width: std::ffi::c_int,
    pub above: Window,
    pub override_redirect: std::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XGravityEvent {
    pub type_0: std::ffi::c_int,
    pub serial: std::ffi::c_ulong,
    pub send_event: std::ffi::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub x: std::ffi::c_int,
    pub y: std::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XResizeRequestEvent {
    pub type_0: std::ffi::c_int,
    pub serial: std::ffi::c_ulong,
    pub send_event: std::ffi::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub width: std::ffi::c_int,
    pub height: std::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XConfigureRequestEvent {
    pub type_0: std::ffi::c_int,
    pub serial: std::ffi::c_ulong,
    pub send_event: std::ffi::c_int,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
    pub x: std::ffi::c_int,
    pub y: std::ffi::c_int,
    pub width: std::ffi::c_int,
    pub height: std::ffi::c_int,
    pub border_width: std::ffi::c_int,
    pub above: Window,
    pub detail: std::ffi::c_int,
    pub value_mask: std::ffi::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XCirculateEvent {
    pub type_0: std::ffi::c_int,
    pub serial: std::ffi::c_ulong,
    pub send_event: std::ffi::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub place: std::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XCirculateRequestEvent {
    pub type_0: std::ffi::c_int,
    pub serial: std::ffi::c_ulong,
    pub send_event: std::ffi::c_int,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
    pub place: std::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XPropertyEvent {
    pub type_0: std::ffi::c_int,
    pub serial: std::ffi::c_ulong,
    pub send_event: std::ffi::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub atom: Atom,
    pub time: Time,
    pub state: std::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XSelectionClearEvent {
    pub type_0: std::ffi::c_int,
    pub serial: std::ffi::c_ulong,
    pub send_event: std::ffi::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub selection: Atom,
    pub time: Time,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XSelectionRequestEvent {
    pub type_0: std::ffi::c_int,
    pub serial: std::ffi::c_ulong,
    pub send_event: std::ffi::c_int,
    pub display: *mut Display,
    pub owner: Window,
    pub requestor: Window,
    pub selection: Atom,
    pub target: Atom,
    pub property: Atom,
    pub time: Time,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XSelectionEvent {
    pub type_0: std::ffi::c_int,
    pub serial: std::ffi::c_ulong,
    pub send_event: std::ffi::c_int,
    pub display: *mut Display,
    pub requestor: Window,
    pub selection: Atom,
    pub target: Atom,
    pub property: Atom,
    pub time: Time,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XColormapEvent {
    pub type_0: std::ffi::c_int,
    pub serial: std::ffi::c_ulong,
    pub send_event: std::ffi::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub colormap: Colormap,
    pub new: std::ffi::c_int,
    pub state: std::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XClientMessageEvent {
    pub type_0: std::ffi::c_int,
    pub serial: std::ffi::c_ulong,
    pub send_event: std::ffi::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub message_type: Atom,
    pub format: std::ffi::c_int,
    pub data: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub b: [std::ffi::c_char; 20],
    pub s: [std::ffi::c_short; 10],
    pub l: [std::ffi::c_long; 5],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XMappingEvent {
    pub type_0: std::ffi::c_int,
    pub serial: std::ffi::c_ulong,
    pub send_event: std::ffi::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub request: std::ffi::c_int,
    pub first_keycode: std::ffi::c_int,
    pub count: std::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XErrorEvent {
    pub type_0: std::ffi::c_int,
    pub display: *mut Display,
    pub resourceid: XID,
    pub serial: std::ffi::c_ulong,
    pub error_code: std::ffi::c_uchar,
    pub request_code: std::ffi::c_uchar,
    pub minor_code: std::ffi::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XAnyEvent {
    pub type_0: std::ffi::c_int,
    pub serial: std::ffi::c_ulong,
    pub send_event: std::ffi::c_int,
    pub display: *mut Display,
    pub window: Window,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XGenericEvent {
    pub type_0: std::ffi::c_int,
    pub serial: std::ffi::c_ulong,
    pub send_event: std::ffi::c_int,
    pub display: *mut Display,
    pub extension: std::ffi::c_int,
    pub evtype: std::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XGenericEventCookie {
    pub type_0: std::ffi::c_int,
    pub serial: std::ffi::c_ulong,
    pub send_event: std::ffi::c_int,
    pub display: *mut Display,
    pub extension: std::ffi::c_int,
    pub evtype: std::ffi::c_int,
    pub cookie: std::ffi::c_uint,
    pub data: *mut std::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union _XEvent {
    pub type_0: std::ffi::c_int,
    pub xany: XAnyEvent,
    pub xkey: XKeyEvent,
    pub xbutton: XButtonEvent,
    pub xmotion: XMotionEvent,
    pub xcrossing: XCrossingEvent,
    pub xfocus: XFocusChangeEvent,
    pub xexpose: XExposeEvent,
    pub xgraphicsexpose: XGraphicsExposeEvent,
    pub xnoexpose: XNoExposeEvent,
    pub xvisibility: XVisibilityEvent,
    pub xcreatewindow: XCreateWindowEvent,
    pub xdestroywindow: XDestroyWindowEvent,
    pub xunmap: XUnmapEvent,
    pub xmap: XMapEvent,
    pub xmaprequest: XMapRequestEvent,
    pub xreparent: XReparentEvent,
    pub xconfigure: XConfigureEvent,
    pub xgravity: XGravityEvent,
    pub xresizerequest: XResizeRequestEvent,
    pub xconfigurerequest: XConfigureRequestEvent,
    pub xcirculate: XCirculateEvent,
    pub xcirculaterequest: XCirculateRequestEvent,
    pub xproperty: XPropertyEvent,
    pub xselectionclear: XSelectionClearEvent,
    pub xselectionrequest: XSelectionRequestEvent,
    pub xselection: XSelectionEvent,
    pub xcolormap: XColormapEvent,
    pub xclient: XClientMessageEvent,
    pub xmapping: XMappingEvent,
    pub xerror: XErrorEvent,
    pub xkeymap: XKeymapEvent,
    pub xgeneric: XGenericEvent,
    pub xcookie: XGenericEventCookie,
    pub pad: [std::ffi::c_long; 24],
}
pub type XEvent = _XEvent;
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
pub struct _XComposeStatus {
    pub compose_ptr: XPointer,
    pub chars_matched: std::ffi::c_int,
}
pub type XComposeStatus = _XComposeStatus;
unsafe fn X_init(c: *mut aa_context, mode: i64) -> i64 {
    unsafe {
        let d: *mut xdriverdata = (*c).driverdata as *mut xdriverdata;
        if (*c).driver != &X11_d as *const aa_driver {
            return 0;
        }
        (*d).attr.event_mask |= (1 as std::ffi::c_long) << 17 as std::ffi::c_int
            | (1 as std::ffi::c_long) << 0 as std::ffi::c_int
            | (if mode & 1 != 0 { 1 << 1 } else { 0 });
        XSelectInput((*d).dp, (*d).wi, (*d).attr.event_mask);
        aa_recommendlow(
            &mut aa_mouserecommended,
            b"X11\0" as *const u8 as *const std::ffi::c_char,
        );
        return 1;
    }
}
unsafe fn X_uninit(c: *mut aa_context) {
    unsafe {
        let d: *mut xdriverdata = (*c).driverdata as *mut xdriverdata;
        (*d).attr.event_mask &= !((1 as std::ffi::c_long) << 0 as std::ffi::c_int
            | (1 as std::ffi::c_long) << 1 as std::ffi::c_int);
        XSelectInput((*d).dp, (*d).wi, (*d).attr.event_mask);
    }
}
unsafe extern "C" fn decodekey(ev: *mut XEvent) -> std::ffi::c_int {
    unsafe {
        let mut ksym: KeySym = 0;
        let mut name: [std::ffi::c_char; 256] = [0; 256];
        ksym = XLookupKeysym(&mut (*ev).xkey, 0 as std::ffi::c_int);
        match ksym {
            65361 => return 302 as std::ffi::c_int,
            65363 => return 303 as std::ffi::c_int,
            65362 => return 300 as std::ffi::c_int,
            65364 => return 301 as std::ffi::c_int,
            65307 => return 305 as std::ffi::c_int,
            32 => return ' ' as i32,
            65293 => return 13 as std::ffi::c_int,
            65288 | 65535 => return 304 as std::ffi::c_int,
            _ => {}
        }
        name[XLookupString(
            &mut (*ev).xkey,
            name.as_mut_ptr(),
            256 as std::ffi::c_int,
            &mut ksym,
            0 as *mut XComposeStatus,
        ) as usize] = 0 as std::ffi::c_int as std::ffi::c_char;
        if strlen(name.as_mut_ptr()) != 1 as std::ffi::c_int as std::ffi::c_ulong {
            return (400 as std::ffi::c_int as KeySym).wrapping_add(ksym) as std::ffi::c_int;
        }
        return name[0 as std::ffi::c_int as usize] as std::ffi::c_int;
    }
}
unsafe fn X_getchar(c: *mut aa_context, wait: i64) -> i64 {
    unsafe {
        let d: *mut xdriverdata = (*c).driverdata as *mut xdriverdata;
        loop {
            let mut ev: XEvent = _XEvent { type_0: 0 };
            if wait == 0 && XPending((*d).dp) == 0 {
                return AA_NONE.try_into().unwrap();
            }
            XNextEvent((*d).dp, &mut ev);
            let current_block_15: u64;
            match ev.type_0 {
                4 => {
                    ev.xbutton.state |= ((1 as std::ffi::c_int)
                        << (ev.xbutton.button)
                            .wrapping_add(7 as std::ffi::c_int as std::ffi::c_uint))
                        as std::ffi::c_uint;
                    current_block_15 = 4023461411533754095;
                }
                5 => {
                    ev.xbutton.state &= !((1 as std::ffi::c_int)
                        << (ev.xbutton.button)
                            .wrapping_add(7 as std::ffi::c_int as std::ffi::c_uint))
                        as std::ffi::c_uint;
                    current_block_15 = 4023461411533754095;
                }
                6 => {
                    current_block_15 = 4023461411533754095;
                }
                12 => {
                    XSync((*d).dp, 0 as std::ffi::c_int);
                    __aa_X_redraw(c);
                    current_block_15 = 8831408221741692167;
                }
                22 => {
                    if __aa_X_getsize(c, d) != 0 {
                        return 258;
                    }
                    current_block_15 = 8831408221741692167;
                }
                2 => return decodekey(&mut ev).into(),
                3 => return decodekey(&mut ev) as i64 | 65536,
                _ => {
                    current_block_15 = 8831408221741692167;
                }
            }
            match current_block_15 {
                8831408221741692167 => {}
                _ => {
                    __X_mousex = ev.xbutton.x;
                    __X_mousey = ev.xbutton.y;
                    __X_buttons = ev.xbutton.state as std::ffi::c_int;
                    return 259;
                }
            }
        }
    }
}

#[unsafe(no_mangle)]
pub static mut kbd_X11_d: aa_kbddriver = unsafe {
    {
        let init = aa_kbddriver {
            shortname: b"X11\0" as *const u8 as *const std::ffi::c_char,
            name: b"X11 keyboard driver 1.0\0" as *const u8 as *const std::ffi::c_char,
            flags: 1,
            init: Some(X_init),
            uninit: Some(X_uninit),
            getkey: Some(X_getchar),
        };
        init
    }
};
