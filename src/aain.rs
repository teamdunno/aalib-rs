use super::aastructs::*;

pub type aa_dithering_mode = std::ffi::c_uint;
pub const AA_DITHERTYPES: aa_dithering_mode = 3;
pub const AA_FLOYD_S: aa_dithering_mode = 2;
pub const AA_ERRORDISTRIB: aa_dithering_mode = 1;
pub const AA_NONE: aa_dithering_mode = 0;

pub unsafe extern "C" fn aa_initkbd(
    mut c: *mut aa_context,
    mut d: *const aa_kbddriver,
    mut mode: std::ffi::c_int,
) -> std::ffi::c_int {
    if ((*d).init).expect("non-null function pointer")(c, mode) != 0 {
        (*c).kbddriver = d;
        return 1 as std::ffi::c_int;
    }
    return 0 as std::ffi::c_int;
}

pub unsafe extern "C" fn aa_initmouse(
    mut c: *mut aa_context,
    mut d: *const aa_mousedriver,
    mut mode: std::ffi::c_int,
) -> std::ffi::c_int {
    if ((*d).init).expect("non-null function pointer")(c, mode) != 0 {
        (*c).mousedriver = d;
        (*c).mousemode = 1 as std::ffi::c_int;
        return 1 as std::ffi::c_int;
    }
    return 0 as std::ffi::c_int;
}

pub unsafe extern "C" fn aa_getmouse(
    mut c: *mut aa_context,
    mut x: *mut std::ffi::c_int,
    mut y: *mut std::ffi::c_int,
    mut z: *mut std::ffi::c_int,
) {
    *x = 0 as std::ffi::c_int;
    *y = 0 as std::ffi::c_int;
    *z = 0 as std::ffi::c_int;
    if !((*c).mousedriver).is_null() {
        ((*(*c).mousedriver).getmouse).expect("non-null function pointer")(c, x, y, z);
    }
}

pub unsafe extern "C" fn aa_getevent(
    mut c: *mut aa_context,
    mut wait: std::ffi::c_int,
) -> std::ffi::c_int {
    let mut x: std::ffi::c_int = 0;
    let mut y: std::ffi::c_int = 0;
    let mut b: std::ffi::c_int = 0;
    let mut ch: std::ffi::c_int = 0;
    if !((*c).mousedriver).is_null() {
        ((*(*c).mousedriver).getmouse).expect("non-null function pointer")(
            c, &mut x, &mut y, &mut b,
        );
        if x != (*c).mousex || y != (*c).mousey || b != (*c).buttons {
            (*c).mousex = x;
            (*c).mousey = y;
            (*c).buttons = b;
            return 259 as std::ffi::c_int;
        }
    }
    if ((*c).kbddriver).is_null() {
        return 400 as std::ffi::c_int;
    }
    if wait != 0 {
        loop {
            ch = ((*(*c).kbddriver).getkey).expect("non-null function pointer")(
                c,
                1 as std::ffi::c_int,
            );
            if !(ch == AA_NONE as std::ffi::c_int) {
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
                    return 259 as std::ffi::c_int;
                }
            }
        }
    } else {
        ch =
            ((*(*c).kbddriver).getkey).expect("non-null function pointer")(c, 0 as std::ffi::c_int);
    }
    if ch == 258 as std::ffi::c_int && ((*c).resizehandler).is_some() {
        ((*c).resizehandler).expect("non-null function pointer")(c);
    }
    if ch == 259 as std::ffi::c_int {
        if !((*c).mousedriver).is_null() {
            ((*(*c).mousedriver).getmouse).expect("non-null function pointer")(
                c, &mut x, &mut y, &mut b,
            );
            if x != (*c).mousex || y != (*c).mousey || b != (*c).buttons {
                (*c).mousex = x;
                (*c).mousey = y;
                (*c).buttons = b;
                return 259 as std::ffi::c_int;
            } else {
                return aa_getevent(c, wait);
            }
        } else {
            return 400 as std::ffi::c_int;
        }
    }
    return ch;
}

pub unsafe extern "C" fn aa_getkey(
    mut co: *mut aa_context,
    mut wait: std::ffi::c_int,
) -> std::ffi::c_int {
    let mut c: std::ffi::c_int = 0;
    loop {
        c = aa_getevent(co, wait);
        if !(c == 259 as std::ffi::c_int
            || c == 258 as std::ffi::c_int
            || c >= 65536 as std::ffi::c_int)
        {
            break;
        }
    }
    return c;
}
