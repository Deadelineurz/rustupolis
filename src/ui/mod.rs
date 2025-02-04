use std::io::{Error, Stdout};

pub mod topbar;
pub mod sidebar;
pub mod bottombar;

pub mod colors;

pub fn draw_ui(
    stdout: &mut Stdout,
    terminal_size: (u16, u16),
    side_bar_size: Option<u16>,
) -> Result<(), Error> {
    let side_bar_size = side_bar_size.unwrap_or(terminal_size.0 / 4);
    
    sidebar::draw_sidebar(stdout, terminal_size, side_bar_size)?;
    topbar::draw_topbar(stdout, terminal_size, side_bar_size)?;
    bottombar::draw_bottombar(stdout, terminal_size, side_bar_size)?;

    Ok(())
}