use rustupolis::terminal::lines::LineDirection::Vertical;
use rustupolis::terminal::lines::LineFormat::{Dashed, Double, Solid};
use rustupolis::terminal::lines::LinePosition::{Bottom, Center, Right};
use rustupolis::terminal::lines::{draw_line, LineStyle};
use rustupolis::terminal::screen::{set_background, set_color, set_foreground, CleanScreen};
use std::error::Error;
use std::io::{stdout, Stdout};
use std::thread::sleep;
use std::time::Duration;
use termion::color::{Blue, Red, Rgb};


fn main() -> Result<(), Box<dyn Error>> {
    let mut stdout: Stdout = stdout();

    let _scr = CleanScreen::new();

    set_foreground(Red);
    draw_line(&mut stdout, 15, 15, 25, LineStyle::new().format(Dashed).position(Bottom))?;

    set_background(Blue);
    draw_line(&mut stdout, 10, 25, 58, LineStyle::new().format(Double).position(Center))?;

    set_color(Rgb(255, 0, 255), Rgb(4, 4, 4));
    draw_line(&mut stdout, 60, 32, 15, LineStyle::new().format(Solid).direction(Vertical).position(Right))?;

    sleep(Duration::from_secs(5));

    Ok(())
}