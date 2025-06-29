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
pub mod aaflush;
pub mod aafont;
pub mod aafonts;
pub mod aagpm;
pub mod aahelp;
pub mod aaimage;
pub mod aaimgheight;
pub mod aaimgwidth;
pub mod aain;
pub mod aakbdreg;
pub mod aalib;
pub mod aalinux;
pub mod aalinuxkbd;
pub mod aamasks;
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
pub mod aascrheight;
pub mod aascrwidth;
pub mod aaslang;
pub mod aaslnkbd;
pub mod aastdin;
pub mod aastdout;
pub mod aastructs;
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
    use crate::{
        aaimgheight::aa_imgheight,
        aaimgwidth::aa_imgwidth,
        aain::aa_getevent,
        aakbdreg::aa_autoinitkbd,
        aalib::aa_close,
        aaout::aa_resizehandler,
        aaparse::aa_parseoptions,
        aaputpixel::aa_putpixel,
        aarender::{aa_defrenderparams, aa_getrenderparams, aa_render, aa_renderpalette},
        aax::{AA_BOLD, AA_DIM},
    };

    use super::{
        aaflush::aa_flush,
        aamasks::{AA_BOLD_MASK, AA_DIM_MASK, AA_NORMAL_MASK},
        aaout::{aa_hidecursor, aa_puts},
        aaregist::aa_autoinit,
        aascrheight::aa_scrheight,
        aascrwidth::aa_scrwidth,
        aastructs::*,
        aax::{AA_NORMAL, AA_SPECIAL},
    };
    use std::{ffi::CString, thread, time::Duration};

    #[test]
    fn tests() {
        // avoid tests running at the same time
        fill_pixels();
        flashing_text();
    }

    fn flashing_text() {
        let mut params: aa_hardware_params = unsafe { std::mem::zeroed() };
        params.supported = (AA_NORMAL_MASK | AA_BOLD_MASK | AA_DIM_MASK);
        let context = unsafe { aa_autoinit(&params) };
        let text = CString::new("hello, world :)").unwrap();
        let mut iters = 0;
        while iters < 5 {
            aa_puts(
                context,
                ((aa_scrwidth(context) - text.to_string_lossy().to_string().len() as i64) / 2)
                    .into(),
                (aa_scrheight(context) / 2).into(),
                AA_DIM,
                text.as_ptr(),
            );
            aa_flush(context);
            thread::sleep(Duration::from_millis(500));

            aa_puts(
                context,
                (aa_scrwidth(context) - text.to_string_lossy().to_string().len() as i64) / 2,
                aa_scrheight(context) / 2,
                AA_NORMAL,
                text.as_ptr(),
            );
            aa_flush(context);
            thread::sleep(Duration::from_millis(500));

            aa_puts(
                context,
                ((aa_scrwidth(context) - text.to_string_lossy().to_string().len() as i64) / 2)
                    .into(),
                (aa_scrheight(context) / 2).into(),
                AA_SPECIAL,
                text.as_ptr(),
            );
            aa_flush(context);
            thread::sleep(Duration::from_millis(500));

            aa_puts(
                context,
                ((aa_scrwidth(context) - text.to_string_lossy().to_string().len() as i64) / 2)
                    .into(),
                (aa_scrheight(context) / 2).into(),
                AA_BOLD,
                text.as_ptr(),
            );
            aa_flush(context);
            thread::sleep(Duration::from_millis(500));

            iters += 1;
        }

        aa_close(context);
    }

    fn fill_pixels() {
        let mut params: aa_hardware_params = unsafe { std::mem::zeroed() };
        params.supported = (AA_NORMAL_MASK | AA_BOLD_MASK | AA_DIM_MASK) as i64;
        let context = aa_autoinit(&params);

        let mut x = 0;
        let mut y = 0;
        let mut finished = false;
        while !finished {
            if x < aa_imgwidth(context) {
                aa_putpixel(context, x, y, 255);
                unsafe {
                    aa_render(
                        context,
                        &aa_defrenderparams,
                        0,
                        0,
                        aa_imgwidth(context),
                        aa_imgheight(context),
                    );
                }
                aa_flush(context);
                x += 1;
            } else {
                y += 1;
                x = 0;
            }

            if y == aa_imgheight(context) {
                finished = true;
            }
        }

        let mut x = 0;
        let mut y = 0;
        let mut finished = false;
        while !finished {
            if x < aa_imgwidth(context) {
                aa_putpixel(context, x, y, 0);
                unsafe {
                    aa_render(
                        context,
                        &aa_defrenderparams,
                        0,
                        0,
                        aa_imgwidth(context),
                        aa_imgheight(context),
                    );
                }
                aa_flush(context);
                x += 1;
            } else {
                y += 1;
                x = 0;
            }

            if y == aa_imgheight(context) {
                finished = true;
            }
        }

        let mut i = 5;
        while i != -1 {
            let text = CString::new(format!("closing in {i} seconds...")).unwrap();
            thread::sleep(Duration::from_secs(1));
            i -= 1;

            aa_puts(
                context,
                (aa_scrwidth(context) - text.to_string_lossy().to_string().len() as i64) / 2,
                aa_scrheight(context) / 2,
                AA_SPECIAL,
                text.as_ptr(),
            );
            aa_flush(context);
        }

        aa_close(context);
    }
}
