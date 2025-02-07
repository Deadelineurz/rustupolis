use log::trace;
use termion::terminal_size;
use crate::engine::drawable::{Drawable};

#[derive(Debug, Copy, Clone)]
pub struct Viewport {
    output_x: u16,
    output_y: u16,

    virtual_x: i16,
    virtual_y: i16,

    width: u16,
    height: u16
}

#[derive(Debug, Copy, Clone)]
pub struct OutputCoordinates {
    pub x: u16,
    pub y: u16,
    pub crop_left: usize,
    pub crop_right: usize,
    pub crop_top: usize,
    pub crop_bottom: usize
}


impl Viewport {

    pub fn is_visible(&self, d: &Box<dyn Drawable>) -> bool {
        let mut res = true;
        res &= d.right() > self.virtual_x;

        res &= d.x() < self.right();

        res &= d.bottom() > self.virtual_y;

        res &= d.y() < self.bottom();

        res
    }

    pub fn get_output_coordinates(&self, d: &Box<dyn Drawable>) -> OutputCoordinates {
        let x = if d.x() < self.virtual_x {
                0
            } else {
                d.x() - self.virtual_x
            } as u16 + self.output_x;
        let y = if d.y() < self.virtual_y {
                0
            } else {
                d.y() - self.virtual_y
            } as u16 + self.output_y;

        let crop_left = if d.x() > self.virtual_x {
            0
        } else {
            self.virtual_x - d.x()
        } as usize;

        let crop_right = if d.right() < self.right() {
            0
        } else {
            d.right() - self.right()
        } as usize;

        let crop_top = if d.y() > self.virtual_y {
            0
        } else {
            self.virtual_y - d.y()
        } as usize;

        let crop_bottom = if d.bottom() < self.bottom() {
            0
        } else {
            d.bottom() - self.bottom()
        } as usize;

        OutputCoordinates{
            x,
            y,
            crop_left,
            crop_right,
            crop_top,
            crop_bottom
        }
    }

    pub fn right(&self) -> i16 {
        self.virtual_x + self.width as i16
    }

    pub fn bottom(&self) -> i16 {
        self.virtual_y + self.height as i16
    }
}

impl Default for Viewport {
    fn default() -> Self {
        let (width, height) = terminal_size().unwrap();

        Viewport{
            output_x: 1,
            output_y: 1,
            virtual_x: 0,
            virtual_y: 0,
            width,
            height,
        }
    }
}