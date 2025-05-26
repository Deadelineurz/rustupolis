use termion::terminal_size;

use super::colors::*;
use crate::engine::keybinds::Tty;
use crate::terminal::boxes::*;
use crate::terminal::lines::{draw_line, LineDirection, LineStyle};
use crate::terminal::text::draw_text;
use std::ops::Deref;
use std::{io::Error, sync::Arc};

const TOPBAR_HEIGHT_MULTIPLIER: u16 = 10;

pub struct TopBar {
    stdout: Arc<Tty>,
    hide: bool,
    ter_width: u16,
    ter_height: u16,

    width: u16,
    height: u16,

    day_number: u32,
}

impl TopBar {
    /// Will get the terminal size
    pub fn new(stdout: Arc<Tty>, right_width_offset: u16) -> Self {
        let (x, y) = terminal_size().unwrap();
        let width = x - right_width_offset - 1;
        let height = y / TOPBAR_HEIGHT_MULTIPLIER;

        TopBar {
            stdout,
            hide: false,
            ter_width: x,
            ter_height: y,
            width,
            height,

            day_number: 0,
        }
    }

    pub fn get_height(&self) -> u16 {
        self.height
    }

    pub fn next_day(&mut self) {
        self.day_number += 1;
    }

    pub fn update_terminal_size(&mut self) {
        let (x, y) = terminal_size().unwrap();
        self.ter_width = x;
        self.ter_height = y;

        self.height = y / TOPBAR_HEIGHT_MULTIPLIER;
    }

    /// Update the population number on the topbar.
    pub fn update_displayed_population(&self, amount: usize) -> Result<(), Error> {
        draw_text(
            &self.stdout,
            &(amount.to_string().to_owned() + &" ".repeat(4)),
            16,
            3,
            UI_WHITE_COLOR,
            UI_BLACK_COLOR,
        )
    }

    /// Update the year number on the topbar.  
    /// \(Will add 2000 to the amount provided)
    pub fn update_displayed_year(&self, amount: usize) -> Result<(), Error> {
        draw_text(
            &self.stdout,
            &((2000 + amount).to_string().to_owned() + &" ".repeat(1)),
            4,
            3,
            UI_WHITE_COLOR,
            UI_BLACK_COLOR,
        )
    }

    pub fn update_displayed_happiness(&self, percentage: f32) -> Result<(), Error> {
        draw_text(
            &self.stdout,
            &(((percentage * 100.0) as u8).to_string().to_owned() + "%" + &" ".repeat(1)),
            31,
            3,
            UI_WHITE_COLOR,
            UI_BLACK_COLOR,
        )
    }

    pub fn update_displayed_workers(&self, amount: u16, population: usize) -> Result<(), Error> {
        draw_text(
            &self.stdout,
            &(amount.to_string().to_owned() + &" ".repeat(1)),
            44,
            3,
            UI_WHITE_COLOR,
            UI_BLACK_COLOR,
        )?;
        draw_text(
            &self.stdout,
            &(((amount as f32 / population as f32) as u8).to_string().to_owned() + "%" + &" ".repeat(1)),
            44,
            4,
            UI_WHITE_COLOR,
            UI_BLACK_COLOR,
        )
    }

    pub fn draw(&self) -> Result<(), Error> {
        if self.hide {
            return Ok(());
        }

        draw_box(
            self.stdout.deref(),
            1,
            1,
            self.width,
            self.height + 1,
            BoxStyle::new()
                .fill(BoxFill::color(UI_BLACK_COLOR))
                .lines_color(UI_WHITE_COLOR),
        )?;
        draw_text(&self.stdout, "Year :", 3, 2, UI_WHITE_COLOR, UI_BLACK_COLOR)?;
        draw_text(
            &self.stdout,
            &2000.to_string(),
            4,
            3,
            UI_WHITE_COLOR,
            UI_BLACK_COLOR,
        )?;

        draw_line(
            &self.stdout,
            10,
            2,
            3,
            LineStyle::new().direction(LineDirection::Vertical),
        )?;

        draw_text(
            &self.stdout,
            "Population :",
            12,
            2,
            UI_WHITE_COLOR,
            UI_BLACK_COLOR,
        )?;
        draw_text(&self.stdout, "100", 16, 3, UI_WHITE_COLOR, UI_BLACK_COLOR)?;

        draw_line(
            &self.stdout,
            25,
            2,
            3,
            LineStyle::new().direction(LineDirection::Vertical),
        )?;

        draw_text(
            &self.stdout,
            "Happiness :",
            27,
            2,
            UI_WHITE_COLOR,
            UI_BLACK_COLOR,
        )?;
        draw_text(&self.stdout, "50%", 31, 3, UI_WHITE_COLOR, UI_BLACK_COLOR)?;

        draw_line(
            &self.stdout,
            39,
            2,
            3,
            LineStyle::new().direction(LineDirection::Vertical),
        )?;

        draw_text(
            &self.stdout,
            "Workers :",
            41,
            2,
            UI_WHITE_COLOR,
            UI_BLACK_COLOR,
        )?;
        draw_text(&self.stdout, "0", 44, 3, UI_WHITE_COLOR, UI_BLACK_COLOR)?;

        draw_line(
            &self.stdout,
            51,
            2,
            3,
            LineStyle::new().direction(LineDirection::Vertical),
        )
    }
}
