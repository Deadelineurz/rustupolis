use std::error::Error;
use std::io::{stdout, Stdout};
use std::thread::sleep;
use std::time::Duration;
use rustupolis::terminal::screen::{set_background, CleanScreen};
use rustupolis::ui::*;
use rustupolis::ui::colors::*;
use termion::*;

fn main()  -> Result<(), Box<dyn Error>> {

    let mut stdout: Stdout = stdout();

    set_background(UI_BLACK_LIGHT_COLOR);
    let _scr = CleanScreen::new();

    // loop {

    //     let size = terminal_size()?;
    
    
    //     draw_ui(&mut stdout, size, None)?;

    //     sleep(Duration::from_secs(1));

    // }

    let size = terminal_size()?;
    
    draw_ui(&mut stdout, size, None)?;

    sleep(Duration::from_secs(5));

    Ok(())
}