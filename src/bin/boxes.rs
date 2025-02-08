use rustupolis::terminal::boxes::{draw_box, BoxFill, BoxStyle};
use rustupolis::terminal::screen::CleanScreen;
use std::error::Error;
use std::io::{stdout, Stdout};
use std::thread::sleep;
use std::time::Duration;
use termion::color::{Blue, Green, Magenta, Red, Reset};
use rustupolis::terminal::text::draw_text;

fn main() -> Result<(), Box<dyn Error>> {
    let mut stdout: Stdout = stdout();

    let _scr = CleanScreen::new();
    let mut abs = 5;

    draw_text(&mut stdout, "Hello", abs+1, 6, Blue, Reset)?;

    draw_box(&mut stdout, abs, 5, 10, 10, BoxStyle::new().fill(BoxFill::none()))?;

    abs += 10;
    draw_text(&mut stdout, "Hello", abs+1, 6, Red, Reset)?;
    draw_box(&mut stdout, abs, 5, 10, 10, BoxStyle::new().fill(BoxFill::blank()))?;

    abs += 10;
    draw_text(&mut stdout, "Hello", abs+1, 6, Magenta, Reset)?;
    draw_box(&mut stdout, abs, 5, 10, 10, BoxStyle::new().fill(BoxFill::fill('⣿')))?;

    abs += 10;
    draw_text(&mut stdout, "Hello", abs+1, 6, Magenta, Reset)?;
    draw_box(&mut stdout, abs, 5, 10, 10, BoxStyle::new().fill(BoxFill::color(Green)))?;

    sleep(Duration::from_secs(5));

    Ok(())
}