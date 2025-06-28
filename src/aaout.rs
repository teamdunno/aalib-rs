use super::aain::aa_context;

pub type aa_attribute = std::ffi::c_uint;
pub const AA_SPECIAL: aa_attribute = 5;
pub const AA_REVERSE: aa_attribute = 4;
pub const AA_BOLDFONT: aa_attribute = 3;
pub const AA_BOLD: aa_attribute = 2;
pub const AA_DIM: aa_attribute = 1;
pub const AA_NORMAL: aa_attribute = 0;
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

pub unsafe extern "C" fn aa_puts(
    mut c: *mut aa_context,
    mut x: std::ffi::c_int,
    mut y: std::ffi::c_int,
    mut attr: aa_attribute,
    mut s: *const std::ffi::c_char,
) {
    let mut s1: [std::ffi::c_char; 10000] = [0; 10000];
    let mut pos: std::ffi::c_int = 0;
    let mut pos1: std::ffi::c_int = 0;
    let mut x1: std::ffi::c_int = 0;
    let mut y1: std::ffi::c_int = 0;
    if x < 0 as std::ffi::c_int
        || y < 0 as std::ffi::c_int
        || x >= (*c).params.width
        || y >= (*c).params.height
    {
        return;
    }
    x1 = x;
    y1 = y;
    pos = 0 as std::ffi::c_int;
    while *s.offset(pos as isize) as std::ffi::c_int != 0 as std::ffi::c_int
        && pos < 10000 as std::ffi::c_int
    {
        s1[pos as usize] = *s.offset(pos as isize);
        pos1 = x1 + y1 * (*c).params.width;
        *((*c).textbuffer).offset(pos1 as isize) = *s.offset(pos as isize) as std::ffi::c_uchar;
        *((*c).attrbuffer).offset(pos1 as isize) = attr as std::ffi::c_uchar;
        x1 += 1;
        x1;
        if x1 >= (*c).params.width {
            x1 = 0 as std::ffi::c_int;
            y1 += 1;
            y1;
            if y1 >= (*c).params.height {
                break;
            }
        }
        pos += 1;
        pos;
    }
}

pub unsafe extern "C" fn aa_resizehandler(
    mut c: *mut aa_context,
    mut handler: Option<unsafe extern "C" fn(*mut aa_context) -> ()>,
) {
    (*c).resizehandler = handler;
}

pub unsafe extern "C" fn aa_hidecursor(mut c: *mut aa_context) {
    (*c).cursorstate -= 1;
    (*c).cursorstate;
    if (*c).cursorstate == -(1 as std::ffi::c_int) && ((*(*c).driver).cursormode).is_some() {
        ((*(*c).driver).cursormode).expect("non-null function pointer")(c, 0 as std::ffi::c_int);
    }
}

pub unsafe extern "C" fn aa_showcursor(mut c: *mut aa_context) {
    (*c).cursorstate += 1;
    (*c).cursorstate;
    if (*c).cursorstate == 0 as std::ffi::c_int && ((*(*c).driver).cursormode).is_some() {
        ((*(*c).driver).cursormode).expect("non-null function pointer")(c, 1 as std::ffi::c_int);
    }
    aa_gotoxy(c, (*c).cursorx, (*c).cursory);
}

pub unsafe extern "C" fn aa_gotoxy(
    mut c: *mut aa_context,
    mut x: std::ffi::c_int,
    mut y: std::ffi::c_int,
) {
    if (*c).cursorstate >= 0 as std::ffi::c_int {
        if x < 0 as std::ffi::c_int {
            x = 0 as std::ffi::c_int;
        }
        if y < 0 as std::ffi::c_int {
            y = 0 as std::ffi::c_int;
        }
        if x >= (*c).params.width {
            x = (*c).params.width - 1 as std::ffi::c_int;
        }
        if y >= (*c).params.height {
            y = (*c).params.height - 1 as std::ffi::c_int;
        }
        ((*(*c).driver).gotoxy).expect("non-null function pointer")(c, x, y);
        (*c).cursorx = x;
        (*c).cursory = y;
    }
}
