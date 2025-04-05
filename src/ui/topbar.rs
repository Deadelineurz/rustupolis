use termion::terminal_size;

use super::colors::*;
use crate::engine::keybinds::Tty;
use crate::terminal::boxes::*;
use std::ops::Deref;
use std::{io::Error, sync::Arc};

const TOPBAR_HEIGHT_MULTIPLIER: u16 = 8;

pub struct TopBar {
    stdout: Arc<Tty>,
    hide: bool,
    ter_width: u16,
    ter_height: u16,

    height: u16,

    day_number: u32,
}

impl TopBar {
    /// Will get the terminal size
    pub fn new(stdout: Arc<Tty>) -> Self {
        let (x, y) = terminal_size().unwrap();
        let height = x / TOPBAR_HEIGHT_MULTIPLIER;

        TopBar {
            stdout,
            hide: false,
            ter_width: x,
            ter_height: y,

            height,

            day_number: 0,
        }
    }

    pub fn next_day(mut self) {
        self.day_number += 1;
    }

    pub fn update_terminal_size(&mut self) {
        let (x, y) = terminal_size().unwrap();
        self.ter_width = x;
        self.ter_height = y;

        self.height = x / TOPBAR_HEIGHT_MULTIPLIER;
    }

    pub fn draw(&self) -> Result<(), Error> {
        if self.hide {
            return Result::Ok(());
        }

        draw_box(
            self.stdout.deref(),
            1,
            1,
            self.ter_width + 1,
            self.height + 1,
            BoxStyle::new()
                .fill(BoxFill::color(UI_BLACK_COLOR))
                .lines_color(UI_WHITE_COLOR),
        )
    }
}
