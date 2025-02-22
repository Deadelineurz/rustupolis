use crate::engine::drawable::Drawable;
use crate::engine::keybinds::Clickable;

#[derive(Debug)]
pub struct BuildingDrawable {
    pub xpos: i16,
    pub ypos: i16,
    pub color: ansi_term::Color,
    pub content: Option<Vec<String>>,
    pub width : Option<u8>,
    pub height : Option<u8>,
    pub texture : Option<char>,
    pub b_type : String
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

impl Clickable for BuildingDrawable {
    fn infos(&self) -> Option<String> {
        Some(format!("texture: {:?}", self.texture))
    }
}


impl Drawable for BuildingDrawable {
    fn x(&self) -> i16 {
        self.xpos
    }

    fn y(&self) -> i16 {
        self.ypos
    }

    fn width(&self) -> u8 {
        if &self.b_type == "custom" {
            let mut n : u8 = 0;
            let lines = (self.content.as_ref()).unwrap();
                for line in lines {
                if n == 0 {

                    n = line.chars().count() as u8
                }
                else if (line.chars().count() as u8) != n {
                    panic!("BuildingDrawable: Invalid line width, all lines need to have the same width")
                }
            }
            n
        }
        else {
            self.width.unwrap()
        }
    }

    fn height(&self) -> u8 {
        if self.b_type == "custom" {
            return self.content.as_ref().unwrap().len() as u8
        }
        self.height.unwrap()
    }

    fn shape(&self) -> String {
        if self.b_type == "custom" {
            return self.content.as_ref().unwrap().join("\n");
        }
        else if &self.b_type == "empty_space" {
            if self.width.unwrap() == 1 && self.height.unwrap() == 1{
                return "▢\n".parse().unwrap();
            }
            else if self.height.unwrap() == 1 {
                let mut str : String = "╞".to_string();
                str.push_str(&*"═".repeat(self.width.unwrap() as usize - 2));
                str.push_str("╡\n");
                return str;
            }
            else if self.width.unwrap() == 1 {
                let mut str : String = "╦\n".to_string();
                str.push_str(&*"║\n".repeat(self.height.unwrap() as usize -2));
                str.push_str("╩\n");
                return str;
            }
            else {
                let in_layers = self.height.unwrap() - 2;
                let in_columns : u8 = self.width.unwrap() - 2;

                let mut str : String = "╔".to_string();

                //First Layer
                str.push_str(&*"═".repeat(in_columns as usize));
                str.push_str("╗\n");

                for _ in 0..in_layers {
                    let mut layer = "║".to_string();
                    layer.push_str(&*" ".repeat(in_columns as usize));
                    layer.push_str("║\n");
                    str.push_str(&*layer);
                }

                str.push_str("╚");
                str.push_str(&*"═".repeat(in_columns as usize));
                str.push_str("╝\n");
                return str ;
            }

        }
        let mut str: String = "".to_string();
        for _ in 0..self.height.unwrap() {
            str += &*(self.texture.unwrap().to_string().repeat(self.width.unwrap() as usize));
            str += "\n";
        }
        str
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

impl Clickable for RoadDrawable {}
