use log::trace;
use termion::terminal_size;
use crate::engine::drawable::{DynDrawable};

#[derive(Debug, Copy, Clone)]
pub struct Viewport {
    pub output_x: u16,
    pub output_y: u16,

    virtual_x: i16,
    virtual_y: i16,

    pub width: u16,
    pub height: u16
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

    pub fn is_visible(&self, d: &Box<DynDrawable>) -> bool {
        let mut res = true;
        res &= d.right() > self.virtual_x;

        res &= d.x() < self.right();

        res &= d.bottom() > self.virtual_y;

        res &= d.y() < self.bottom();

        res
    }

    pub fn move_x(&mut self, amount: i16) {
        self.virtual_x += amount;
        trace!("{:?}", self)
    }

    pub fn move_y(&mut self, amount: i16) {
        self.virtual_y += amount;
        trace!("{:?}", self)
    }

    pub fn get_virtual_coordinates(&self, x: u16, y: u16) -> (i16, i16) {
        ((x-1) as i16 + self.virtual_x, (y-1) as i16 + self.virtual_y)
    }

    pub fn get_output_coordinates(&self, d: &Box<DynDrawable>) -> OutputCoordinates {
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