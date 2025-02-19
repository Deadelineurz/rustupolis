use std::io::{stdout, Stdout, Write};
use crate::engine::drawable::{DynDrawable};
use crate::engine::viewport::Viewport;
use log::trace;
use termion::cursor;
use termion::input::MouseTerminal;
use termion::raw::RawTerminal;

pub struct Engine<'a> {
    pub viewport: Viewport,
    drawables: Vec<Box<DynDrawable>>,
    stdout: &'a MouseTerminal<RawTerminal<Stdout>>
}

impl<'a> Engine<'a> {
    pub fn register_drawable(&mut self, drawable: Box<DynDrawable>) {
        self.drawables.push(drawable)
    }

    pub fn refresh(&self) {
        self.clear_viewport();

        for d in self.drawables.iter().filter(|i| self.viewport.is_visible(*i)) {
            let mut coordinates = self.viewport.get_output_coordinates(d);
            trace!("blit at: {:?}", coordinates);

            for line in &d.shape().lines().collect::<Vec<&str>>()[coordinates.crop_top..(d.height() as usize - coordinates.crop_bottom)] {
                let _ = write!(self.stdout.lock(), "{}{}", cursor::Goto(coordinates.x, coordinates.y), &line[coordinates.crop_left..(d.width() as usize - coordinates.crop_right)]);
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
        for y in self.viewport.output_y..(self.viewport.output_y + self.viewport.height) {
            print!("{}{}", cursor::Goto(self.viewport.output_x, y), String::from(" ").repeat(self.viewport.width as usize))
        }

        self.stdout.lock().flush().unwrap()
    }

    pub fn new(viewport: Viewport, stdout: &'a MouseTerminal<RawTerminal<Stdout>>) -> Self {
        Engine{
            viewport,
            stdout,
            drawables: vec![]
        }
    }
}

impl<'a> From<&'a MouseTerminal<RawTerminal<Stdout>>> for Engine<'a> {
    fn from(value: &'a MouseTerminal<RawTerminal<Stdout>>) -> Self {
        Engine{
            viewport: Viewport::default(),
            drawables: vec![],
            stdout: value
        }
    }
}