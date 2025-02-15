use crate::engine::drawable::Drawable;

#[derive(Debug)]
pub struct BuildingDrawable {
    pub xpos: i16,
    pub ypos: i16,
    pub color: ansi_term::Color,
    pub content: String
}

#[derive(Debug)]
pub struct RoadDrawable {
    pub start_x: i16,
    pub start_y: i16,
    pub horizontal : bool,
    pub pavement : char,
    pub width: u8,
    pub length: u8,
    pub color: ansi_term::Color,
}


impl Drawable for BuildingDrawable {
    fn x(&self) -> i16 {
        self.xpos
    }

    fn y(&self) -> i16 {
        self.ypos
    }

    fn width(&self) -> u8 {
        let mut n : u8 = 0;
        for line in self.content.lines() {
            if n == 0 {
                n = line.chars().count() as u8
            }
            else if (line.chars().count() as u8) != n {
                panic!("BuildingDrawable: Invalid line width, all lines need to have the same width")
            }
        }
        n
    }

    fn height(&self) -> u8 {
        let mut n = 0;
        for _ in self.content.lines() {
            n+=1;
        }
        n
    }

    fn shape(&self) -> String {
        self.content.to_string()
    }

    fn color(&self) -> ansi_term::Color {
        self.color
    }
}

impl Drawable for RoadDrawable {
    fn x(&self) -> i16 {
        self.start_x
    }

    fn y(&self) -> i16 {
        self.start_y
    }

    fn width(&self) -> u8 {
        if self.horizontal {
            self.length
        }
        else {
            self.width
        }
    }

    fn height(&self) -> u8 {
        if self.horizontal {
            self.width
        }
        else {
            self.length
        }
    }

    fn shape(&self) -> String {
        if self.horizontal {
            let mut str = self.pavement.to_string().repeat(self.length as usize);
            for _ in 0..self.width {
                str += "\n";
                str += &*self.pavement.to_string().repeat(self.length as usize)
            }
            str
        }
        else {
            let mut shape : String = self.pavement.to_string().repeat(self.width as usize);
            for _ in 0..self.length {
                shape += "\n";
                shape += &*self.pavement.to_string().repeat(self.width as usize)
            }

                shape
        }
    }

    fn color(&self) -> ansi_term::Color {
        self.color
    }
}
