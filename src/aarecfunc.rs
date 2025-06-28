use super::aarec::{aa_linkedlist, aa_recommendhi, aa_recommendlow};

unsafe extern "C" {
    static mut aa_kbdrecommended: *mut aa_linkedlist;
    static mut aa_mouserecommended: *mut aa_linkedlist;
    static mut aa_displayrecommended: *mut aa_linkedlist;
}

pub unsafe extern "C" fn aa_recommendhikbd(mut t: *const std::ffi::c_char) {
    aa_recommendhi(&mut aa_kbdrecommended, t);
}

pub unsafe extern "C" fn aa_recommendhimouse(mut t: *const std::ffi::c_char) {
    aa_recommendhi(&mut aa_mouserecommended, t);
}

pub unsafe extern "C" fn aa_recommendhidisplay(mut t: *const std::ffi::c_char) {
    aa_recommendhi(&mut aa_displayrecommended, t);
}

pub unsafe extern "C" fn aa_recommendlowkbd(mut t: *const std::ffi::c_char) {
    aa_recommendlow(&mut aa_kbdrecommended, t);
}

pub unsafe extern "C" fn aa_recommendlowmouse(mut t: *const std::ffi::c_char) {
    aa_recommendlow(&mut aa_mouserecommended, t);
}

pub unsafe extern "C" fn aa_recommendlowdisplay(mut t: *const std::ffi::c_char) {
    aa_recommendlow(&mut aa_displayrecommended, t);
}
