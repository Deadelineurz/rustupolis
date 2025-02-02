use crate::engine::drawable::{Drawable, DrawableUtils};

pub struct Viewport {
    output_x: u16,
    output_y: u16,

    virtual_x: i16,
    virtual_y: i16,

    width: u16,
    height: u16
}

impl Viewport {

    fn is_visible<T: Drawable + DrawableUtils>(&self, d: &T) -> bool {
        if (d.right() > self.virtual_x) {
            return true
        }

        if (d.x() < self.right()) {
            return true
        }

        if (d.bottom() > self.virtual_y) {
            return true
        }

        if (d.y() < self.bottom()) {
            return true
        }

        false
    }
}

impl DrawableUtils for Viewport {
    fn right(&self) -> i16 {
        self.virtual_x + self.width as i16
    }

    fn bottom(&self) -> i16 {
        self.virtual_y + self.height as i16
    }
}