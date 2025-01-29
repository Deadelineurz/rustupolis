use crate::terminal::boxes::*;
use super::colors::*;
use std::io::{Stdout, Error};

const TOPBAR_HEIGHT_PERCENTAGE: f32 = 0.1;
const BOTTOMBAR_HEIGHT_PERCENTAGE: f32 = 0.1;


fn draw_sidebar(stdout: &mut Stdout, terminal_size: (u16, u16), side_bar_size: u16) -> Result<(), Error> {
    let side_bar_size = terminal_size.0 - side_bar_size;

    draw_box(
        stdout,
        side_bar_size,
        1,
        terminal_size.0 - side_bar_size + 1,
        terminal_size.1 + 1,
        BoxStyle::new().fill(BoxFill::color(UI_BLACK_COLOR)).lines_color(UI_WHITE_COLOR),
    )
}

fn draw_topbar(stdout: &mut Stdout, terminal_size: (u16, u16), side_bar_size: u16) -> Result<(), Error> {
    draw_box(
        stdout,
        1,
        1,
        terminal_size.0 - side_bar_size - 1,
        (terminal_size.1 as f32 * TOPBAR_HEIGHT_PERCENTAGE) as u16,
        BoxStyle::new().fill(BoxFill::color(UI_BLACK_COLOR)).lines_color(UI_WHITE_COLOR),
    )
}


fn draw_bottombar(stdout: &mut Stdout, terminal_size: (u16, u16), side_bar_size: u16) -> Result<(), Error> {
    let height = (terminal_size.1 as f32 * BOTTOMBAR_HEIGHT_PERCENTAGE) as u16;
    draw_box(
        stdout,
        1,
        terminal_size.1 - height / 2,
        terminal_size.0 - side_bar_size - 1,
        height,
        BoxStyle::new().fill(BoxFill::color(UI_BLACK_COLOR)).lines_color(UI_WHITE_COLOR),
    )
}


pub fn draw_ui(stdout: &mut Stdout, terminal_size: (u16, u16), side_bar_size: Option<u16>) -> Result<(), Error> {
    let side_bar_size = side_bar_size.unwrap_or(terminal_size.0 / 4);

    draw_sidebar(stdout, terminal_size, side_bar_size)?;
    draw_topbar(stdout, terminal_size, side_bar_size)?;
    draw_bottombar(stdout, terminal_size, side_bar_size)?;

    Ok(())
}