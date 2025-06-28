use super::aastructs::*;

unsafe extern "C" fn aa_display(
    mut c: *mut aa_context,
    mut x1: std::ffi::c_int,
    mut y1: std::ffi::c_int,
    mut x2: std::ffi::c_int,
    mut y2: std::ffi::c_int,
) {
    let mut x: std::ffi::c_int = 0;
    let mut y: std::ffi::c_int = 0;
    let mut pos: std::ffi::c_int = 0;
    let mut attr: std::ffi::c_int = 0;
    let mut p: std::ffi::c_int = 0;
    let mut str: [std::ffi::c_uchar; 80] = [0; 80];
    let mut cursor: std::ffi::c_int = (*c).mousemode;
    let mut hidden: std::ffi::c_int = 0 as std::ffi::c_int;
    if x2 < 0 as std::ffi::c_int
        || y2 < 0 as std::ffi::c_int
        || x1 > (*c).params.width
        || y1 > (*c).params.height
    {
        return;
    }
    if x2 >= (*c).params.width {
        x2 = (*c).params.width;
    }
    if y2 >= (*c).params.height {
        y2 = (*c).params.height;
    }
    if x1 < 0 as std::ffi::c_int {
        x1 = 0 as std::ffi::c_int;
    }
    if y1 < 0 as std::ffi::c_int {
        y1 = 0 as std::ffi::c_int;
    }
    if ((*(*c).driver).print).is_none() {
        return;
    }
    pos = 0 as std::ffi::c_int;
    y = y1;
    while y < y2 {
        pos = y * (*c).params.width + x1;
        ((*(*c).driver).gotoxy).expect("non-null function pointer")(c, x1, y);
        x = x1;
        while x < x2 {
            p = 0 as std::ffi::c_int;
            attr = *((*c).attrbuffer).offset(pos as isize) as std::ffi::c_int;
            while p < 79 as std::ffi::c_int
                && x < x2
                && *((*c).attrbuffer).offset(pos as isize) as std::ffi::c_int == attr
            {
                str[p as usize] = *((*c).textbuffer).offset(pos as isize);
                pos += 1;
                pos;
                p += 1;
                p;
                x += 1;
                x;
            }
            str[p as usize] = 0 as std::ffi::c_int as std::ffi::c_uchar;
            if hidden == 0
                && cursor != 0
                && !((*c).mousedriver).is_null()
                && (*(*c).mousedriver).flags & 8 as std::ffi::c_int != 0
            {
                aa_hidemouse(c);
                hidden = 1 as std::ffi::c_int;
            }
            ((*(*c).driver).setattr).expect("non-null function pointer")(c, attr);
            ((*(*c).driver).print).expect("non-null function pointer")(
                c,
                str.as_mut_ptr() as *mut std::ffi::c_char,
            );
        }
        ((*(*c).driver).gotoxy).expect("non-null function pointer")(c, (*c).cursorx, (*c).cursory);
        y += 1;
        y;
    }
    if hidden != 0 && cursor != 0 {
        aa_showmouse(c);
    }
}

pub unsafe extern "C" fn aa_hidemouse(mut c: *mut aa_context) {
    if (*c).mousemode != 0 {
        (*c).mousemode = 0 as std::ffi::c_int;
        if !((*c).mousedriver).is_null() && ((*(*c).mousedriver).cursormode).is_some() {
            ((*(*c).mousedriver).cursormode).expect("non-null function pointer")(
                c,
                0 as std::ffi::c_int,
            );
        }
    }
}

pub unsafe extern "C" fn aa_showmouse(mut c: *mut aa_context) {
    if (*c).mousemode == 0 {
        (*c).mousemode = 1 as std::ffi::c_int;
        if !((*c).mousedriver).is_null() && ((*(*c).mousedriver).cursormode).is_some() {
            ((*(*c).mousedriver).cursormode).expect("non-null function pointer")(
                c,
                1 as std::ffi::c_int,
            );
        }
    }
}

pub unsafe extern "C" fn aa_flush(mut c: *mut aa_context) {
    if ((*(*c).driver).print).is_some() {
        aa_display(
            c,
            0 as std::ffi::c_int,
            0 as std::ffi::c_int,
            (*c).imgwidth,
            (*c).imgheight,
        );
    }
    if ((*(*c).driver).flush).is_some() {
        let mut cursor: std::ffi::c_int = (*c).mousemode;
        if cursor != 0
            && !((*c).mousedriver).is_null()
            && (*(*c).mousedriver).flags & 8 as std::ffi::c_int != 0
        {
            aa_hidemouse(c);
        }
        ((*(*c).driver).flush).expect("non-null function pointer")(c);
        if cursor != 0
            && !((*c).mousedriver).is_null()
            && (*(*c).mousedriver).flags & 8 as std::ffi::c_int != 0
        {
            aa_showmouse(c);
        }
    }
}
