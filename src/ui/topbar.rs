use super::colors::*;
use crate::terminal::{
    boxes::*,
    lines::{draw_line, LineDirection, LineStyle}, text::draw_text,
};
use std::io::{Error, Stdout};
use termion::input::MouseTerminal;
use termion::raw::RawTerminal;

const TOPBAR_HEIGHT_PERCENTAGE: f32 = 0.1;

#[derive(Debug, Clone)]

pub struct TopBarInfo {
    pub date: String,
    pub population: u32,
    pub happiness: f32,

}


pub fn draw_topbar(stdout: &MouseTerminal<RawTerminal<Stdout>>, terminal_size: (u16, u16), side_bar_size: u16) -> Result<(), Error> {
    let date = " Lorem Ipsum ";

    let term_width = terminal_size.0;
    let term_height = terminal_size.1;

    let height = ((term_height as f32) * TOPBAR_HEIGHT_PERCENTAGE).round() as u16;
    let separator1_x = date.len() as u16 + 3 ;
    // let separator2_x = ((term_width as f32) * TOPBAR_SEPARATOR2_PERCENTAGE).round() as u16;

    draw_box(
        stdout,
        1,
        1,
        term_width - side_bar_size - 1,
        height,
        BoxStyle::new()
            .fill(BoxFill::color(UI_BLACK_COLOR))
            .lines_color(UI_WHITE_COLOR),
    )?;

    draw_text(stdout, date, 3, 2, UI_WHITE_COLOR, UI_BLACK_COLOR)?;

    draw_line(
        stdout,
        separator1_x,
        2,
        height - 2,
        LineStyle::new().direction(LineDirection::Vertical),
    )?;

    Ok(())
}