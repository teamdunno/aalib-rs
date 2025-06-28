unsafe extern "C" {
    fn strcmp(_: *const std::ffi::c_char, _: *const std::ffi::c_char) -> std::ffi::c_int;
    fn strdup(_: *const std::ffi::c_char) -> *mut std::ffi::c_char;
    fn malloc(_: std::ffi::c_ulong) -> *mut std::ffi::c_void;
    fn free(_: *mut std::ffi::c_void);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aa_linkedlist {
    pub text: *mut std::ffi::c_char,
    pub next: *mut aa_linkedlist,
    pub previous: *mut aa_linkedlist,
}

pub static mut aa_kbdrecommended: *mut aa_linkedlist =
    0 as *const aa_linkedlist as *mut aa_linkedlist;

pub static mut aa_mouserecommended: *mut aa_linkedlist =
    0 as *const aa_linkedlist as *mut aa_linkedlist;

pub static mut aa_displayrecommended: *mut aa_linkedlist =
    0 as *const aa_linkedlist as *mut aa_linkedlist;
unsafe extern "C" fn aa_find(
    mut l: *mut aa_linkedlist,
    mut text: *const std::ffi::c_char,
) -> *mut aa_linkedlist {
    let mut m: *mut aa_linkedlist = l;
    if l.is_null() {
        return 0 as *mut aa_linkedlist;
    }
    loop {
        if strcmp((*m).text, text) == 0 {
            return m;
        }
        m = (*m).next;
        if !(l != m) {
            break;
        }
    }
    return 0 as *mut aa_linkedlist;
}

pub unsafe extern "C" fn aa_recommendhi(
    mut l: *mut *mut aa_linkedlist,
    mut name: *const std::ffi::c_char,
) {
    let mut m: *mut aa_linkedlist =
        malloc(::core::mem::size_of::<aa_linkedlist>() as std::ffi::c_ulong) as *mut aa_linkedlist;
    let mut o: *mut aa_linkedlist = aa_find(*l, name);
    if !o.is_null() {
        (*(*o).next).previous = (*o).previous;
        (*(*o).previous).next = (*o).next;
        if *l == o {
            *l = if (**l).next == *l {
                0 as *mut aa_linkedlist
            } else {
                (**l).next
            };
        } else {
        };
    }
    (*m).text = strdup(name);
    if !(*l).is_null() {
        (*m).next = *l;
        (*m).previous = (**l).previous;
        (**l).previous = m;
        (*(*m).previous).next = m;
    } else {
        (*m).next = m;
        (*m).previous = m;
        *l = m;
    };
    *l = m;
}

pub unsafe extern "C" fn aa_recommendlow(
    mut l: *mut *mut aa_linkedlist,
    mut name: *const std::ffi::c_char,
) {
    let mut o: *mut aa_linkedlist = aa_find(*l, name);
    if o.is_null() {
        let mut m: *mut aa_linkedlist =
            malloc(::core::mem::size_of::<aa_linkedlist>() as std::ffi::c_ulong)
                as *mut aa_linkedlist;
        (*m).text = strdup(name);
        if !(*l).is_null() {
            (*m).next = *l;
            (*m).previous = (**l).previous;
            (**l).previous = m;
            (*(*m).previous).next = m;
        } else {
            (*m).next = m;
            (*m).previous = m;
            *l = m;
        };
    }
}

pub unsafe extern "C" fn aa_getfirst(mut l: *mut *mut aa_linkedlist) -> *mut std::ffi::c_char {
    let mut c: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
    let mut m: *mut aa_linkedlist = *l;
    if !(*l).is_null() {
        (*(*m).next).previous = (*m).previous;
        (*(*m).previous).next = (*m).next;
        if *l == m {
            *l = if (**l).next == *l {
                0 as *mut aa_linkedlist
            } else {
                (**l).next
            };
        } else {
        };
        c = (*m).text;
        free(m as *mut std::ffi::c_char as *mut std::ffi::c_void);
    }
    return c;
}
