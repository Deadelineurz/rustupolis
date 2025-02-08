use std::io::{stdout, Write};
use crate::engine::drawable::{Drawable, DynDrawable};
use crate::engine::viewport::Viewport;
use log::{debug, info, trace};
use termion::cursor;
use crate::terminal::boxes::draw_box;

pub struct Engine {
    pub viewport: Viewport,
    drawables: Vec<Box<DynDrawable>>
}

impl Engine {
    pub fn register_drawable(&mut self, drawable: Box<DynDrawable>) {
        self.drawables.push(drawable)
    }

    pub fn refresh(&self) {
        self.clear_viewport();

        for d in self.drawables.iter().filter(|i| self.viewport.is_visible(*i)) {
            let mut coordinates = self.viewport.get_output_coordinates(d);
            trace!("blit at: {:?}", coordinates);

            for line in &d.shape().lines().collect::<Vec<&str>>()[coordinates.crop_top..(d.height() as usize - coordinates.crop_bottom)] {
                print!("{}{}", cursor::Goto(coordinates.x, coordinates.y), &line[coordinates.crop_left..(d.width() as usize - coordinates.crop_right)]);
                coordinates.y += 1;
            }
        }

        stdout().flush().unwrap()
    }

    fn clear_viewport(&self) {
        for y in self.viewport.output_y..(self.viewport.output_y + self.viewport.height) {
            print!("{}{}", cursor::Goto(self.viewport.output_x, y), String::from(" ").repeat(self.viewport.width as usize))
        }
    }
}

impl From<Viewport> for Engine {
    fn from(value: Viewport) -> Self {
        Engine {
            viewport: value,
            drawables: Vec::new()
        }
    }
}

impl Default for Engine {
    fn default() -> Self {
        Engine {
            viewport: Viewport::default(),
            drawables: Vec::new()
        }
    }
}