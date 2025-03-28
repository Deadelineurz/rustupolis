use crate::engine::drawable::DynDrawable;
use crate::engine::keybinds::Tty;
use crate::engine::viewport::{background, Viewport};
use crate::ui::colors::A_UI_BLACK_LIGHT_COLOR;
use log::trace;
use std::io::Write;
use std::sync::{Arc, RwLock};
use std::sync::mpsc::Sender;
use termion::{cursor, terminal_size};
use crate::threads::sidebar::SideBarMessage;
use crate::ui::sidebar::SideBar;

pub type LockableEngine = Arc<RwLock<Engine>>;

pub struct Engine {
    pub viewport: Viewport,
    pub side_bar_tx: Sender<SideBarMessage>,
    pub background: String,
    pub stdout: Arc<Tty>,
    drawables: Vec<Box<DynDrawable>>,
}

impl Engine {
    pub fn register_drawable(&mut self, drawable: Box<DynDrawable>) {
        self.drawables.push(drawable)
    }

    pub fn refresh(&self) {
        self.clear_viewport();

        for d in self
            .drawables
            .iter()
            .filter(|i| self.viewport.is_visible(*i))
        {
            let mut coordinates = self.viewport.get_output_coordinates(d);
            trace!("blit at: {:?}", coordinates);

            for line in &d.shape().lines().collect::<Vec<&str>>()
                [coordinates.crop_top..(d.height() as usize - coordinates.crop_bottom)]
            {
                let _ = write!(
                    self.stdout.lock(),
                    "{}{}",
                    cursor::Goto(coordinates.x, coordinates.y),
                    d.color().paint(
                        line.chars().collect::<Vec<char>>()
                            [coordinates.crop_left..(d.width() as usize - coordinates.crop_right)]
                            .iter()
                            .collect::<String>()
                    )
                );
                coordinates.y += 1;
            }
        }

        self.stdout.lock().flush().unwrap()
    }

    pub fn get_drawable_for_coordinates(&self, x: i16, y: i16) -> Option<&Box<DynDrawable>> {
        self.drawables
            .iter()
            .find(|it| it.x() <= x && it.right() > x && it.y() <= y && it.bottom() > y)
    }

    fn clear_viewport(&self) {
        let bg_lines = self.background.lines().collect::<Vec<&str>>();
        trace!("{}", bg_lines[0].chars().count());
        for y in self.viewport.output_y..(self.viewport.output_y + self.viewport.height) {
            print!(
                "{}{}",
                cursor::Goto(self.viewport.output_x, y),
                A_UI_BLACK_LIGHT_COLOR.paint(bg_lines[(y - self.viewport.output_y) as usize])
            )
        }

        self.stdout.lock().flush().unwrap()
    }

    pub fn new(viewport: Viewport, stdout: Arc<Tty>, chan: Sender<SideBarMessage>) -> Self {
        trace!("{:?}", terminal_size());
        Engine {
            viewport,
            stdout,
            side_bar_tx: chan,
            drawables: vec![],
            background: { background(viewport.output_y, viewport.width, viewport.height) },
        }
    }
}