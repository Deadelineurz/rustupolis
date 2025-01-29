use std::error::Error;
use std::io::{stdout, Stdout};
use std::thread::sleep;
use std::time::Duration;
use rustupolis::terminal::screen::CleanScreen;
use rustupolis::ui::bars::*;
use termion::*;

fn main()  -> Result<(), Box<dyn Error>> {

    let mut stdout: Stdout = stdout();
    let _scr = CleanScreen::new();

    loop {

        let size = terminal_size()?;
    
    
        draw_ui(&mut stdout, size, None)?;

        sleep(Duration::from_secs(1));

    }


    // Ok(())
}