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
fn aa_editdisplay(mut e: *mut aa_edit) {
    unsafe {
        let mut s: [std::ffi::c_char; 1000] = [0; 1000];
        let mut i: std::ffi::c_int = 0;
        if (*e).cursor > strlen((*e).data) as i64 {
            (*e).cursor = strlen((*e).data) as i64;
        }
        if (*e).cursor < (*e).printpos {
            (*e).printpos = (*e).cursor;
        }
        if (*e).cursor >= (*e).printpos + (*e).size {
            (*e).printpos = (*e).cursor - (*e).size;
        }
        if (*e).printpos < 0 {
            (*e).printpos = 0;
        }
        strncpy(
            s.as_mut_ptr(),
            ((*e).data).offset((*e).printpos as isize),
            (*e).size as std::ffi::c_ulong,
        );
        s[(*e).size as usize] = 0 as std::ffi::c_int as std::ffi::c_char;
        i = (strlen((*e).data)).wrapping_sub((*e).printpos as std::ffi::c_ulong) as std::ffi::c_int;
        while (i as i64) < (*e).size {
            s[i as usize] = ' ' as i32 as std::ffi::c_char;
            i += 1;
        }
        aa_puts(
            (*e).c,
            (*e).x as i64,
            (*e).y as i64,
            (if (*e).clearafterpress != 0 {
                AA_REVERSE as std::ffi::c_int
            } else {
                AA_SPECIAL as std::ffi::c_int
            }) as aa_attribute,
            s.as_mut_ptr(),
        );
        aa_gotoxy((*e).c, (*e).x + (*e).cursor - (*e).printpos, (*e).y);
    }
}

pub fn aa_createedit(
    mut c: *mut aa_context,
    mut x: &mut i32,
    mut y: &mut i32,
    mut size: &mut i32,
    mut s: *mut std::ffi::c_char,
    mut maxsize: &mut i32,
) -> *mut aa_edit {
    unsafe {
        let mut e: *mut aa_edit = 0 as *mut aa_edit;
        if *x < 0 {
            *x = 0;
        }
        if *y < 0 {
            *y = 0;
        }
        if *x >= ((*c).imgwidth - 1) as i32 {
            *x = ((*c).imgwidth - 2) as i32;
        }
        if *y >= ((*c).imgheight - 1) as i32 {
            *y = ((*c).imgwidth - 2) as i32;
        }
        if (*x + *size) as i64 >= (*c).imgwidth {
            *size = ((*c).imgwidth - 1 - (*x as i64)) as i32;
        }
        e = malloc(::core::mem::size_of::<aa_edit>() as std::ffi::c_ulong) as *mut aa_edit;
        if e.is_null() {
            return 0 as *mut aa_edit;
        }
        (*e).maxsize = (*maxsize as i64);
        (*e).data = s;
        (*e).cursor = strlen(s) as i64;
        (*e).clearafterpress = 1;
        (*e).x = *x as i64;
        (*e).y = *y as i64;
        (*e).size = *size as i64;
        (*e).c = c;
        (*e).printpos = 0;
        aa_editdisplay(e);
        return e;
    }
}
fn aa_insert(mut e: *mut aa_edit, mut ch: std::ffi::c_char) {
    unsafe {
        let mut i: i64 = 0;
        let mut s: i64 = strlen((*e).data) as i64;
        if s == (*e).maxsize - 1 {
            return;
        }
        i = s;
        while i >= (*e).cursor {
            *((*e).data).offset((i + 1) as isize) = *((*e).data).offset(i as isize);
            i -= 1;
            i;
        }
        *((*e).data).offset((s + 1) as isize) = 0 as std::ffi::c_int as std::ffi::c_char;
        *((*e).data).offset((*e).cursor as isize) = ch;
        (*e).cursor += 1;
        (*e).cursor;
    }
}
fn aa_delete(mut e: *mut aa_edit) {
    unsafe {
        let mut i = 0;
        let mut s = strlen((*e).data) as i64;
        if (*e).cursor == 0 {
            return;
        }
        (*e).cursor -= 1;
        (*e).cursor;
        i = (*e).cursor;
        while i <= s {
            *((*e).data).offset(i as isize) = *((*e).data).offset((i + 1) as isize);
            i += 1;
            i;
        }
    }
}

pub fn aa_editkey(mut e: *mut aa_edit, mut c: i64) {
    unsafe {
        if c < 127
            && (*(*__ctype_b_loc()).offset(c as isize) as u32 & _ISgraph as u32 != 0
                || c == ' ' as i64)
        {
            if (*e).clearafterpress != 0 {
                *((*e).data).offset(0 as isize) = 0 as std::ffi::c_char;
                (*e).cursor = 0;
            }
            (*e).clearafterpress = 0;
            aa_insert(e, c as std::ffi::c_char);
            aa_editdisplay(e);
        } else if c == 304 {
            (*e).clearafterpress = 0;
            aa_delete(e);
            aa_editdisplay(e);
        } else if c == 302 {
            (*e).cursor -= 1;
            (*e).cursor;
            (*e).clearafterpress = 0;
            if (*e).cursor < 0 {
                (*e).cursor = 0;
            }
            aa_editdisplay(e);
        } else if c == 303 {
            (*e).cursor += 1;
            (*e).cursor;
            (*e).clearafterpress = 0;
            if (*e).cursor > strlen((*e).data) as _ {
                (*e).cursor = strlen((*e).data) as _;
            }
            aa_editdisplay(e);
        }
        aa_flush((*e).c);
    }
}

pub fn aa_edit(
    mut c: *mut aa_context,
    mut x: i32,
    mut y: i32,
    mut size: i32,
    mut s: *mut std::ffi::c_char,
    mut maxsize: i32,
) {
    unsafe {
        let mut e: *mut aa_edit = 0 as *mut aa_edit;
        let mut ch = 0;
        aa_showcursor(c);
        e = aa_createedit(c, &mut x, &mut y, &mut size, s, &mut maxsize);
        aa_flush(c);
        loop {
            ch = aa_getkey(c, 1);
            if !(ch != 13 && ch != '\n' as i64) {
                break;
            }
            aa_editkey(e, ch);
        }
        aa_hidecursor(c);
        free(e as *mut std::ffi::c_void);
    }
}
