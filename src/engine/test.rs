use crate::engine::drawable::Drawable;

#[derive(Debug)]
pub struct TestDrawable {

}

#[derive(Debug)]
pub struct Test2Drawable {
    pub xpos: i16,
    pub ypos: i16
}

impl Drawable for TestDrawable {
    fn x(&self) -> i16 {
        25
    }

    fn y(&self) -> i16 {
        12
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

impl Drawable for Test2Drawable {
    fn x(&self) -> i16 {
        self.xpos
    }

    fn y(&self) -> i16 {
        self.ypos
    }

    fn width(&self) -> u8 {
        3
    }

    fn height(&self) -> u8 {
        3
    }

    fn shape(&self) -> String {
        "bbb\ndef\nghi".to_string()
    }
}