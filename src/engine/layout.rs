use crate::{
    population::people::BasePeopleInfo,
    ui::colors::*, POPULATION,
};
use serde::{de, Deserialize, Serialize};
use std::cmp::PartialEq;
use std::{fmt::Display, str::FromStr};
use std::fmt::{write, Debug, Formatter};
use std::ops::Deref;
use std::slice::Iter;
use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use serde::de::Error;
use super::{drawable::Drawable, keybinds::Clickable};

pub const LAYOUT_ID_LENGTH: usize = 12;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct LayoutId {
    value: [u8; LAYOUT_ID_LENGTH]
}

impl LayoutId {
    pub fn new(value: [u8; LAYOUT_ID_LENGTH]) -> Self {
        LayoutId {
            value
        }
    }

    pub fn iter(&self) -> Iter<'_, u8> {
        self.value.iter()
    }
}


impl Default for LayoutId {
    fn default() -> Self {
        LayoutId {
            value: [0u8; LAYOUT_ID_LENGTH]
        }
    }
}

impl Debug for LayoutId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", BASE64_STANDARD.encode(self.value))
    }
}


#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BuildingType {
    Custom,
    Uniform,
    EmptySpace,
}

impl Display for BuildingType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

// ----- BUILDINGS -----

#[derive(Deserialize, Clone, Debug)]
pub struct Building {
    name: String,
    #[serde(deserialize_with = "deserialize_b64")]
    pub id: LayoutId,
    district_id: usize,
    pos_x: i16,
    pos_y: i16,
    b_type: BuildingType,
    width: Option<u8>,
    height: Option<u8>,
    texture: Option<char>,
    content: Option<Vec<String>>,
}

impl Building {
    pub fn get_num_people_in_building(&self) -> usize {
        POPULATION.lock().unwrap()
            .get_district(self.district_id)
            .unwrap()
            .peoples
            .iter()
            .filter(|people| {
                if let Some(uuid) = people.get_building_uuid() {
                    *uuid == self.id
                } else {
                    false
                }
            })
            .count()
    }

    pub fn get_building_uuid(&self) -> LayoutId {
        self.id.clone()
    }

    pub fn get_district_id(&self) -> usize {
        self.district_id
    }
}

impl Clickable for Building {
    fn infos(&self) -> Option<Vec<String>> {
        Some(vec![
            String::from(format!("Name: {}", self.name)),
            String::from(format!("Position: {}, {}", self.pos_x, self.pos_y)),
            String::from(format!("Population: {}", self.get_num_people_in_building())),
            String::from(" ".to_string()), // act as a newline
        ])
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
        if self.b_type == BuildingType::Custom {
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
        if self.b_type == BuildingType::Custom {
            return self.content.as_ref().unwrap().len() as u8;
        }
        self.height.unwrap()
    }

    fn shape(&self) -> String {
        if self.b_type == BuildingType::Custom {
            return self.content.as_ref().unwrap().join("\n");
        } else if self.b_type == BuildingType::EmptySpace {
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
            s if s == &BuildingType::EmptySpace => A_SAND_COLOR,
            _ => A_RUST_COLOR_1,
        }
    }
}

// ----- ROADS -----

#[derive(Deserialize, Clone, Debug)]
pub struct Road {
    name: String,
    #[serde(deserialize_with = "deserialize_b64")]
    pub id: LayoutId,
    start_x: i16,
    start_y: i16,
    horizontal: bool,
    width: u8,
    length: u8,
    pavement: char,
}

impl Clickable for Road {
    fn infos(&self) -> Option<Vec<String>> {
        Some(vec![
            String::from(format!("Name: {}", self.name)),
            String::from(format!("Position: {}, {}", self.start_x, self.start_y)),
            String::from(format!("Length: {}", self.length)),
            String::from(" ".to_string()), // act as a newline
        ])
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


// ----- LAYOUT -----

#[derive(Deserialize, Debug)]
pub struct Layout {
    pub buildings: Vec<Building>,
    pub roads: Vec<Road>,
}

impl Layout {
    pub fn load_default_layout() -> Self {
        let layout = include_str!("../initial_data/layout.json");

        let layout_obj: Layout = serde_json::from_str(layout).unwrap();

        layout_obj
    }

    pub fn load_core_layout() -> Self {
        let layout = include_str!("../initial_data/starting_core.json");

        let layout_obj: Layout = serde_json::from_str(layout).unwrap();

        layout_obj
    }

    pub fn add_building(&mut self, building: Building) {
        self.buildings.push(building);
    }

    pub fn add_road(&mut self, road: Road) {
        self.roads.push(road);
    }

    /// Clone the vec
    pub fn get_buildings(&self) -> Vec<Building> {
        self.buildings.iter().map(|b| b.clone()).collect()
    }

    pub fn get_buildings_mut(&mut self) -> Vec<&mut Building> {
        self.buildings.iter_mut().collect()
    }

    pub fn get_buildings_district_mut(&mut self, district_id: usize) -> Vec<&mut Building> {
        self.buildings.iter_mut().filter(|b| b.district_id == district_id).collect()
    }

    /// Clone the vec
    pub fn get_roads(&self) -> Vec<Road> {
        self.roads.iter().map(|r| r.clone()).collect()
    }
}

fn deserialize_b64<'de, D>(deserializer: D) -> Result<LayoutId, D::Error>
where
    D: de::Deserializer<'de>
{
    let s: &str = de::Deserialize::deserialize(deserializer)?;
    let res = BASE64_STANDARD.decode(s);

    if let Err(_) = res {
        return Err(Error::custom("Invalid base64"))
    }

    let mut out = [0u8; LAYOUT_ID_LENGTH];

    for (i, x) in res.unwrap().iter().enumerate() {
        if i > LAYOUT_ID_LENGTH - 1 {
            break
        }

        out[i] = x.clone()
    }

    Ok(LayoutId::new(out))
}