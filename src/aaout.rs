use super::aastructs::*;

pub type aa_attribute = std::ffi::c_uint;
pub const AA_SPECIAL: aa_attribute = 5;
pub const AA_REVERSE: aa_attribute = 4;
pub const AA_BOLDFONT: aa_attribute = 3;
pub const AA_BOLD: aa_attribute = 2;
pub const AA_DIM: aa_attribute = 1;
pub const AA_NORMAL: aa_attribute = 0;

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
