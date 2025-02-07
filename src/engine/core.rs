use log::{debug, info, trace};
use termion::cursor;
use crate::engine::drawable::Drawable;
use crate::engine::viewport::Viewport;


pub struct Engine {
    pub viewport: Viewport,
    drawables: Vec<Box<dyn Drawable>>
}

impl Engine {
    pub fn register_drawable(&mut self, drawable: Box<dyn Drawable>) {
        self.drawables.push(drawable)
    }

    pub fn refresh(&self) {
        for d in self.drawables.iter().filter(|i| self.viewport.is_visible(*i)) {
            let mut coordinates = self.viewport.get_output_coordinates(d);

            for line in &d.shape().lines().collect::<Vec<&str>>()[coordinates.crop_top..(d.height() as usize - coordinates.crop_bottom)] {
                trace!("full: {:?} | cropped: {:?}", line, &line[coordinates.crop_left..(line.len() - coordinates.crop_right)]);
                print!("{}{}", cursor::Goto(coordinates.x, coordinates.y), &line[coordinates.crop_left..(d.width() as usize - coordinates.crop_right)]);
                coordinates.y += 1;
            }
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