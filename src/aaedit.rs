use super::aastructs::*;

use super::aaflush::aa_flush;
use super::aain::aa_getkey;
use super::aaout::aa_puts;
use super::aaout::{aa_gotoxy, aa_hidecursor, aa_showcursor};
unsafe extern "C" {
    fn strncpy(
        _: *mut std::ffi::c_char,
        _: *const std::ffi::c_char,
        _: std::ffi::c_ulong,
    ) -> *mut std::ffi::c_char;
    fn strlen(_: *const std::ffi::c_char) -> std::ffi::c_ulong;
    fn __ctype_b_loc() -> *mut *const std::ffi::c_ushort;
    fn malloc(_: std::ffi::c_ulong) -> *mut std::ffi::c_void;
    fn free(_: *mut std::ffi::c_void);
}
pub type C2RustUnnamed = std::ffi::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type aa_attribute = std::ffi::c_uint;
pub const AA_SPECIAL: aa_attribute = 5;
pub const AA_REVERSE: aa_attribute = 4;
pub const AA_BOLDFONT: aa_attribute = 3;
pub const AA_BOLD: aa_attribute = 2;
pub const AA_DIM: aa_attribute = 1;
pub const AA_NORMAL: aa_attribute = 0;
unsafe extern "C" fn aa_editdisplay(mut e: *mut aa_edit) {
    let mut s: [std::ffi::c_char; 1000] = [0; 1000];
    let mut i: std::ffi::c_int = 0;
    if (*e).cursor > strlen((*e).data) as std::ffi::c_int {
        (*e).cursor = strlen((*e).data) as std::ffi::c_int;
    }
    if (*e).cursor < (*e).printpos {
        (*e).printpos = (*e).cursor;
    }
    if (*e).cursor >= (*e).printpos + (*e).size {
        (*e).printpos = (*e).cursor - (*e).size;
    }
    if (*e).printpos < 0 as std::ffi::c_int {
        (*e).printpos = 0 as std::ffi::c_int;
    }
    strncpy(
        s.as_mut_ptr(),
        ((*e).data).offset((*e).printpos as isize),
        (*e).size as std::ffi::c_ulong,
    );
    s[(*e).size as usize] = 0 as std::ffi::c_int as std::ffi::c_char;
    i = (strlen((*e).data)).wrapping_sub((*e).printpos as std::ffi::c_ulong) as std::ffi::c_int;
    while i < (*e).size {
        s[i as usize] = ' ' as i32 as std::ffi::c_char;
        i += 1;
        i;
    }
    aa_puts(
        (*e).c,
        (*e).x,
        (*e).y,
        (if (*e).clearafterpress != 0 {
            AA_REVERSE as std::ffi::c_int
        } else {
            AA_SPECIAL as std::ffi::c_int
        }) as aa_attribute,
        s.as_mut_ptr(),
    );
    aa_gotoxy((*e).c, (*e).x + (*e).cursor - (*e).printpos, (*e).y);
}

pub unsafe extern "C" fn aa_createedit(
    mut c: *mut aa_context,
    mut x: std::ffi::c_int,
    mut y: std::ffi::c_int,
    mut size: std::ffi::c_int,
    mut s: *mut std::ffi::c_char,
    mut maxsize: std::ffi::c_int,
) -> *mut aa_edit {
    let mut e: *mut aa_edit = 0 as *mut aa_edit;
    if x < 0 as std::ffi::c_int {
        x = 0 as std::ffi::c_int;
    }
    if y < 0 as std::ffi::c_int {
        y = 0 as std::ffi::c_int;
    }
    if x >= (*c).imgwidth - 1 as std::ffi::c_int {
        x = (*c).imgwidth - 2 as std::ffi::c_int;
    }
    if y >= (*c).imgheight - 1 as std::ffi::c_int {
        y = (*c).imgwidth - 2 as std::ffi::c_int;
    }
    if x + size >= (*c).imgwidth {
        size = (*c).imgwidth - 1 as std::ffi::c_int - x;
    }
    e = malloc(::core::mem::size_of::<aa_edit>() as std::ffi::c_ulong) as *mut aa_edit;
    if e.is_null() {
        return 0 as *mut aa_edit;
    }
    (*e).maxsize = maxsize;
    (*e).data = s;
    (*e).cursor = strlen(s) as std::ffi::c_int;
    (*e).clearafterpress = 1 as std::ffi::c_int;
    (*e).x = x;
    (*e).y = y;
    (*e).size = size;
    (*e).c = c;
    (*e).printpos = 0 as std::ffi::c_int;
    aa_editdisplay(e);
    return e;
}
unsafe extern "C" fn aa_insert(mut e: *mut aa_edit, mut ch: std::ffi::c_char) {
    let mut i: std::ffi::c_int = 0;
    let mut s: std::ffi::c_int = strlen((*e).data) as std::ffi::c_int;
    if s == (*e).maxsize - 1 as std::ffi::c_int {
        return;
    }
    i = s;
    while i >= (*e).cursor {
        *((*e).data).offset((i + 1 as std::ffi::c_int) as isize) = *((*e).data).offset(i as isize);
        i -= 1;
        i;
    }
    *((*e).data).offset((s + 1 as std::ffi::c_int) as isize) =
        0 as std::ffi::c_int as std::ffi::c_char;
    *((*e).data).offset((*e).cursor as isize) = ch;
    (*e).cursor += 1;
    (*e).cursor;
}
unsafe extern "C" fn aa_delete(mut e: *mut aa_edit) {
    let mut i: std::ffi::c_int = 0;
    let mut s: std::ffi::c_int = strlen((*e).data) as std::ffi::c_int;
    if (*e).cursor == 0 as std::ffi::c_int {
        return;
    }
    (*e).cursor -= 1;
    (*e).cursor;
    i = (*e).cursor;
    while i <= s {
        *((*e).data).offset(i as isize) = *((*e).data).offset((i + 1 as std::ffi::c_int) as isize);
        i += 1;
        i;
    }
}

pub unsafe extern "C" fn aa_editkey(mut e: *mut aa_edit, mut c: std::ffi::c_int) {
    if c < 127 as std::ffi::c_int
        && (*(*__ctype_b_loc()).offset(c as isize) as std::ffi::c_int
            & _ISgraph as std::ffi::c_int as std::ffi::c_ushort as std::ffi::c_int
            != 0
            || c == ' ' as i32)
    {
        if (*e).clearafterpress != 0 {
            *((*e).data).offset(0 as std::ffi::c_int as isize) =
                0 as std::ffi::c_int as std::ffi::c_char;
            (*e).cursor = 0 as std::ffi::c_int;
        }
        (*e).clearafterpress = 0 as std::ffi::c_int;
        aa_insert(e, c as std::ffi::c_char);
        aa_editdisplay(e);
    } else if c == 304 as std::ffi::c_int {
        (*e).clearafterpress = 0 as std::ffi::c_int;
        aa_delete(e);
        aa_editdisplay(e);
    } else if c == 302 as std::ffi::c_int {
        (*e).cursor -= 1;
        (*e).cursor;
        (*e).clearafterpress = 0 as std::ffi::c_int;
        if (*e).cursor < 0 as std::ffi::c_int {
            (*e).cursor = 0 as std::ffi::c_int;
        }
        aa_editdisplay(e);
    } else if c == 303 as std::ffi::c_int {
        (*e).cursor += 1;
        (*e).cursor;
        (*e).clearafterpress = 0 as std::ffi::c_int;
        if (*e).cursor > strlen((*e).data) as std::ffi::c_int {
            (*e).cursor = strlen((*e).data) as std::ffi::c_int;
        }
        aa_editdisplay(e);
    }
    aa_flush((*e).c);
}

pub unsafe extern "C" fn aa_edit(
    mut c: *mut aa_context,
    mut x: std::ffi::c_int,
    mut y: std::ffi::c_int,
    mut size: std::ffi::c_int,
    mut s: *mut std::ffi::c_char,
    mut maxsize: std::ffi::c_int,
) {
    let mut e: *mut aa_edit = 0 as *mut aa_edit;
    let mut ch: std::ffi::c_int = 0;
    aa_showcursor(c);
    e = aa_createedit(c, x, y, size, s, maxsize);
    aa_flush(c);
    loop {
        ch = aa_getkey(c, 1 as std::ffi::c_int);
        if !(ch != 13 as std::ffi::c_int && ch != '\n' as i32) {
            break;
        }
        aa_editkey(e, ch);
    }
    aa_hidecursor(c);
    free(e as *mut std::ffi::c_void);
}
