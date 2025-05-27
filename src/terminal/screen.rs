use std::panic;
use std::process::exit;
use std::sync::atomic::{AtomicBool, Ordering};
use termion::color::{Bg, Color, Fg};
use termion::{clear, color, cursor};

static PANICKING: AtomicBool = AtomicBool::new(false);

pub struct CleanScreen {}

impl CleanScreen {
    pub fn new() -> Self {
        print!("{}{}", clear::All, cursor::Hide);
        ctrlc::set_handler(move || {
            println!("{}{}{}", clear::All, cursor::Goto(1, 1), cursor::Show);
            exit(0);
        }).expect("Arg");

        panic::set_hook(Box::new(|pan| {
            PANICKING.store(true, Ordering::SeqCst);
            print!("{}{}{}", Fg(color::LightRed), Bg(color::Reset), cursor::Show);
            println!("{}", pan);
            print!("{}", Fg(color::Reset))
        }));
        CleanScreen{}
    }
}

impl Drop for CleanScreen {
    fn drop(&mut self) {
        if !PANICKING.load(Ordering::SeqCst) {
            print!("{}{}{}", clear::All, cursor::Goto(1, 1), cursor::Show);
        }
    }
}

pub fn set_foreground<C: Color>(color: C) {
    print!("{}", Fg(color))
}

pub fn set_background<C: Color>(color: C) {
    print!("{}", Bg(color))
}

pub fn set_color<C: Color, D: Color>(foreground: C, background: D) {
    print!("{}{}", Fg(foreground), Bg(background))
}