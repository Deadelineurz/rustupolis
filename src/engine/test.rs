use crate::engine::drawable::Drawable;

pub struct TestDrawable {

}

impl Drawable for TestDrawable {
    fn x(&self) -> i16 {
        -5
    }

    fn y(&self) -> i16 {
        -4
    }

    fn width(&self) -> u8 {
        3
    }

    fn height(&self) -> u8 {
        3
    }

    fn shape(&self) -> String {
        "abc\ndef\nghi".to_string()
    }
}