use crate::engine::drawable::DynDrawable;
use crate::engine::viewport::{background, Viewport};
use ansi_term::Color::{Black, Green, Red};
use log::trace;
use std::io::{stdout, Write};
use termion::{cursor, terminal_size};
use termion::color::LightBlack;

pub struct Engine {
    pub viewport: Viewport,
    pub background : String,
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
                print!("{}{}", cursor::Goto(coordinates.x, coordinates.y), d.color().paint(line.chars().collect::<Vec<char>>()[coordinates.crop_left..(d.width() as usize - coordinates.crop_right)].iter().collect::<String>()));
                coordinates.y += 1;
            }
        }

        stdout().flush().unwrap()
    }



    fn clear_viewport(&self) {
        for y in self.viewport.output_y..(self.viewport.output_y + self.viewport.height) {
            print!(
                "{}{}",
                cursor::Goto(self.viewport.output_x, y),
                Black.paint(self.background.to_string())
            )
        }
    }
}

impl From<Viewport> for Engine {
    fn from(value: Viewport) -> Self {
        Engine {
            viewport: value,
            drawables: Vec::new(),
            background: background(value.output_y, value.height, value.width)
        }
    }
}

impl Default for Engine {
    fn default() -> Self {
        Engine {
            viewport: Viewport::default(),
            drawables: Vec::new(),
            background: {
                let (width, height) = terminal_size().unwrap();
                background(1, height, width)
            }
        }
    }
}
