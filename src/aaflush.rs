use super::aastructs::*;

unsafe fn aa_display(c: *mut aa_context, mut x1: i64, mut y1: i64, mut x2: i64, mut y2: i64) { unsafe {
    let mut x = 0;
    let mut y = 0;
    let mut pos = 0;
    let mut attr = 0;
    let mut p = 0;
    let mut str: [std::ffi::c_uchar; 80] = [0; 80];
    let cursor = (*c).mousemode;
    let mut hidden = 0;
    if x2 < 0 || y2 < 0 || x1 > (*c).params.width || y1 > (*c).params.height {
        return;
    }
    if x2 >= (*c).params.width {
        x2 = (*c).params.width;
    }
    if y2 >= (*c).params.height {
        y2 = (*c).params.height;
    }
    if x1 < 0 {
        x1 = 0;
    }
    if y1 < 0 {
        y1 = 0;
    }
    if ((*(*c).driver).print).is_none() {
        return;
    }
    pos = 0;
    y = y1;
    while y < y2 {
        pos = y * (*c).params.width + x1;
        ((*(*c).driver).gotoxy).expect("non-null function pointer")(c, x1 as i64, y as i64);
        x = x1;
        while x < x2 {
            p = 0;
            attr = *((*c).attrbuffer).offset(pos as isize);
            while p < 79 && x < x2 && *((*c).attrbuffer).offset(pos as isize) == attr {
                str[p as usize] = *((*c).textbuffer).offset(pos as isize);
                pos += 1;
                p += 1;
                x += 1;
            }
            str[p as usize] = 0 as std::ffi::c_uchar;
            if hidden == 0
                && cursor != 0
                && !((*c).mousedriver).is_null()
                && (*(*c).mousedriver).flags & 8 != 0
            {
                aa_hidemouse(c);
                hidden = 1;
            }
            ((*(*c).driver).setattr).expect("non-null function pointer")(c, attr.into());
            ((*(*c).driver).print).expect("non-null function pointer")(
                c,
                str.as_mut_ptr() as *mut std::ffi::c_char,
            );
        }
        ((*(*c).driver).gotoxy).expect("non-null function pointer")(
            c,
            (*c).cursorx as i64,
            (*c).cursory as i64,
        );
        y += 1;
        y;
    }
    if hidden != 0 && cursor != 0 {
        aa_showmouse(c);
    }
}}

pub fn aa_hidemouse(c: *mut aa_context) {
    unsafe {
        if (*c).mousemode != 0 {
            (*c).mousemode = 0;
            if !((*c).mousedriver).is_null() && ((*(*c).mousedriver).cursormode).is_some() {
                ((*(*c).mousedriver).cursormode).expect("non-null function pointer")(c, 0);
            }
        }
    }
}

pub fn aa_showmouse(c: *mut aa_context) {
    unsafe {
        if (*c).mousemode == 0 {
            (*c).mousemode = 1;
            if !((*c).mousedriver).is_null() && ((*(*c).mousedriver).cursormode).is_some() {
                ((*(*c).mousedriver).cursormode).expect("non-null function pointer")(c, 1);
            }
        }
    }
}

pub fn aa_flush(c: *mut aa_context) {
    unsafe {
        if ((*(*c).driver).print).is_some() {
            aa_display(c, 0, 0, (*c).imgwidth, (*c).imgheight);
        }
        if ((*(*c).driver).flush).is_some() {
            let cursor = (*c).mousemode;
            if cursor != 0 && !((*c).mousedriver).is_null() && (*(*c).mousedriver).flags & 8 != 0 {
                aa_hidemouse(c);
            }
            ((*(*c).driver).flush).expect("non-null function pointer")(c);
            if cursor != 0 && !((*c).mousedriver).is_null() && (*(*c).mousedriver).flags & 8 != 0 {
                aa_showmouse(c);
            }
        }
    }
}
