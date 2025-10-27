use std::{
    ffi::c_char,
    io::{Write, stderr, stdout},
};

use super::aastructs::*;

unsafe fn stdout_init(
    p: *const aa_hardware_params,
    none: *const std::ffi::c_void,
    dest: *mut aa_hardware_params,
    n: *mut *mut std::ffi::c_void,
) -> i64 {
    unsafe {
        static mut def: aa_hardware_params = {
            let init = aa_hardware_params {
                font: 0 as *const aa_font,
                supported: 1 | (128 | 256),
                minwidth: 0,
                minheight: 0,
                maxwidth: 0,
                maxheight: 0,
                recwidth: 0,
                recheight: 0,
                mmwidth: 0,
                mmheight: 0,
                width: 0,
                height: 0,
                dimmul: 0.,
                boldmul: 0.,
            };
            init
        };
        *dest = def;
        return 1;
    }
}
unsafe fn stdout_uninit(_: *mut aa_context) {}
unsafe fn stdout_getsize(_: *mut aa_context, x: &mut i64, y: &mut i64) {
    unsafe fn stdout_getsize(_: *mut aa_context, x: &mut i64, y: &mut i64) {
        let (x_termion, y_termion) = termion::terminal_size().unwrap();

        *x = x_termion as i64;
        *y = y_termion as i64;
    }
}
unsafe fn stdout_flush(c: *mut aa_context) {
    unsafe {
        let mut x = 0;
        let mut y = 0;
        let mut stdout = stdout();
        while y < (*c).params.height {
            x = 0;
            while x < (*c).params.width {
                let character = *((*c).textbuffer).offset((x + y * (*c).params.width) as isize)
                    as std::ffi::c_int as u8 as char;
                write!(stdout, "{character}");
                x += 1;
                x;
            }
            write!(stdout, "\n");
            y += 1;
        }
        write!(stdout, "\n");

        stdout.flush();
    }
}
unsafe fn stdout_gotoxy(_: *mut aa_context, _: i64, _: i64) {}

#[unsafe(no_mangle)]
pub static mut stdout_d: aa_driver = unsafe {
    {
        let init = aa_driver {
            shortname: b"stdout\0" as *const u8 as *const std::ffi::c_char,
            name: b"Standard output driver\0" as *const u8 as *const std::ffi::c_char,
            init: Some(stdout_init),
            uninit: Some(stdout_uninit),
            getsize: Some(stdout_getsize),
            setattr: None,
            print: None,
            gotoxy: Some(stdout_gotoxy),
            flush: Some(stdout_flush),
            cursormode: None,
        };
        init
    }
};
unsafe fn stderr_flush(c: *mut aa_context) {
    unsafe {
        let mut x = 0;
        let mut y = 0;
        let mut stderr = stderr();
        while y < (*c).params.height {
            x = 0;
            while x < (*c).params.width {
                let character = *((*c).textbuffer).offset((x + y * (*c).params.width) as isize)
                    as std::ffi::c_int as u8 as char;
                write!(stderr, "{character}");
                x += 1;
                x;
            }
            write!(stderr, "\n");
            y += 1;
        }
        write!(stderr, "\n");

        stderr.flush();
    }
}

#[unsafe(no_mangle)]
pub static mut stderr_d: aa_driver = unsafe {
    {
        let init = aa_driver {
            shortname: b"stderr\0" as *const u8 as *const std::ffi::c_char,
            name: b"Standard error driver\0" as *const u8 as *const std::ffi::c_char,
            init: Some(stdout_init),
            uninit: Some(stdout_uninit),
            getsize: Some(stdout_getsize),
            setattr: None,
            print: None,
            gotoxy: Some(stdout_gotoxy),
            flush: Some(stderr_flush),
            cursormode: None,
        };
        init
    }
};
