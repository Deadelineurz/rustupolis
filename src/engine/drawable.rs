use std::fmt::Debug;
use crate::engine::keybinds::Clickable;
use crate::engine::layout::LayoutId;

pub type DynDrawable = dyn Drawable;

pub trait Drawable
where
    Self: Debug + Sync + Send + Clickable,
{
    fn x(&self) -> i16;
    fn y(&self) -> i16;
    fn width(&self) -> u8;
    fn height(&self) -> u8;
    fn shape(&self) -> String;
    fn color(&self) -> ansi_term::Color;
    fn id(&self) -> LayoutId;
}

impl DynDrawable {
    pub fn right(&self) -> i16 {
        self.x() + (self.width() as i16)
    }

    pub fn bottom(&self) -> i16 {
        self.y() + (self.height() as i16)
    }
}
