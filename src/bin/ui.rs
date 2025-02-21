use rustupolis::terminal::screen::{set_background, CleanScreen};
use rustupolis::ui::colors::*;
use rustupolis::ui::sidebar::*;
use std::error::Error;
use std::io::{stdout, Stdout};
use std::thread::sleep;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    let mut stdout: Stdout = stdout();

    set_background(UI_BLACK_LIGHT_COLOR);
    let _scr = CleanScreen::new();

    let mut side_bar = SideBar::new();

    side_bar.draw(&mut stdout)?;

    for i in 0..side_bar.get_max_number_of_logs() {
        side_bar.push_log(
            Box::new("test ".to_string() + &i.to_string()),
            rustupolis::ui::sidebar::LogType::Debug,
            rustupolis::ui::sidebar::LogColor::Normal,
        );

        sleep(Duration::from_millis(50));

        side_bar.draw_logs(&mut stdout)?;
    }

    sleep(Duration::from_secs(1));

    side_bar.display_custom_infos(
        &mut stdout,
        &"Building info:",
        &[
            &"Name = Trump Tower",
            &"ID = 3",
            &"Residents = 63",
            &"Size = ginormous",
            &"Girth = huge",
            &"Wow factor = WOooOooOOoW!!!",
        ],
    )?;

    sleep(Duration::from_secs(2));

    side_bar.push_multiline_log_and_display(
        &mut stdout,
        vec![
            Box::new("A problem as occured..."),
            Box::new("no it's a joke everything is fine."),
        ],
        LogType::Debug,
        LogColor::Important,
    )?;
    sleep(Duration::from_secs(1));

    side_bar.push_log_and_display(
        &mut stdout,
        Box::new("or is it ?"),
        LogType::Debug,
        LogColor::Unusual,
    )?;
    sleep(Duration::from_secs(2));

    side_bar.clear_custom_infos(&mut stdout)?;

    sleep(Duration::from_secs(2));

    side_bar.clear_logs(&mut stdout)?;

    sleep(Duration::from_secs(2));


    Ok(())
}
