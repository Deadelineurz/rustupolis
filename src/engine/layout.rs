use std::{fmt::Display, str::FromStr};
use std::ops::Deref;
use std::sync::Mutex;
use lazy_static::lazy_static;
use crate::ui::colors::*;
use serde::{Deserialize, Serialize};
use strum_macros::EnumString;
use crate::ENGINE;
use crate::engine::core::Engine;
use crate::engine::keybinds::Clickable;
use super::drawable::Drawable;

lazy_static! {
    pub static ref LAYOUT: Mutex<Layout> = Mutex::new(Layout {
        buildings: Vec::new(),
        roads: Vec::new(),
    });

}

pub fn get_layout() -> Arc<Mutex<Layout>> {
    dbg!("getted lyaout");
    let mut lock = LAYOUT.try_lock();
    if let Ok( mutex) = lock {
        return mutex;
    } else {
        panic!("try_lock failed");
    }

}
#[derive(Debug, EnumString, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum BuildingType {
    custom,
    uniform,
    empty_space,
}

impl Display for BuildingType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Building {
    pub name: String,
    pub id: String,
    pub district_id: usize,
    pub pos_x: i16,
    pub pos_y: i16,
    pub b_type: String,
    pub width: Option<u8>,
    pub height: Option<u8>,
    pub texture: Option<char>,
    pub content: Option<Vec<String>>,
    pub engine: Arc<Mutex<Engine>>,
    pub layout: Arc<Mutex<Layout>>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Road {
    name: String,
    id: String,
    start_x: i16,
    start_y: i16,
    horizontal: bool,
    width: u8,
    length: u8,
    pavement: char,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Layout {
    pub buildings: Vec<Building>,
    pub roads: Vec<Road>,
    pub engine: Arc<Mutex<Engine>>
}

impl Layout {
    pub fn new(engine: Arc<Mutex<Engine>>) -> Self {
    }

    pub fn load_default_layout() {
        let layout = include_str!("../initial_data/layout.json");

        let layout_obj: Layout = serde_json::from_str(layout).unwrap();
        let mut global_layout = get_layout();
        *global_layout = layout_obj;
    }

    pub fn load_core_layout() {
        let layout = include_str!("../initial_data/starting_core.json");

        let layout_obj: Layout = serde_json::from_str(layout).unwrap();

        let mut global_layout = get_layout();
        *global_layout = layout_obj;
    }

    pub fn add_building(&mut self, building: Building) {
        let mut layout = LAYOUT.lock().unwrap();
        layout.buildings.push(building);
    }

    pub fn add_road(road: Road) {
        let mut layout = LAYOUT.lock().unwrap();
        layout.roads.push(road);
    }

    /// Clone the vec
    pub fn get_buildings(&self) -> Vec<Building> {
        let layout = LAYOUT.lock().unwrap();
        layout.buildings.iter().map(|b| b.clone()).collect()
    }

    /// Clone the vec
    pub fn get_roads(&self) -> Vec<Road> {
        let layout = LAYOUT.lock().unwrap();
        layout.roads.iter().map(|r| r.clone()).collect()
    }
}


impl Clickable for Building {
    fn infos(&self) -> Option<String> {
        let mut layout = get_layout();
        let mut build = layout.buildings[0].clone();
        build.pos_x += 30;
        build.name = "aaa".to_string();

        layout.buildings.push(build);
        dbg!("aaaaaaa");

        dbg!(layout.buildings.last());
        match ENGINE.deref().lock() {
            Ok(guard) => {
                guard.register_drawable(Box::new(build))
            }
            _ => {}
        }

        Option::from(self.name.to_string())
    }
}
impl Clickable for Road {
    fn infos(&self) -> Option<String> {
        Option::from(self.name.to_string())
    }
}
impl Drawable for Building {

    fn x(&self) -> i16 {
        self.pos_x
    }

    fn y(&self) -> i16 {
        self.pos_y
    }

    fn width(&self) -> u8 {
        if &self.b_type == "custom" {
            let mut n: u8 = 0;
            let lines = (self.content.as_ref()).unwrap();
            for line in lines {
                if n == 0 {
                    n = line.chars().count() as u8
                } else if (line.chars().count() as u8) != n {
                    panic!("BuildingDrawable: Invalid line width, all lines need to have the same width")
                }
            }
            n
        } else {
            self.width.unwrap()
        }
    }

    fn height(&self) -> u8 {
        if self.b_type == BuildingType::custom.to_string() {
            return self.content.as_ref().unwrap().len() as u8;
        }
        self.height.unwrap()
    }

    fn shape(&self) -> String {
        if self.b_type == BuildingType::custom.to_string() {
            return self.content.as_ref().unwrap().join("\n");
        } else if self.b_type == BuildingType::empty_space.to_string() {
            if self.width.unwrap() == 1 && self.height.unwrap() == 1 {
                return "▢\n".parse().unwrap();
            } else if self.height.unwrap() == 1 {
                let mut str: String = "╞".to_string();
                str.push_str(&*"═".repeat(self.width.unwrap() as usize - 2));
                str.push_str("╡\n");
                return str;
            } else if self.width.unwrap() == 1 {
                let mut str: String = "╦\n".to_string();
                str.push_str(&*"║\n".repeat(self.height.unwrap() as usize - 2));
                str.push_str("╩\n");
                return str;
            } else {
                let in_layers = self.height.unwrap() - 2;
                let in_columns: u8 = self.width.unwrap() - 2;

                let mut str: String = "╔".to_string();

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
                return str;
            }
        }
        let mut str: String = "".to_string();
        for _ in 0..self.height.unwrap() {
            str += &*(self
                .texture
                .unwrap()
                .to_string()
                .repeat(self.width.unwrap() as usize));
            str += "\n";
        }
        str
    }

    fn color(&self) -> ansi_term::Color {
        match &self.b_type {
            s if BuildingType::from_str(s).unwrap() == BuildingType::empty_space => A_SAND_COLOR,
            _ => A_RUST_COLOR_1,
        }
    }
}

impl Drawable for Road {
    fn x(&self) -> i16 {
        self.start_x
    }

    fn y(&self) -> i16 {
        self.start_y
    }

    fn width(&self) -> u8 {
        if self.horizontal {
            self.length
        } else {
            self.width
        }
    }

    fn height(&self) -> u8 {
        if self.horizontal {
            self.width
        } else {
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
        } else {
            let mut shape: String = self.pavement.to_string().repeat(self.width as usize);
            for _ in 0..self.length {
                shape += "\n";
                shape += &*self.pavement.to_string().repeat(self.width as usize)
            }

            shape
        }
    }

    fn color(&self) -> ansi_term::Color {
        A_GREY_COLOR
    }
}
