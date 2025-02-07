pub trait Drawable
where Self: Sync {
    fn x(&self) -> i16;
    fn y(&self) -> i16;
    fn width(&self) -> u8;
    fn height(&self) -> u8;
    fn shape(&self) -> String;
}

impl dyn Drawable {
    pub fn right(&self) -> i16 {
        self.x() + (self.width() as i16)
    }

    pub fn bottom(&self) -> i16 {
        self.y() + (self.height() as i16)
    }
}

