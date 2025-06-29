use super::aastructs::*;

pub type aa_attribute = std::ffi::c_uint;
pub const AA_SPECIAL: aa_attribute = 5;
pub const AA_REVERSE: aa_attribute = 4;
pub const AA_BOLDFONT: aa_attribute = 3;
pub const AA_BOLD: aa_attribute = 2;
pub const AA_DIM: aa_attribute = 1;
pub const AA_NORMAL: aa_attribute = 0;

pub fn aa_puts(
    c: *mut aa_context,
    x: i64,
    y: i64,
    attr: aa_attribute,
    s: *const std::ffi::c_char,
) {
    unsafe {
        let mut s1: [std::ffi::c_char; 10000] = [0; 10000];
        let mut pos = 0;
        let mut pos1 = 0;
        let mut x1 = 0;
        let mut y1 = 0;
        if x < 0 || y < 0 || x >= (*c).params.width || y >= (*c).params.height {
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
            if x1 >= (*c).params.width {
                x1 = 0;
                y1 += 1;
                if y1 >= (*c).params.height {
                    break;
                }
            }
            pos += 1;
            pos;
        }
    }
}

pub fn aa_resizehandler(
    c: *mut aa_context,
    handler: Option<unsafe fn(*mut aa_context) -> ()>,
) {
    unsafe {
        (*c).resizehandler = handler;
    }
}

pub fn aa_hidecursor(c: *mut aa_context) {
    unsafe {
        (*c).cursorstate -= 1;
        (*c).cursorstate;
        if (*c).cursorstate == -(1) && ((*(*c).driver).cursormode).is_some() {
            ((*(*c).driver).cursormode).expect("non-null function pointer")(c, 0);
        }
    }
}

pub fn aa_showcursor(c: *mut aa_context) {
    unsafe {
        (*c).cursorstate += 1;
        (*c).cursorstate;
        if (*c).cursorstate == 0 && ((*(*c).driver).cursormode).is_some() {
            ((*(*c).driver).cursormode).expect("non-null function pointer")(c, 1);
        }
        aa_gotoxy(c, (*c).cursorx, (*c).cursory);
    }
}

pub fn aa_gotoxy(c: *mut aa_context, mut x: i64, mut y: i64) {
    unsafe {
        if (*c).cursorstate >= 0 {
            if x < 0 {
                x = 0;
            }
            if y < 0 {
                y = 0;
            }
            if x >= (*c).params.width {
                x = (*c).params.width - 1;
            }
            if y >= (*c).params.height {
                y = (*c).params.height - 1;
            }
            ((*(*c).driver).gotoxy).expect("non-null function pointer")(c, x, y);
            (*c).cursorx = x;
            (*c).cursory = y;
        }
    }
}
