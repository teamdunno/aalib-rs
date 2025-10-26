use std::{
    io::{self, Stdin as StdStdin, Stdout as StdStdout, Write, stdout},
    sync::{LazyLock, Mutex},
    thread::{self, sleep},
    time::Duration,
};
use termion::{
    event::{Event, Key, MouseButton as TermionMouseButton, MouseEvent},
    input::TermRead,
    raw::{IntoRawMode, RawTerminal as TermionRawTerminal},
};

use crate::{
    aarec::{aa_mouserecommended, aa_recommendlow},
    aastructs::{aa_context, aa_kbddriver, aa_mousedriver},
};

static Stdin: LazyLock<StdStdin> = LazyLock::new(|| io::stdin());
static RawTerminal: LazyLock<TermionRawTerminal<StdStdout>> =
    LazyLock::new(|| stdout().into_raw_mode().unwrap());

static MouseX: LazyLock<Mutex<i64>> = LazyLock::new(|| Mutex::new(0));
static MouseY: LazyLock<Mutex<i64>> = LazyLock::new(|| Mutex::new(0));
static MouseButton: LazyLock<Mutex<Option<TermionMouseButton>>> =
    LazyLock::new(|| Mutex::new(None));

static KillThread: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));

unsafe fn termion_kbd_getchar(_: *mut aa_context, wait: i64) -> i64 {
    let mut stdout = RawTerminal.lock();

    if wait != 0 {
        sleep(Duration::from_secs(wait as u64));
    }

    stdout.flush();

    loop {
        let stdin = Stdin.lock();
        let mut stdout = RawTerminal.lock();

        for event in stdin.events() {
            let event = event.unwrap();

            match event {
                Event::Key(Key::Char(pressed_key)) => {
                    return (pressed_key as u64) as i64;
                }
                _ => {}
            }

            stdout.flush();
        }

        sleep(Duration::from_millis(250));
    }
}

unsafe fn termion_kbd_init(context: *mut aa_context, mode: i64) -> i64 {
    unsafe {
        aa_recommendlow(
            &mut aa_mouserecommended,
            b"termion_kbd\0" as *const u8 as *const std::ffi::c_char,
        );
        return 1;
    }
}

unsafe fn termion_kbd_uninit(c: *mut aa_context) {}

unsafe fn termion_mouse_uninit(c: *mut aa_context) {
    let mut kill_thread = KillThread.lock().unwrap();

    *kill_thread = true;
}

unsafe fn termion_mouse_init(context: *mut aa_context, mode: i64) -> i64 {
    thread::spawn(|| {
        loop {
            let stdin = Stdin.lock();
            let mut stdout = RawTerminal.lock();

            let kill_thread = KillThread.lock().unwrap();

            if *kill_thread {
                break;
            }

            for event in stdin.events() {
                let event = event.unwrap();

                match event {
                    Event::Mouse(mouse_event) => match mouse_event {
                        MouseEvent::Press(button, x, y) => {
                            let mut mouse_button = MouseButton.lock().unwrap();
                            let mut mouse_x = MouseX.lock().unwrap();
                            let mut mouse_y = MouseY.lock().unwrap();

                            *mouse_button = Some(button);
                            *mouse_x = x as _;
                            *mouse_y = y as _;
                        }

                        MouseEvent::Release(x, y) | MouseEvent::Hold(x, y) => {
                            let mut mouse_x = MouseX.lock().unwrap();
                            let mut mouse_y = MouseY.lock().unwrap();

                            *mouse_x = x as _;
                            *mouse_y = y as _;
                        }
                    },
                    _ => {}
                }

                stdout.flush();
            }

            sleep(Duration::from_millis(250));
        }
    });

    return 1;
}

// TODO: use termion's "MouseButton" type
unsafe fn termion_mouse(c: *mut aa_context, x: *mut i64, y: *mut i64, b: *mut i64) {
    unsafe {
        let mouse_x = MouseX.lock().unwrap();
        let mouse_y = MouseY.lock().unwrap();
        let mouse_button = MouseButton.lock().unwrap();

        *x = mouse_x.clone();
        *y = mouse_y.clone();
        if let Some(mouse_button) = mouse_button.clone() {
            *b = match mouse_button {
                TermionMouseButton::Left => 4,

                TermionMouseButton::Right => 1,

                TermionMouseButton::Middle => 2,

                TermionMouseButton::WheelUp => 8,

                _ => 0,
            }
        }
    }
}

pub static mut kbd_termion_d: aa_kbddriver = unsafe {
    aa_kbddriver {
        shortname: b"termion_kbd\0" as *const u8 as *const std::ffi::c_char,
        name: b"Termion keyboard driver 1.0\0" as *const u8 as *const std::ffi::c_char,
        flags: 0,
        init: Some(termion_kbd_init),
        uninit: Some(termion_kbd_uninit),
        getkey: Some(termion_kbd_getchar),
    }
};

pub static mut mouse_termion_d: aa_mousedriver = unsafe {
    aa_mousedriver {
        shortname: b"termion_mouse\0" as *const u8 as *const std::ffi::c_char,
        name: b"Termion mouse driver 1.0\0" as *const u8 as *const std::ffi::c_char,
        flags: 0,
        init: Some(termion_mouse_init),
        uninit: Some(termion_mouse_uninit),
        getmouse: Some(termion_mouse),
        cursormode: None,
    }
};
