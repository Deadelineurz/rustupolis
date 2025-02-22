use super::colors::*;
use crate::engine::keybinds::Tty;
use crate::terminal::boxes::*;
use std::io::Error;

const BOTTOMBAR_HEIGHT_PERCENTAGE: f32 = 0.1;

pub fn draw_bottombar(
    stdout: &Tty,
    terminal_size: (u16, u16),
    side_bar_size: u16,
) -> Result<(), Error> {
    let height = (terminal_size.1 as f32 * BOTTOMBAR_HEIGHT_PERCENTAGE) as u16;
    draw_box(
        stdout,
        1,
        terminal_size.1 - height / 2,
        terminal_size.0 - side_bar_size - 1,
        height,
        BoxStyle::new()
            .fill(BoxFill::color(UI_BLACK_COLOR))
            .lines_color(UI_WHITE_COLOR),
    )
}
