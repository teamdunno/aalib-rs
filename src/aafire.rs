use super::aaflush::aa_flush;
use super::aain::aa_getevent;
use super::aakbdreg::aa_autoinitkbd;
use super::aalib::{aa_close, aa_resize};
use super::aaout::{aa_hidecursor, aa_resizehandler};
use super::aaparse::aa_parseoptions;
use super::aaregist::aa_autoinit;
use super::aarender::{aa_getrenderparams, aa_renderpalette};

use super::aastructs::*;

use std::ffi::c_int;

pub type parameters = c_int;
pub type aa_dithering_mode = i32;

unsafe extern "C" {
    fn printf(_: *const std::ffi::c_char, _: ...) -> std::ffi::c_int;
    static aa_help: *const std::ffi::c_char;
    static mut aa_defparams: aa_hardware_params;
    fn rand() -> std::ffi::c_int;
    fn exit(_: std::ffi::c_int) -> !;
}
pub const AA_DITHERTYPES: aa_dithering_mode = 3;
pub const AA_FLOYD_S: aa_dithering_mode = 2;
pub const AA_ERRORDISTRIB: aa_dithering_mode = 1;
pub const AA_NONE: aa_dithering_mode = 0;

pub type aa_palette = [std::ffi::c_int; 256];
static mut context: *mut aa_context = 0 as *const aa_context as *mut aa_context;
static mut params: *mut aa_renderparams = 0 as *mut aa_renderparams;
static mut palette: aa_palette = [0; 256];
static mut table: [std::ffi::c_uint; 1280] = [0; 1280];
static mut pal: [std::ffi::c_int; 768] = [
    0 as std::ffi::c_int,
    0 as std::ffi::c_int,
    0 as std::ffi::c_int,
    0 as std::ffi::c_int,
    0 as std::ffi::c_int,
    6 as std::ffi::c_int,
    0 as std::ffi::c_int,
    0 as std::ffi::c_int,
    6 as std::ffi::c_int,
    0 as std::ffi::c_int,
    0 as std::ffi::c_int,
    7 as std::ffi::c_int,
    0 as std::ffi::c_int,
    0 as std::ffi::c_int,
    8 as std::ffi::c_int,
    0 as std::ffi::c_int,
    0 as std::ffi::c_int,
    8 as std::ffi::c_int,
    0 as std::ffi::c_int,
    0 as std::ffi::c_int,
    9 as std::ffi::c_int,
    0 as std::ffi::c_int,
    0 as std::ffi::c_int,
    10 as std::ffi::c_int,
    2 as std::ffi::c_int,
    0 as std::ffi::c_int,
    10 as std::ffi::c_int,
    4 as std::ffi::c_int,
    0 as std::ffi::c_int,
    9 as std::ffi::c_int,
    6 as std::ffi::c_int,
    0 as std::ffi::c_int,
    9 as std::ffi::c_int,
    8 as std::ffi::c_int,
    0 as std::ffi::c_int,
    8 as std::ffi::c_int,
    10 as std::ffi::c_int,
    0 as std::ffi::c_int,
    7 as std::ffi::c_int,
    12 as std::ffi::c_int,
    0 as std::ffi::c_int,
    7 as std::ffi::c_int,
    14 as std::ffi::c_int,
    0 as std::ffi::c_int,
    6 as std::ffi::c_int,
    16 as std::ffi::c_int,
    0 as std::ffi::c_int,
    5 as std::ffi::c_int,
    18 as std::ffi::c_int,
    0 as std::ffi::c_int,
    5 as std::ffi::c_int,
    20 as std::ffi::c_int,
    0 as std::ffi::c_int,
    4 as std::ffi::c_int,
    22 as std::ffi::c_int,
    0 as std::ffi::c_int,
    4 as std::ffi::c_int,
    24 as std::ffi::c_int,
    0 as std::ffi::c_int,
    3 as std::ffi::c_int,
    26 as std::ffi::c_int,
    0 as std::ffi::c_int,
    2 as std::ffi::c_int,
    28 as std::ffi::c_int,
    0 as std::ffi::c_int,
    2 as std::ffi::c_int,
    30 as std::ffi::c_int,
    0 as std::ffi::c_int,
    1 as std::ffi::c_int,
    32 as std::ffi::c_int,
    0 as std::ffi::c_int,
    0 as std::ffi::c_int,
    32 as std::ffi::c_int,
    0 as std::ffi::c_int,
    0 as std::ffi::c_int,
    33 as std::ffi::c_int,
    0 as std::ffi::c_int,
    0 as std::ffi::c_int,
    34 as std::ffi::c_int,
    0 as std::ffi::c_int,
    0 as std::ffi::c_int,
    35 as std::ffi::c_int,
    0 as std::ffi::c_int,
    0 as std::ffi::c_int,
    36 as std::ffi::c_int,
    0 as std::ffi::c_int,
    0 as std::ffi::c_int,
    36 as std::ffi::c_int,
    0 as std::ffi::c_int,
    0 as std::ffi::c_int,
    37 as std::ffi::c_int,
    0 as std::ffi::c_int,
    0 as std::ffi::c_int,
    38 as std::ffi::c_int,
    0 as std::ffi::c_int,
    0 as std::ffi::c_int,
    39 as std::ffi::c_int,
    0 as std::ffi::c_int,
    0 as std::ffi::c_int,
    40 as std::ffi::c_int,
    0 as std::ffi::c_int,
    0 as std::ffi::c_int,
    40 as std::ffi::c_int,
    0 as std::ffi::c_int,
    0 as std::ffi::c_int,
    41 as std::ffi::c_int,
    0 as std::ffi::c_int,
    0 as std::ffi::c_int,
    42 as std::ffi::c_int,
    0 as std::ffi::c_int,
    0 as std::ffi::c_int,
    43 as std::ffi::c_int,
    0 as std::ffi::c_int,
    0 as std::ffi::c_int,
    44 as std::ffi::c_int,
    0 as std::ffi::c_int,
    0 as std::ffi::c_int,
    45 as std::ffi::c_int,
    0 as std::ffi::c_int,
    0 as std::ffi::c_int,
    46 as std::ffi::c_int,
    1 as std::ffi::c_int,
    0 as std::ffi::c_int,
    47 as std::ffi::c_int,
    1 as std::ffi::c_int,
    0 as std::ffi::c_int,
    48 as std::ffi::c_int,
    2 as std::ffi::c_int,
    0 as std::ffi::c_int,
    49 as std::ffi::c_int,
    2 as std::ffi::c_int,
    0 as std::ffi::c_int,
    50 as std::ffi::c_int,
    3 as std::ffi::c_int,
    0 as std::ffi::c_int,
    51 as std::ffi::c_int,
    3 as std::ffi::c_int,
    0 as std::ffi::c_int,
    52 as std::ffi::c_int,
    4 as std::ffi::c_int,
    0 as std::ffi::c_int,
    53 as std::ffi::c_int,
    4 as std::ffi::c_int,
    0 as std::ffi::c_int,
    54 as std::ffi::c_int,
    5 as std::ffi::c_int,
    0 as std::ffi::c_int,
    55 as std::ffi::c_int,
    5 as std::ffi::c_int,
    0 as std::ffi::c_int,
    56 as std::ffi::c_int,
    6 as std::ffi::c_int,
    0 as std::ffi::c_int,
    57 as std::ffi::c_int,
    6 as std::ffi::c_int,
    0 as std::ffi::c_int,
    58 as std::ffi::c_int,
    7 as std::ffi::c_int,
    0 as std::ffi::c_int,
    59 as std::ffi::c_int,
    7 as std::ffi::c_int,
    0 as std::ffi::c_int,
    60 as std::ffi::c_int,
    8 as std::ffi::c_int,
    0 as std::ffi::c_int,
    61 as std::ffi::c_int,
    8 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    9 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    9 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    10 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    10 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    11 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    11 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    12 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    12 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    13 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    13 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    14 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    14 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    15 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    15 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    16 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    16 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    17 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    17 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    18 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    18 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    19 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    19 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    20 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    20 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    21 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    21 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    22 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    22 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    23 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    24 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    24 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    25 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    25 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    26 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    26 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    27 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    27 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    28 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    28 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    29 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    29 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    30 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    30 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    31 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    31 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    32 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    32 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    33 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    33 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    34 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    34 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    35 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    35 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    36 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    36 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    37 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    38 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    38 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    39 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    39 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    40 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    40 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    41 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    41 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    42 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    42 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    43 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    43 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    44 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    44 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    45 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    45 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    46 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    46 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    47 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    47 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    48 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    48 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    49 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    49 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    50 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    50 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    51 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    52 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    52 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    52 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    52 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    52 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    53 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    53 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    53 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    53 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    54 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    54 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    54 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    54 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    54 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    55 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    55 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    55 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    55 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    56 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    56 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    56 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    56 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    57 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    57 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    57 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    57 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    57 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    58 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    58 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    58 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    58 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    59 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    59 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    59 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    59 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    60 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    60 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    60 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    60 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    60 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    61 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    61 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    61 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    61 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    62 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    62 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    62 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    62 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    63 as std::ffi::c_int,
    0 as std::ffi::c_int,
    63 as std::ffi::c_int,
    63 as std::ffi::c_int,
    1 as std::ffi::c_int,
    63 as std::ffi::c_int,
    63 as std::ffi::c_int,
    2 as std::ffi::c_int,
    63 as std::ffi::c_int,
    63 as std::ffi::c_int,
    3 as std::ffi::c_int,
    63 as std::ffi::c_int,
    63 as std::ffi::c_int,
    4 as std::ffi::c_int,
    63 as std::ffi::c_int,
    63 as std::ffi::c_int,
    5 as std::ffi::c_int,
    63 as std::ffi::c_int,
    63 as std::ffi::c_int,
    6 as std::ffi::c_int,
    63 as std::ffi::c_int,
    63 as std::ffi::c_int,
    7 as std::ffi::c_int,
    63 as std::ffi::c_int,
    63 as std::ffi::c_int,
    8 as std::ffi::c_int,
    63 as std::ffi::c_int,
    63 as std::ffi::c_int,
    9 as std::ffi::c_int,
    63 as std::ffi::c_int,
    63 as std::ffi::c_int,
    10 as std::ffi::c_int,
    63 as std::ffi::c_int,
    63 as std::ffi::c_int,
    10 as std::ffi::c_int,
    63 as std::ffi::c_int,
    63 as std::ffi::c_int,
    11 as std::ffi::c_int,
    63 as std::ffi::c_int,
    63 as std::ffi::c_int,
    12 as std::ffi::c_int,
    63 as std::ffi::c_int,
    63 as std::ffi::c_int,
    13 as std::ffi::c_int,
    63 as std::ffi::c_int,
    63 as std::ffi::c_int,
    14 as std::ffi::c_int,
    63 as std::ffi::c_int,
    63 as std::ffi::c_int,
    15 as std::ffi::c_int,
    63 as std::ffi::c_int,
    63 as std::ffi::c_int,
    16 as std::ffi::c_int,
    63 as std::ffi::c_int,
    63 as std::ffi::c_int,
    17 as std::ffi::c_int,
    63 as std::ffi::c_int,
    63 as std::ffi::c_int,
    18 as std::ffi::c_int,
    63 as std::ffi::c_int,
    63 as std::ffi::c_int,
    19 as std::ffi::c_int,
    63 as std::ffi::c_int,
    63 as std::ffi::c_int,
    20 as std::ffi::c_int,
    63 as std::ffi::c_int,
    63 as std::ffi::c_int,
    21 as std::ffi::c_int,
    63 as std::ffi::c_int,
    63 as std::ffi::c_int,
    21 as std::ffi::c_int,
    63 as std::ffi::c_int,
    63 as std::ffi::c_int,
    22 as std::ffi::c_int,
    63 as std::ffi::c_int,
    63 as std::ffi::c_int,
    23 as std::ffi::c_int,
    63 as std::ffi::c_int,
    63 as std::ffi::c_int,
    24 as std::ffi::c_int,
    63 as std::ffi::c_int,
    63 as std::ffi::c_int,
    25 as std::ffi::c_int,
    63 as std::ffi::c_int,
    63 as std::ffi::c_int,
    26 as std::ffi::c_int,
    63 as std::ffi::c_int,
    63 as std::ffi::c_int,
    27 as std::ffi::c_int,
    63 as std::ffi::c_int,
    63 as std::ffi::c_int,
    28 as std::ffi::c_int,
    63 as std::ffi::c_int,
    63 as std::ffi::c_int,
    29 as std::ffi::c_int,
    63 as std::ffi::c_int,
    63 as std::ffi::c_int,
    30 as std::ffi::c_int,
    63 as std::ffi::c_int,
    63 as std::ffi::c_int,
    31 as std::ffi::c_int,
    63 as std::ffi::c_int,
    63 as std::ffi::c_int,
    31 as std::ffi::c_int,
    63 as std::ffi::c_int,
    63 as std::ffi::c_int,
    32 as std::ffi::c_int,
    63 as std::ffi::c_int,
    63 as std::ffi::c_int,
    33 as std::ffi::c_int,
    63 as std::ffi::c_int,
    63 as std::ffi::c_int,
    34 as std::ffi::c_int,
    63 as std::ffi::c_int,
    63 as std::ffi::c_int,
    35 as std::ffi::c_int,
    63 as std::ffi::c_int,
    63 as std::ffi::c_int,
    36 as std::ffi::c_int,
    63 as std::ffi::c_int,
    63 as std::ffi::c_int,
    37 as std::ffi::c_int,
    63 as std::ffi::c_int,
    63 as std::ffi::c_int,
    38 as std::ffi::c_int,
    63 as std::ffi::c_int,
    63 as std::ffi::c_int,
    39 as std::ffi::c_int,
    63 as std::ffi::c_int,
    63 as std::ffi::c_int,
    40 as std::ffi::c_int,
    63 as std::ffi::c_int,
    63 as std::ffi::c_int,
    41 as std::ffi::c_int,
    63 as std::ffi::c_int,
    63 as std::ffi::c_int,
    42 as std::ffi::c_int,
    63 as std::ffi::c_int,
    63 as std::ffi::c_int,
    42 as std::ffi::c_int,
    63 as std::ffi::c_int,
    63 as std::ffi::c_int,
    43 as std::ffi::c_int,
    63 as std::ffi::c_int,
    63 as std::ffi::c_int,
    44 as std::ffi::c_int,
    63 as std::ffi::c_int,
    63 as std::ffi::c_int,
    45 as std::ffi::c_int,
    63 as std::ffi::c_int,
    63 as std::ffi::c_int,
    46 as std::ffi::c_int,
    63 as std::ffi::c_int,
    63 as std::ffi::c_int,
    47 as std::ffi::c_int,
    63 as std::ffi::c_int,
    63 as std::ffi::c_int,
    48 as std::ffi::c_int,
    63 as std::ffi::c_int,
    63 as std::ffi::c_int,
    49 as std::ffi::c_int,
    63 as std::ffi::c_int,
    63 as std::ffi::c_int,
    50 as std::ffi::c_int,
    63 as std::ffi::c_int,
    63 as std::ffi::c_int,
    51 as std::ffi::c_int,
    63 as std::ffi::c_int,
    63 as std::ffi::c_int,
    52 as std::ffi::c_int,
    63 as std::ffi::c_int,
    63 as std::ffi::c_int,
    52 as std::ffi::c_int,
    63 as std::ffi::c_int,
    63 as std::ffi::c_int,
    53 as std::ffi::c_int,
    63 as std::ffi::c_int,
    63 as std::ffi::c_int,
    54 as std::ffi::c_int,
    63 as std::ffi::c_int,
    63 as std::ffi::c_int,
    55 as std::ffi::c_int,
    63 as std::ffi::c_int,
    63 as std::ffi::c_int,
    56 as std::ffi::c_int,
    63 as std::ffi::c_int,
    63 as std::ffi::c_int,
    57 as std::ffi::c_int,
    63 as std::ffi::c_int,
    63 as std::ffi::c_int,
    58 as std::ffi::c_int,
    63 as std::ffi::c_int,
    63 as std::ffi::c_int,
    59 as std::ffi::c_int,
    63 as std::ffi::c_int,
    63 as std::ffi::c_int,
    60 as std::ffi::c_int,
    63 as std::ffi::c_int,
    63 as std::ffi::c_int,
    61 as std::ffi::c_int,
    63 as std::ffi::c_int,
    63 as std::ffi::c_int,
    62 as std::ffi::c_int,
    63 as std::ffi::c_int,
    63 as std::ffi::c_int,
    63 as std::ffi::c_int,
];
unsafe extern "C" fn initialize() {
    let mut i: std::ffi::c_int = 0;
    context = aa_autoinit(&mut aa_defparams);
    if context.is_null() {
        printf(b"Failed to initialize aalib\n\0" as *const u8 as *const std::ffi::c_char);
        exit(1 as std::ffi::c_int);
    }
    aa_autoinitkbd(context, 0 as std::ffi::c_int);
    params = aa_getrenderparams();
    i = 0 as std::ffi::c_int;
    while i < 256 as std::ffi::c_int {
        palette[i as usize] =
            pal[(i * 3 as std::ffi::c_int) as usize] * 4 as std::ffi::c_int * 30 as std::ffi::c_int
                + pal[(i * 3 as std::ffi::c_int + 1 as std::ffi::c_int) as usize]
                    * 4 as std::ffi::c_int
                    * 59 as std::ffi::c_int
                + pal[(i * 3 as std::ffi::c_int + 2 as std::ffi::c_int) as usize]
                    * 4 as std::ffi::c_int
                    * 11 as std::ffi::c_int
                >> 8 as std::ffi::c_int;
        i += 1;
        i;
    }
    aa_hidecursor(context);
}
unsafe extern "C" fn uninitialize() {
    aa_close(context);
    exit(0 as std::ffi::c_int);
}
unsafe extern "C" fn gentable() {
    let mut i: std::ffi::c_uint = 0;
    let mut p2: std::ffi::c_uint = 0;
    let mut minus: std::ffi::c_int =
        800 as std::ffi::c_int / ((*context).imgheight - 4 as std::ffi::c_int);
    if minus == 0 as std::ffi::c_int {
        minus = 1 as std::ffi::c_int;
    }
    i = 0 as std::ffi::c_int as std::ffi::c_uint;
    while i < (256 as std::ffi::c_int * 5 as std::ffi::c_int) as std::ffi::c_uint {
        if i > minus as std::ffi::c_uint {
            p2 = i
                .wrapping_sub(minus as std::ffi::c_uint)
                .wrapping_div(5 as std::ffi::c_int as std::ffi::c_uint);
            table[i as usize] = p2;
        } else {
            table[i as usize] = 0 as std::ffi::c_int as std::ffi::c_uint;
        }
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn firemain() {
    let mut i: std::ffi::c_uint = 0;
    let mut p: *mut std::ffi::c_uchar = 0 as *mut std::ffi::c_uchar;
    let mut bitmap: *mut std::ffi::c_char = (*context).imagebuffer as *mut std::ffi::c_char;
    i = 0 as std::ffi::c_int as std::ffi::c_uint;
    p = bitmap as *mut std::ffi::c_uchar;
    while p
        <= bitmap
            .offset(((*context).imgwidth * ((*context).imgheight - 4 as std::ffi::c_int)) as isize)
            as *mut std::ffi::c_uchar
    {
        *p = table[(*p
            .offset((*context).imgwidth as isize)
            .offset(-(1 as std::ffi::c_int as isize)) as std::ffi::c_int
            + *p.offset((*context).imgwidth as isize)
                .offset(1 as std::ffi::c_int as isize) as std::ffi::c_int
            + *p.offset((*context).imgwidth as isize) as std::ffi::c_int
            + (*p
                .offset((2 as std::ffi::c_int * (*context).imgwidth) as isize)
                .offset(-(1 as std::ffi::c_int as isize)) as std::ffi::c_int
                + *p.offset((2 as std::ffi::c_int * (*context).imgwidth) as isize)
                    .offset(1 as std::ffi::c_int as isize) as std::ffi::c_int))
            as usize] as std::ffi::c_uchar;
        p = p.offset(1 as std::ffi::c_int as isize);
    }
}
unsafe extern "C" fn drawfire() {
    let mut i: std::ffi::c_uint = 0;
    let mut last1: std::ffi::c_uint = 0;
    let mut i1: std::ffi::c_uint = 0;
    let mut i2: std::ffi::c_uint = 0;
    static mut loop_0: std::ffi::c_int = 0 as std::ffi::c_int;
    static mut sloop: std::ffi::c_int = 0 as std::ffi::c_int;
    static mut height: std::ffi::c_int = 0 as std::ffi::c_int;
    let mut p: *mut std::ffi::c_uchar = 0 as *mut std::ffi::c_uchar;
    let mut bitmap: *mut std::ffi::c_char = (*context).imagebuffer as *mut std::ffi::c_char;
    height += 1;
    height;
    loop_0 -= 1;
    loop_0;
    if loop_0 < 0 as std::ffi::c_int {
        loop_0 = rand() % 3 as std::ffi::c_int;
        sloop += 1;
        sloop;
    }
    i1 = 1 as std::ffi::c_int as std::ffi::c_uint;
    i2 = (4 as std::ffi::c_int * (*context).imgwidth + 1 as std::ffi::c_int) as std::ffi::c_uint;
    p = bitmap.offset(
        ((*context).imgwidth * ((*context).imgheight - 4 as std::ffi::c_int + 0 as std::ffi::c_int))
            as isize,
    ) as *mut std::ffi::c_uchar;
    while p
        < (bitmap as *mut std::ffi::c_uchar).offset(
            ((*context).imgwidth
                * ((*context).imgheight - 4 as std::ffi::c_int + 1 as std::ffi::c_int))
                as isize,
        )
    {
        last1 = (rand() as std::ffi::c_uint).wrapping_rem(
            if i1
                < (if i2 < height as std::ffi::c_uint {
                    i2
                } else {
                    height as std::ffi::c_uint
                })
            {
                i1
            } else {
                if i2 < height as std::ffi::c_uint {
                    i2
                } else {
                    height as std::ffi::c_uint
                }
            },
        );
        i = (rand() % 6 as std::ffi::c_int) as std::ffi::c_uint;
        while p
            < (bitmap as *mut std::ffi::c_uchar).offset(
                ((*context).imgwidth
                    * ((*context).imgheight - 4 as std::ffi::c_int + 1 as std::ffi::c_int))
                    as isize,
            )
            && i != 0 as std::ffi::c_int as std::ffi::c_uint
        {
            *p = last1 as std::ffi::c_uchar;
            last1 = last1.wrapping_add(
                (rand() % 6 as std::ffi::c_int - 2 as std::ffi::c_int) as std::ffi::c_uint,
            );
            *p.offset((*context).imgwidth as isize) = last1 as std::ffi::c_uchar;
            last1 = last1.wrapping_add(
                (rand() % 6 as std::ffi::c_int - 2 as std::ffi::c_int) as std::ffi::c_uint,
            );
            p = p.offset(1);
            p;
            i = i.wrapping_sub(1);
            i;
            i1 = i1.wrapping_add(4 as std::ffi::c_int as std::ffi::c_uint);
            i2 = i2.wrapping_sub(4 as std::ffi::c_int as std::ffi::c_uint);
        }
        *p.offset((2 as std::ffi::c_int * (*context).imgwidth) as isize) =
            last1 as std::ffi::c_uchar;
        last1 = last1.wrapping_add(
            (rand() % 6 as std::ffi::c_int - 2 as std::ffi::c_int) as std::ffi::c_uint,
        );
        p = p.offset(1);
        p;
        i1 = i1.wrapping_add(4 as std::ffi::c_int as std::ffi::c_uint);
        i2 = i2.wrapping_sub(4 as std::ffi::c_int as std::ffi::c_uint);
    }
    i = 0 as std::ffi::c_int as std::ffi::c_uint;
    firemain();
    aa_renderpalette(
        context,
        palette.as_mut_ptr() as *const std::ffi::c_int,
        params,
        0 as std::ffi::c_int,
        0 as std::ffi::c_int,
        (*context).params.width,
        (*context).params.height,
    );
    aa_flush(context);
}
unsafe extern "C" fn game() {
    let mut event: std::ffi::c_int = 0;
    gentable();
    loop {
        event = aa_getevent(context, 0 as std::ffi::c_int);
        if !(event == 0 || event == 258 as std::ffi::c_int) {
            break;
        }
        drawfire();
    }
}
unsafe fn main_0(
    mut argc: std::ffi::c_int,
    mut argv: *mut *mut std::ffi::c_char,
) -> std::ffi::c_int {
    if aa_parseoptions(
        0 as *mut aa_hardware_params,
        0 as *mut aa_renderparams,
        &mut argc,
        argv,
    ) == 0
        || argc != 1 as std::ffi::c_int
    {
        printf(b"%s\0" as *const u8 as *const std::ffi::c_char, aa_help);
        exit(1 as std::ffi::c_int);
    }
    initialize();
    aa_resizehandler(
        context,
        ::core::mem::transmute::<
            *mut std::ffi::c_void,
            Option<unsafe extern "C" fn(*mut aa_context) -> ()>,
        >(::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut aa_context) -> std::ffi::c_int>,
            *mut std::ffi::c_void,
        >(Some(
            aa_resize as unsafe extern "C" fn(*mut aa_context) -> std::ffi::c_int,
        ))),
    );
    game();
    uninitialize();
    return 1 as std::ffi::c_int;
}
pub fn main() {
    let mut args: Vec<*mut std::ffi::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0(
            (args.len() - 1) as std::ffi::c_int,
            args.as_mut_ptr() as *mut *mut std::ffi::c_char,
        ) as i32)
    }
}
