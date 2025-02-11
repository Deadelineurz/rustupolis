use crate::engine::drawable::Drawable;

#[derive(Debug)]
pub struct TestDrawable {
    pub xpos: i16,
    pub ypos: i16,
    pub color: ansi_term::Color,
}

#[derive(Debug)]
pub struct Test2Drawable {
    pub xpos: i16,
    pub ypos: i16,
    pub color: ansi_term::Color,
}

#[derive(Debug)]
pub struct QG {
    pub xpos: i16,
    pub ypos: i16,
    pub color: ansi_term::Color,
}

impl Drawable for TestDrawable {
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
        "abc\ndef\nghi".to_string()
    }

    fn color(&self) -> ansi_term::Color {
        self.color
    }
}

impl Drawable for QG {
    fn x(&self) -> i16 {
        self.xpos
    }

    fn y(&self) -> i16 {
        self.ypos
    }

    fn width(&self) -> u8 {
        15
    }

    fn height(&self) -> u8 {
        5
    }

    fn shape(&self) -> String {
        "███████████████\n███████████████\n███████████████\n███████████████\n███████████████"
            .to_string()
    }

    fn color(&self) -> ansi_term::Color {
        self.color
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
        "b²b\n███\n█hi".to_string()
    }

    fn color(&self) -> ansi_term::Color {
        self.color
    }
}
