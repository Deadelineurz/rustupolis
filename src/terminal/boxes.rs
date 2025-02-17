use std::io::{Error, Stdout};
use termion::color::{Bg, Fg, Color, Reset};
use termion::cursor;
use std::io::Write;

struct CharacterSet {
    pub horizontal: char,
    pub vertical: char,
    pub top_left: char,
    pub top_right: char,
    pub bottom_left: char,
    pub bottom_right: char
}

#[derive(Debug, Copy, Clone)]
pub enum BoxFill<C: Color> {
    None,
    Fill(char),
    Color(C)
}

impl<C: Color> BoxFill<C> {
    pub fn color(color: C) -> BoxFill<C> {
        BoxFill::Color(color)
    }
}

impl BoxFill<Reset> {
    pub fn none() -> BoxFill<Reset> {
        BoxFill::None
    }

    pub fn blank() -> BoxFill<Reset> {
        BoxFill::Color(Reset)
    }

    pub fn fill(c: char) -> BoxFill<Reset> {
        BoxFill::Fill(c)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum BoxFormat {
    Solid,
    Dashed,
    Double
}

impl BoxFormat {
    fn characters(&self) -> &'static CharacterSet {
        match self {
            BoxFormat::Solid => &CharacterSet {
                horizontal: '─',
                vertical: '│',
                top_left: '┌',
                top_right: '┐',
                bottom_left: '└',
                bottom_right: '┘',
            },
            BoxFormat::Dashed => &CharacterSet {
                horizontal: '┄',
                vertical: '┆',
                top_left: '┌',
                top_right: '┐',
                bottom_left: '└',
                bottom_right: '┘',
            },
            BoxFormat::Double => &CharacterSet{
                horizontal: '═',
                vertical: '║',
                top_left: '╔',
                top_right: '╗',
                bottom_left: '╚',
                bottom_right: '╝',
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct BoxStyle<C: Color + Copy, D: Color + Copy> {
    pub fill: BoxFill<C>,
    pub lines_color: D,
    pub format: BoxFormat
}

impl<C: Color + Copy, D: Color + Copy> BoxStyle<C, D> {
    pub fn fill<E: Color + Copy>(&self, fill: BoxFill<E>) -> BoxStyle<E, D> {
        BoxStyle{
            fill,
            lines_color: self.lines_color,
            format: self.format
        }
    }

    pub fn lines_color<E: Color + Copy>(&self, color: E) -> BoxStyle<C, E> {
        BoxStyle{
            fill: self.fill,
            lines_color: color,
            format: self.format
        }
    }

    pub fn format(&self, format: BoxFormat) -> BoxStyle<C, D> {
        BoxStyle{ format, .. *self }
    }
}

impl BoxStyle<Reset, Reset> {
    pub fn new() -> BoxStyle<Reset, Reset> {
        BoxStyle{
            fill: BoxFill::None,
            lines_color: Reset,
            format: BoxFormat::Solid
        }
    }
}

pub fn draw_box<C: Color + Copy, D: Color + Copy>(stdout: &mut Stdout, x: u16, y: u16, width: u16, height: u16, style: BoxStyle<C, D>) -> Result<(), Error> {
    let chars = style.format.characters();

    if height < 2 {
        panic!("Height is too small")
    }

    if width < 2 {
        panic!("Width is too small")
    }

    // Top bar drawing
    match style.fill {
        BoxFill::None => write!(stdout, "{}", Bg(Reset))?,
        BoxFill::Color(c) => write!(stdout, "{}", Bg(c))?,
        BoxFill::Fill(_) => write!(stdout, "{}", Bg(Reset))?
    }

    write!(stdout, "{}{}", Fg(style.lines_color), cursor::Goto(x, y))?;
    write!(stdout, "{}{}{}", chars.top_left, String::from(chars.horizontal).repeat((width - 4) as usize), chars.top_right)?;

    match style.fill {
        BoxFill::None => {
            for ord in (y+1)..(y+height-1) {
                write!(stdout, "{}{}{}{}", cursor::Goto(x, ord), chars.vertical, cursor::Goto(x+width-1, ord), chars.vertical)?;
            }
        }
        BoxFill::Color(_) => {
            for ord in (y+1)..(y+height-1) {
                write!(stdout, "{}{}{}{}", cursor::Goto(x, ord), chars.vertical, String::from(' ').repeat((width-4) as usize), chars.vertical)?;
            }
        }
        BoxFill::Fill(c) => {
            for ord in (y+1)..(y+height-1) {
                write!(stdout, "{}{}{}{}", cursor::Goto(x, ord), chars.vertical, String::from(c).repeat((width-4) as usize), chars.vertical)?;
            }
        }
    }

    write!(stdout, "{}{}{}{}", cursor::Goto(x, y+height-1), chars.bottom_left, String::from(chars.horizontal).repeat((width - 4) as usize), chars.bottom_right)?;

    stdout.flush()?;

    Ok(())
}