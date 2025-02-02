pub trait Drawable {
    fn x(&self) -> i16;
    fn y(&self) -> i16;
    fn width(&self) -> u8;
    fn height(&self) -> u8;
}

pub trait DrawableUtils {
    fn right(&self) -> i16;
    fn bottom(&self) -> i16;
}

impl<T: Drawable> DrawableUtils for T {
    fn right(&self) -> i16 {
        self.x() + (self.width() as i16)
    }

    fn bottom(&self) -> i16 {
        self.y() + (self.height() as i16)
    }
}

