use termion::{color::Rgb, terminal_size};

use super::colors::*;
use crate::terminal::{
    boxes::*,
    lines::{draw_line, LineStyle},
    text::draw_text,
};
use std::{
    fmt::{self, Display, Formatter},
    io::{Error, Stdout},
    str::FromStr,
};

const DEFAULT_WIDTH_MULTIPLIER: u16 = 4; // one fourth of the screen
const BORDER_WIDTH: u16 = 2;
const SEPARATOR_HEIGHT: u16 = 3;

pub struct SideBar {
    hide: bool,
    ter_width: u16,
    ter_height: u16,

    width: u16,
    /// offset is the x coord of the leftmost border of the sidebar
    offset: u16,

    text_line_max_len: u16,
    /// the bottom part of the log separator
    log_separator_y_pos: u16,

    logs: Vec<(Vec<Box<dyn Display>>, LogType, LogColor)>,
}

impl SideBar {
    /// Will get the terminal size
    pub fn new() -> Self {
        let (x, y) = terminal_size().unwrap();
        let width = x / DEFAULT_WIDTH_MULTIPLIER;
        let offset = x - width;

        SideBar {
            hide: false,
            ter_width: x,
            ter_height: y,

            offset,
            width,

            text_line_max_len: width - BORDER_WIDTH,
            log_separator_y_pos: SEPARATOR_HEIGHT + 1,
            logs: Vec::new(),
        }
    }

    /// The number of chars that can be displayed on one line
    pub fn get_text_line_max_len(&self) -> u16 {
        self.text_line_max_len
    }

    /// The number of lines that can be show on the logs at once
    pub fn get_max_number_of_logs(&self) -> u16 {
        self.ter_height - self.log_separator_y_pos - 1
    }

    pub fn update_terminal_size(&mut self) {
        let (x, y) = terminal_size().unwrap();
        self.ter_width = x;
        self.ter_height = y;

        self.width = x / DEFAULT_WIDTH_MULTIPLIER;
        self.offset = x - self.width;

        self.text_line_max_len = self.width - BORDER_WIDTH;
    }

    pub fn draw(&self, stdout: &mut Stdout) -> Result<(), Error> {
        if self.hide {
            return Result::Ok(());
        }

        draw_box(
            stdout,
            self.offset,
            1,
            self.width + 1,
            self.ter_height + 1,
            BoxStyle::new()
                .fill(BoxFill::color(UI_BLACK_COLOR))
                .lines_color(UI_WHITE_COLOR),
        )?;

        self.draw_separator(stdout, &"Logs:", 1)
    }

    /// Draw the maximum of log possible to display, last inserted at the bottom.
    pub fn draw_logs(&self, stdout: &mut Stdout) -> Result<(), Error> {
        let mut count = 0;
        for log in self
            .logs
            .iter()
            .rev()
            .take(self.get_max_number_of_logs() as usize)
        {
            for line in log.0.iter().skip(1).rev() {
                self.draw_log_line(stdout, &line.to_string(), log.2, count)?;

                count += 1;
            }

            let mut header: String = String::from_str(&log.1.to_string()).unwrap();
            header.push_str(&log.0[0].to_string());
            self.draw_log_line(stdout, &header, log.2, count)?;

            count += 1;
        }

        Ok(())
    }

    fn draw_log_line(
        &self,
        stdout: &mut Stdout,
        text: &dyn Display,
        color: LogColor,
        y_offset: u16,
    ) -> Result<(), Error> {
        let mut line = String::from_str(&text.to_string()).unwrap();
        line.push_str(&" ".repeat(self.width as usize - line.len() - 1));
        draw_text(
            stdout,
            &line,
            self.offset + 1,
            self.ter_height - 1 - y_offset,
            match color {
                LogColor::Normal => UI_WHITE_COLOR,
                LogColor::Unusual => RUST_COLOR_1,
                LogColor::Important => Rgb(170, 20, 5),
            },
            UI_BLACK_COLOR,
        )
    }

    pub fn push_log(&mut self, log: Vec<Box<dyn Display>>, log_type: LogType, color: LogColor) {
        self.logs.push((log, log_type, color));
    }

    pub fn push_log_and_display(
        &mut self,
        stdout: &mut Stdout,
        log: Vec<Box<dyn Display>>,
        log_type: LogType,
        log_color: LogColor,
    ) -> Result<(), Error> {
        self.logs.push((log, log_type, log_color));

        self.draw_logs(stdout)
    }

    /// display custom infos at the top of the sidebar, each display in text is a new line. \
    /// text should not be larger than `get_text_line_max_len()`. \
    /// **Will update the place of the log separator to be able to draw them properly later!**
    pub fn display_custom_infos(
        &mut self,
        stdout: &mut Stdout,
        header: &dyn Display,
        text: &[&dyn Display],
    ) -> Result<(), Error> {
        self.draw_separator(stdout, &header, 1)?;

        for (y_offset, line) in text.iter().enumerate() {
            let mut line = String::from_str(&line.to_string()).unwrap();
            line.push_str(&" ".repeat(self.width as usize - line.len() - 1));

            draw_text(
                stdout,
                &line,
                self.offset + 1,
                SEPARATOR_HEIGHT + y_offset as u16 + 1,
                UI_WHITE_COLOR,
                UI_BLACK_COLOR,
            )?;
        }

        let pos = SEPARATOR_HEIGHT + text.len() as u16 + 1;
        self.draw_separator(stdout, &"LOGS:", pos)?;

        self.log_separator_y_pos = pos + SEPARATOR_HEIGHT;

        Ok(())
    }

    pub fn clear_custom_infos(&mut self, stdout: &mut Stdout) -> Result<(), Error> {
        draw_box(
            stdout,
            self.offset,
            1,
            self.width + 1,
            self.ter_height + 1,
            BoxStyle::new()
                .fill(BoxFill::color(UI_BLACK_COLOR))
                .lines_color(UI_WHITE_COLOR),
        )?;

        self.log_separator_y_pos = SEPARATOR_HEIGHT + 1;
        self.draw_separator(stdout, &"Logs:", 1)?;

        self.draw_logs(stdout)
    }

    /// Draw a simple separator (header + two lines).
    pub fn draw_separator(
        &self,
        stdout: &mut Stdout,
        title: &dyn Display,
        y: u16,
    ) -> Result<(), Error> {
        draw_line(stdout, self.offset + 1, y, self.width - 1, LineStyle::new())?;

        let mut title = String::from_str(&title.to_string()).unwrap();
        title.push_str(&" ".repeat(self.width as usize - title.len() - 1));

        draw_text(
            stdout,
            &title,
            self.offset + 1,
            y + 1,
            UI_WHITE_COLOR,
            UI_BLACK_COLOR,
        )?;

        draw_line(
            stdout,
            self.offset + 1,
            y + 2,
            self.width - 1,
            LineStyle::new(),
        )?;

        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum LogColor {
    Normal,
    Unusual,
    Important,
}

#[derive(Debug, Clone, Copy)]
pub enum LogType {
    None,
    Info,
    Event,
    City,
    Debug,
}

impl Display for LogType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::None => write!(f, ""),
            _ => write!(f, "[{:?}] ", self)
        }
        
    }
}
