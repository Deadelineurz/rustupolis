use crate::engine::keybinds::Tty;
use std::io::{Error, Write};
use termion::color::{Bg, Color, Fg, Reset};
use termion::cursor;

pub fn draw_text<C: Color, D: Color>(stdout: &Tty, text: &str, x: u16, y: u16, fg: C, bg: D) -> Result<(), Error> {
    write!(stdout.lock(), "{}{}{}{}{}{}", cursor::Goto(x, y), Fg(fg), Bg(bg), text, Fg(Reset), Bg(Reset))?;

    stdout.lock().flush()
}