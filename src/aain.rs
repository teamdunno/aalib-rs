use super::aastructs::*;

pub type aa_dithering_mode = std::ffi::c_uint;
pub const AA_DITHERTYPES: aa_dithering_mode = 3;
pub const AA_FLOYD_S: aa_dithering_mode = 2;
pub const AA_ERRORDISTRIB: aa_dithering_mode = 1;
pub const AA_NONE: aa_dithering_mode = 0;

pub fn aa_initkbd(c: *mut aa_context, d: *const aa_kbddriver, mode: i64) -> u32 {
    unsafe {
        if ((*d).init).expect("non-null function pointer")(c, mode) != 0 {
            (*c).kbddriver = d;
            return 1;
        }
        return 0;
    }
}

pub fn aa_initmouse(c: *mut aa_context, d: *const aa_mousedriver, mode: i64) -> i64 {
    unsafe {
        if ((*d).init).expect("non-null function pointer")(c, mode) != 0 {
            (*c).mousedriver = d;
            (*c).mousemode = 1;
            return 1;
        }
        return 0;
    }
}

pub fn aa_getmouse(c: *mut aa_context, x: *mut i64, y: *mut i64, z: *mut i64) {
    unsafe {
        *x = 0;
        *y = 0;
        *z = 0;
        if !((*c).mousedriver).is_null() {
            ((*(*c).mousedriver).getmouse).expect("non-null function pointer")(c, x, y, z);
        }
    }
}

pub fn aa_getevent(c: *mut aa_context, wait: i32) -> i64 {
    let mut x = 0;
    let mut y = 0;
    let mut b = 0;
    let mut ch = 0;
    unsafe {
        if !((*c).mousedriver).is_null() {
            ((*(*c).mousedriver).getmouse).expect("non-null function pointer")(
                c, &mut x, &mut y, &mut b,
            );
            if x != (*c).mousex || y != (*c).mousey || b != (*c).buttons {
                (*c).mousex = x;
                (*c).mousey = y;
                (*c).buttons = b;
                return 259;
            }
        }
        if ((*c).kbddriver).is_null() {
            return 400;
        }
        if wait != 0 {
            loop {
                ch = ((*(*c).kbddriver).getkey).expect("non-null function pointer")(c, 1);
                if !(ch == AA_NONE.into()) {
                    break;
                }
                if !((*c).mousedriver).is_null() {
                    ((*(*c).mousedriver).getmouse).expect("non-null function pointer")(
                        c, &mut x, &mut y, &mut b,
                    );
                    if x != (*c).mousex || y != (*c).mousey || b != (*c).buttons {
                        (*c).mousex = x;
                        (*c).mousey = y;
                        (*c).buttons = b;
                        return 259;
                    }
                }
            }
        } else {
            ch = ((*(*c).kbddriver).getkey).expect("non-null function pointer")(c, 0);
        }
        if ch == 258 && ((*c).resizehandler).is_some() {
            ((*c).resizehandler).expect("non-null function pointer")(c);
        }
        if ch == 259 {
            if !((*c).mousedriver).is_null() {
                ((*(*c).mousedriver).getmouse).expect("non-null function pointer")(
                    c, &mut x, &mut y, &mut b,
                );
                if x != (*c).mousex || y != (*c).mousey || b != (*c).buttons {
                    (*c).mousex = x;
                    (*c).mousey = y;
                    (*c).buttons = b;
                    return 259;
                } else {
                    return aa_getevent(c, wait);
                }
            } else {
                return 400;
            }
        }
        return ch;
    }
}

pub fn aa_getkey(co: *mut aa_context, wait: i32) -> i64 {
    unsafe {
        let mut c = 0;
        loop {
            c = aa_getevent(co, wait);
            if !(c == 259 || c == 258 || c >= 65536) {
                break;
            }
        }
        return c;
    }
}
