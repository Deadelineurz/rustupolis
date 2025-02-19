use std::fmt::Debug;
use downcast_rs::{impl_downcast, Downcast};

pub type DynDrawable = dyn Drawable;

pub trait Drawable: Downcast
where Self: Debug + Send {
    fn x(&self) -> i16;
    fn y(&self) -> i16;
    fn width(&self) -> u8;
    fn height(&self) -> u8;
    fn shape(&self) -> String;
}

impl_downcast!(Drawable);

impl DynDrawable {
    pub fn right(&self) -> i16 {
        self.x() + (self.width() as i16)
    }

    pub fn bottom(&self) -> i16 {
        self.y() + (self.height() as i16)
    }
}

