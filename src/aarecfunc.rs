use super::aarec::{aa_recommendhi, aa_recommendlow};
use super::aastructs::*;

unsafe extern "C" {
    static mut aa_kbdrecommended: *mut aa_linkedlist;
    static mut aa_mouserecommended: *mut aa_linkedlist;
    static mut aa_displayrecommended: *mut aa_linkedlist;
}

pub fn aa_recommendhikbd(t: *const std::ffi::c_char) {
    unsafe {
        aa_recommendhi(&mut aa_kbdrecommended, t);
    }
}

pub fn aa_recommendhimouse(t: *const std::ffi::c_char) {
    unsafe {
        aa_recommendhi(&mut aa_mouserecommended, t);
    }
}

pub fn aa_recommendhidisplay(t: *const std::ffi::c_char) {
    unsafe {
        aa_recommendhi(&mut aa_displayrecommended, t);
    }
}

pub fn aa_recommendlowkbd(t: *const std::ffi::c_char) {
    unsafe {
        aa_recommendlow(&mut aa_kbdrecommended, t);
    }
}

pub fn aa_recommendlowmouse(t: *const std::ffi::c_char) {
    unsafe {
        aa_recommendlow(&mut aa_mouserecommended, t);
    }
}

pub fn aa_recommendlowdisplay(t: *const std::ffi::c_char) {
    unsafe {
        aa_recommendlow(&mut aa_displayrecommended, t);
    }
}
