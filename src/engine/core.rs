use crate::engine::drawable::DynDrawable;
use crate::engine::viewport::{background, Viewport};
use ansi_term::Color::{Black, Green};
use log::trace;
use std::io::{Write};
use termion::{cursor, terminal_size};
use crate::engine::keybinds::Tty;
use crate::ui::colors::{A_UI_BLACK_LIGHTER_COLOR, A_UI_BLACK_LIGHT_COLOR};

pub struct Engine<'a> {
    pub viewport: Viewport,
    pub background : String,
    drawables: Vec<Box<DynDrawable>>,
    stdout: &'a Tty
}

impl<'a> Engine<'a> {
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
                let _ = write!(self.stdout.lock(), "{}{}", cursor::Goto(coordinates.x, coordinates.y), d.color().paint(line.chars().collect::<Vec<char>>()[coordinates.crop_left..(d.width() as usize - coordinates.crop_right)].iter().collect::<String>()));
                coordinates.y += 1;
            }
        }

        self.stdout.lock().flush().unwrap()
    }

    pub fn get_drawable_for_coordinates(&self, x: i16, y: i16) -> Option<&Box<DynDrawable>> {
        self.drawables.iter().find(|it| {
            it.x() <= x && it.right() > x && it.y() <= y && it.bottom() > y
        })
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

    pub fn new(viewport: Viewport, stdout: &'a Tty) -> Self {
        trace!("{:?}", terminal_size());
        Engine{
            viewport,
            stdout,
            drawables: vec![],
            background: {
                background(viewport.output_y, viewport.width, viewport.height)
            }
        }
    }
}

impl<'a> From<&'a Tty> for Engine<'a> {
    fn from(value: &'a Tty) -> Self {
        Engine{
            viewport: Viewport::default(),
            drawables: Vec::new(),
            background: {
                let (width, height) = terminal_size().unwrap();
                background(1, width, height)
            },
            stdout: value
        }
    }
}