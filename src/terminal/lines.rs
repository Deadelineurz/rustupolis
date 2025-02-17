use std::cmp::PartialEq;
use std::fmt::Display;
use std::io::{Error, Stdout, Write};
use termion::color::{Bg, Color, Fg};

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum LinePosition {
    Top,
    Center,
    Bottom,
    Left,
    Right
}

impl Default for LinePosition {
    fn default() -> Self {
        LinePosition::Center
    }
}

#[derive(Debug, Clone, Copy)]
pub enum LineFormat {
    Solid,
    Double,
    Thick,
    Dashed
}

impl Default for LineFormat {
    fn default() -> Self {
        LineFormat::Solid
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum LineDirection {
    Horizontal,
    Vertical
}

impl Default for LineDirection {
    fn default() -> Self {
        LineDirection::Horizontal
    }
}

#[derive(Debug, Clone, Copy)]
pub struct LineStyle {
    pub position: LinePosition,
    pub format: LineFormat,
    pub direction: LineDirection
}

impl LineStyle {
    pub fn new() -> LineStyle {
        Self::default()
    }

    pub fn position(&self, position: LinePosition) -> Self {
        LineStyle{ position, .. *self }
    }

    pub fn format(&self, format: LineFormat) -> Self {
        LineStyle{ format, .. *self }
    }

    pub fn direction(&self, direction: LineDirection) -> Self {
        LineStyle{ direction, .. *self }
    }

    fn get_char(&self) -> Result<char, &str> {
        match (self.position, self.direction) {
            (LinePosition::Top, LineDirection::Vertical) |  (LinePosition::Bottom, LineDirection::Vertical) => return Err("Invalid direction"),
            (LinePosition::Left, LineDirection::Horizontal) | (LinePosition::Right, LineDirection::Horizontal) => return Err("Invalid direction"),
            (_, _) => {}
        }

       match self.format {
           LineFormat::Solid => {
               match self.position {
                   LinePosition::Top => Ok('🭶'),
                   LinePosition::Center => if self.direction == LineDirection::Horizontal { Ok('─') } else { Ok('│') },
                   LinePosition::Bottom => Ok('🭻'),
                   LinePosition::Left => Ok('🭰'),
                   LinePosition::Right => Ok('🭵')
               }
           },
           LineFormat::Dashed => {
               match self.position {
                   LinePosition::Top => Ok('⠉'),
                   LinePosition::Center => if self.direction == LineDirection::Horizontal { Ok('╌') } else { Ok('┊') },
                   LinePosition::Bottom => Ok('⣀'),
                   LinePosition::Left => Ok('⡇'),
                   LinePosition::Right => Ok('⢸'),
               }
           },
           LineFormat::Double => {
               match self.direction {
                   LineDirection::Horizontal => Ok('═'),
                   LineDirection::Vertical => Ok('║')
               }
           },
           LineFormat::Thick => {
               match self.position {
                   LinePosition::Top => Ok('🬂'),
                   LinePosition::Center => if self.direction == LineDirection::Horizontal { Ok('━') } else { Ok('┃') },
                   LinePosition::Bottom => Ok('🬭'),
                   LinePosition::Left => Ok('▐'),
                   LinePosition::Right => Ok('▌'),
               }
           }
       }
    }
}

impl Default for LineStyle {
    fn default() -> Self {
        LineStyle {
            position: LinePosition::Center,
            format: LineFormat::Solid,
            direction: LineDirection::Horizontal
        }
    }
}

pub fn draw_line(stdout: &mut Stdout, x: u16, y: u16, length: u16, style: LineStyle) -> Result<(), Error> {
    write!(stdout, "{}", termion::cursor::Goto(x, y))?;

    let ch = style.get_char().expect("Error while trying to create char");

    if style.direction == LineDirection::Vertical {
        for ord in y..(y + length) {
            write!(stdout, "{}{ch}", termion::cursor::Goto(x, ord))?;
        }
    } else {
        write!(stdout, "{}", String::from(ch).repeat(length as usize))?;
    }


    stdout.flush()?;
    Ok(())
}