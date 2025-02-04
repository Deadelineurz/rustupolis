use super::colors::*;
use crate::terminal::boxes::*;
use std::io::{Error, Stdout};

pub fn draw_sidebar(
    stdout: &mut Stdout,
    terminal_size: (u16, u16),
    side_bar_size: u16,
) -> Result<(), Error> {
    let side_bar_size = terminal_size.0 - side_bar_size;

    draw_box(
        stdout,
        side_bar_size,
        1,
        terminal_size.0 - side_bar_size + 1,
        terminal_size.1 + 1,
        BoxStyle::new()
            .fill(BoxFill::color(UI_BLACK_COLOR))
            .lines_color(UI_WHITE_COLOR),
    )
}
