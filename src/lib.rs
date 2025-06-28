#![allow(warnings)]
#![allow(mutable_transmutes, static_mut_refs)]

use aastructs::aa_driver;

#[macro_use]
extern crate c2rust_bitfields;
extern crate libc;

pub mod aaattrs;
pub mod aacurkbd;
pub mod aacurmou;
pub mod aacurrfnt;
pub mod aacurses;
pub mod aaedit;
pub mod aafastre;
pub mod aafire;
pub mod aaflush;
pub mod aafont;
pub mod aafonts;
pub mod aagpm;
pub mod aahelp;
pub mod aaimage;
pub mod aaimgheight;
pub mod aaimgwidth;
pub mod aain;
pub mod aainfo;
pub mod aakbdreg;
pub mod aalib;
pub mod aalinux;
pub mod aalinuxkbd;
pub mod aamem;
pub mod aamktabl;
pub mod aammheight;
pub mod aammwidth;
pub mod aamoureg;
pub mod aaout;
pub mod aaparse;
pub mod aaprintf;
pub mod aaputpixel;
pub mod aarec;
pub mod aarecfunc;
pub mod aaregist;
pub mod aarender;
pub mod aasave;
pub mod aasavefont;
pub mod aascrheight;
pub mod aascrwidth;
pub mod aaslang;
pub mod aaslnkbd;
pub mod aastdin;
pub mod aastdout;
pub mod aastructs;
pub mod aatest;
pub mod aatext;
pub mod aavyhen;
pub mod aax;
pub mod aaxkbd;
pub mod aaxmouse;
pub mod font14;
pub mod font16;
pub mod font8;
pub mod font9;
pub mod fontcour;
pub mod fontgl;
pub mod fontline;
pub mod fontx13;
pub mod fontx13b;
pub mod fontx16;

#[cfg(test)]
mod tests {
    use crate::aalib::aa_close;

    use super::{
        aaflush::aa_flush,
        aaout::{aa_hidecursor, aa_puts},
        aaregist::aa_autoinit,
        aascrheight::aa_scrheight,
        aascrwidth::aa_scrwidth,
        aastructs::*,
        aax::{AA_BOLD, AA_DIM, AA_NORMAL, AA_SPECIAL},
    };
    use std::{ffi::CString, thread, time::Duration};

    #[test]
    fn flashing_text() {
        let mut params: aa_hardware_params = unsafe { std::mem::zeroed() };
        params.supported = (AA_NORMAL | AA_BOLD | AA_DIM) as i32;
        let context = unsafe { aa_autoinit(&params) };
        let text = CString::new("hello, world :)").unwrap();
        let mut iters = 0;
        while iters < 5 {
            unsafe {
                aa_puts(
                    context,
                    (aa_scrwidth(context) - text.to_string_lossy().to_string().len() as i32) / 2,
                    aa_scrheight(context) / 2,
                    AA_SPECIAL,
                    text.as_ptr(),
                );
                aa_flush(context);
            };
            thread::sleep(Duration::from_millis(500));
            unsafe {
                aa_puts(
                    context,
                    (aa_scrwidth(context) - text.to_string_lossy().to_string().len() as i32) / 2,
                    aa_scrheight(context) / 2,
                    AA_NORMAL,
                    text.as_ptr(),
                );
                aa_flush(context);
            }
            thread::sleep(Duration::from_millis(500));

            iters += 1;
        }

        unsafe { aa_close(context) };
    }
}
