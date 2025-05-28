use super::core::Engine;
use super::drawable::DrawableType;
use super::{drawable::Drawable, keybinds::Clickable};
use crate::population::Population;
use crate::roads::road_graph::{Graph, Rect};
use crate::threads::engine_loop::Selection;
use crate::utils::intersections::intersection;
use crate::{lock_read, lock_unlock, population::people::BasePeopleInfo, ui::colors::*};
use base64::prelude::BASE64_STANDARD;
use base64::Engine as b64Engine;
use log::{debug, trace};
use rand::{rng, Fill};
use serde::de::Error;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::array::IntoIter;
use std::cmp::PartialEq;
use std::fmt::Display;
use std::fmt::{Debug, Formatter};
use std::slice::Iter;

pub const LAYOUT_ID_LENGTH: usize = 12;
pub const TERMINAL_RATIO: u8 = 2;
pub const ROAD_WIDTH: i16 = 2;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct LayoutId {
    value: [u8; LAYOUT_ID_LENGTH],
}

impl LayoutId {
    pub fn new(value: [u8; LAYOUT_ID_LENGTH]) -> Self {
        LayoutId { value }
    }

    pub fn iter(&self) -> Iter<'_, u8> {
        self.value.iter()
    }

    pub fn random() -> Self {
        let mut rng = rng();

        let mut x = [0u8; LAYOUT_ID_LENGTH];
        x.fill(&mut rng);
        LayoutId { value: x }
    }
}

impl Serialize for LayoutId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&BASE64_STANDARD.encode(self.value))
    }
}

impl<'de> Deserialize<'de> for LayoutId {
    fn deserialize<D>(deserializer: D) -> Result<LayoutId, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: &str = de::Deserialize::deserialize(deserializer)?;
        let res = BASE64_STANDARD.decode(s);

        if let Err(_) = res {
            return Err(Error::custom("Invalid base64"));
        }

        let mut out = [0u8; LAYOUT_ID_LENGTH];

        for (i, x) in res.unwrap().iter().enumerate() {
            if i > LAYOUT_ID_LENGTH - 1 {
                break;
            }

            out[i] = x.clone()
        }

        Ok(LayoutId::new(out))
    }
}

impl IntoIterator for &LayoutId {
    type Item = u8;
    type IntoIter = IntoIter<u8, LAYOUT_ID_LENGTH>;

    fn into_iter(self) -> Self::IntoIter {
        self.value.into_iter()
    }
}

impl Default for LayoutId {
    fn default() -> Self {
        LayoutId {
            value: [0u8; LAYOUT_ID_LENGTH],
        }
    }
}

impl Debug for LayoutId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", BASE64_STANDARD.encode(self.value))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BuildingType {
    Custom,
    Uniform,
    EmptySpace,
}

impl Display for BuildingType {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

// ----- BUILDINGS -----

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Building {
    name: String,
    pub id: LayoutId,
    district_id: usize,
    pub pos_x: i16,
    pub pos_y: i16,
    b_type: BuildingType,
    width: Option<u8>,
    height: Option<u8>,
    texture: Option<char>,
    content: Option<Vec<String>>,
}

impl Building {
    pub fn get_num_people_in_building(&self, population: &Population) -> usize {
        if let Some(pop) = population.get_district(self.district_id) {
            pop.peoples
                .iter()
                .filter(|people| {
                    if let Some(uuid) = people.get_building_uuid() {
                        *uuid == self.id
                    } else {
                        false
                    }
                })
                .count()
        } else {
            0
        }
    }

    pub fn is_overcrowded(&self, population: &Population) -> bool {
        self.get_num_people_in_building(population) > self.get_area().len() * 3
    }

    pub fn get_building_uuid(&self) -> LayoutId {
        self.id.clone()
    }

    pub fn get_district_id(&self) -> usize {
        self.district_id
    }

    fn width(&self) -> u8 {
        if self.b_type == BuildingType::Custom {
            let mut n: u8 = 0;
            let lines = self.content.as_ref().unwrap();
            for line in lines {
                if n == 0 {
                    n = line.chars().count() as u8
                } else if (line.chars().count() as u8) != n {
                    panic!("BuildingDrawable: Invalid line width, all lines need to have the same width")
                }
            }
            n
        } else {
            if let Some(width) = self.width {
                width
            } else {
                0
            }
        }
    }

    fn height(&self) -> u8 {
        if self.b_type == BuildingType::Custom {
            return self.content.as_ref().unwrap().len() as u8;
        }
        self.height.unwrap()
    }

    pub fn get_building_type(&self) -> BuildingType {
        self.b_type.clone()
    }

    pub fn get_content(&self) -> Option<Vec<String>> {
        self.content.clone()
    }

    pub fn new_at(x: i16, y: i16, width: u8, height: u8) -> Self {
        Building {
            name: "Roadside Building".to_string(),
            id: LayoutId::random(),
            district_id: 0,
            pos_x: x,
            pos_y: y,
            b_type: BuildingType::Uniform,
            width: Some(width),
            height: Some(height),
            texture: Some('█'),
            content: None,
        }
    }

    pub fn get_area(&self) -> Vec<(i16, i16)> {
        let mut tiles = Vec::new();

        if let Some(content) = &self.content {
            for (dy, line) in content.iter().enumerate() {
                for (dx, ch) in line.chars().enumerate() {
                    if ch != ' ' {
                        let x = self.pos_x + dx as i16;
                        let y = self.pos_y + dy as i16;
                        tiles.push((x, y));
                    }
                }
            }
        } else {
            for h in 0..self.height() {
                for w in 0..self.width() {
                    let x = self.pos_x + w as i16;
                    let y = self.pos_y + h as i16;
                    tiles.push((x, y));
                }
            }
        }

        tiles
    }
}

impl Clickable for Building {
    fn infos(&self, engine: &Engine) -> Option<Vec<String>> {
        let x = Some(vec![
            String::from(format!("Name: {}", self.name)),
            String::from(format!("Position: {}, {}", self.pos_x, self.pos_y)),
            String::from(format!(
                "Population: {}",
                self.get_num_people_in_building(&engine.population)
            )),
            String::from(format!(
                "Population: {}",
                self.get_num_people_in_building(&engine.population)
            )),
            String::from(" ".to_string()), // act as a newline
        ]);
        x
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
        self.width()
    }

    fn height(&self) -> u8 {
        self.height()
    }

    fn shape(&self) -> String {
        if self.b_type == BuildingType::Custom {
            return self.content.as_ref().unwrap().join("\n");
        } else if self.b_type == BuildingType::EmptySpace {
            return if self.width.unwrap() == 1 && self.height.unwrap() == 1 {
                "▢\n".parse().unwrap()
            } else if self.height.unwrap() == 1 {
                let mut str: String = "╞".to_string();
                str.push_str(&*"═".repeat(self.width.unwrap() as usize - 2));
                str.push_str("╡\n");
                str
            } else if self.width.unwrap() == 1 {
                let mut str: String = "╦\n".to_string();
                str.push_str(&*"║\n".repeat(self.height.unwrap() as usize - 2));
                str.push_str("╩\n");
                str
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
                str
            };
        }

        let mut str: String = "".to_string();
        for i in 0..self.height.unwrap() {
            if i == 0 {
                // Just to test
                str += &*(self
                    .texture
                    .unwrap()
                    .to_string()
                    .repeat(self.width.unwrap() as usize));
            } else {
                if i == 0 {
                    // Just to test
                    str += &*(self
                        .texture
                        .unwrap()
                        .to_string()
                        .repeat(self.width.unwrap() as usize));
                } else {
                    str += &*(self
                        .texture
                        .unwrap()
                        .to_string()
                        .repeat(self.width.unwrap() as usize));
                }

                str += "\n";
            }
        }
        str
    }

    fn color(&self, population: &Population) -> ansi_term::Color {
        match &self.b_type {
            s if s == &BuildingType::EmptySpace => A_SAND_COLOR,
            _ => {
                if self.get_num_people_in_building(population) > 200 {
                    A_RUST_COLOR_1
                } else if self.get_num_people_in_building(population) > 150 {
                    A_RUST_COLOR_2
                } else if self.get_num_people_in_building(population) > 100 {
                    A_LIGHT_COLOR
                } else if self.get_num_people_in_building(population) > 20 {
                    A_SAND_COLOR
                } else {
                    A_DARKEST_COLOR
                }
            }
        }
    }

    fn id(&self) -> LayoutId {
        self.id
    }

    fn d_type(&self) -> DrawableType {
        if self.b_type == BuildingType::EmptySpace {
            DrawableType::BuildingEmpty
        } else {
            DrawableType::Building
        }
    }
}

// ----- ROADS -----

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Road {
    pub name: String,
    pub id: LayoutId,
    pub start_x: i16,
    pub start_y: i16,

    pub(crate) horizontal: bool,
    pub(crate) width: u8,
    pub(crate) length: u8,
    pub(crate) pavement: char,
}

impl Road {
    pub fn new(
        start: (i16, i16),
        length: u8,
        width: u8,
        is_horizontal: bool,
        pavement: char,
    ) -> Self {
        Road {
            name: "New Road".to_string(),
            id: LayoutId::random(),
            start_x: start.0,
            start_y: start.1,
            horizontal: is_horizontal,
            width: width,
            length: length,
            pavement: pavement,
        }
    }

    pub fn get_area(&self) -> Vec<(i16, i16)> {
        let mut tiles = Vec::new();
        for i in 0..self.length {
            for w in 0..self.width {
                let (x, y) = if self.horizontal {
                    (self.start_x + i as i16, self.start_y + w as i16)
                } else {
                    (self.start_x + w as i16, self.start_y + i as i16)
                };
                tiles.push((x, y));
            }
        }
        tiles
    }

    pub fn extend_down(&mut self, amount: u8) -> (i16, i16) {
        self.length += if self.horizontal { amount * 2 } else { amount };

        if self.horizontal {
            (self.start_x + self.length as i16 - 1, self.start_y)
        } else {
            (self.start_x, self.start_y + self.length as i16 - 1)
        }
    }

    pub fn extend_up(&mut self, amount: u8) -> (i16, i16) {
        let amount = if self.horizontal { amount * 2 } else { amount };

        if self.horizontal {
            self.start_x -= amount as i16;
        } else {
            self.start_y -= amount as i16;
        }

        self.length += amount;

        (self.start_x, self.start_y)
    }

    pub fn length(&self) -> u8 {
        self.length
    }

    pub fn is_horizontal(&self) -> bool {
        self.horizontal
    }
}

impl Clickable for Road {
    fn infos(&self, _engine: &Engine) -> Option<Vec<String>> {
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

    fn color(&self, pop: &Population) -> ansi_term::Color {
        if self.pavement == '░' {
            A_LIGHT_COLOR
        } else {
            A_GREY_COLOR
        }
    }

    fn id(&self) -> LayoutId {
        self.id
    }
    fn d_type(&self) -> DrawableType {
        DrawableType::Road
    }
}

// ----- LAYOUT -----

#[derive(Serialize, Deserialize, Debug)]
pub struct Layout<'a> {
    pub buildings: Vec<Building>,
    pub roads: Vec<Road>,
    #[serde(skip)]
    pub selections: Vec<Selection>,
    #[serde(skip)]
    pub graph: Option<Graph<'a>>,
}

impl Layout<'_> {
    pub fn update_graph(&mut self) {
        unsafe { self.graph = Some(Graph::new((&raw const *self).as_ref().unwrap())) }
    }

    pub fn load_default_layout() -> Self {
        let layout = include_str!("../initial_data/layout.json");

        let layout_obj: Layout = serde_json::from_str(layout).unwrap();

        layout_obj
    }

    pub fn load_default_layout2() -> Self {
        let layout = include_str!("../initial_data/layout2.json");

        let layout_obj: Layout = serde_json::from_str(layout).unwrap();

        layout_obj
    }

    pub fn load_core_layout() -> Self {
        let layout = include_str!("../initial_data/starting_core.json");

        let layout_obj: Layout = serde_json::from_str(layout).unwrap();

        layout_obj
    }

    pub fn load_empty_layout() -> Self {
        let layout = include_str!("../initial_data/starting_empty.json");

        let layout_obj: Layout = serde_json::from_str(layout).unwrap();

        layout_obj
    }

    pub fn add_building(&mut self, building: Building) {
        self.buildings.push(building);
        self.update_graph();
    }

    pub fn add_road(&mut self, road: Road) {
        self.roads.push(road);
        self.update_graph()
    }

    /// Clone the vec
    pub fn get_buildings(&self) -> Vec<Building> {
        self.buildings.iter().map(|b| b.clone()).collect()
    }

    pub fn get_buildings_mut(&mut self) -> Vec<&mut Building> {
        self.buildings.iter_mut().collect()
    }

    pub fn get_buildings_district_mut(&mut self, district_id: usize) -> Vec<&mut Building> {
        self.buildings
            .iter_mut()
            .filter(|b| b.district_id == district_id)
            .collect()
    }

    /// Clone the vec
    pub fn get_roads(&self) -> Vec<Road> {
        self.roads.iter().map(|r| r.clone()).collect()
    }

    pub fn get_selections(&self) -> Vec<Selection> {
        self.selections.iter().map(|r| r.clone()).collect()
    }

    pub fn get_building_for_coordinates(
        &self,
        x: i16,
        y: i16,
        filter: BuildingType,
    ) -> Option<&Building> {
        for bldg in &(self.buildings) {
            if let Some(_hei) = bldg.height {
                //debug!("{:} {:} {:} {:} {:}", bldg.name, bldg.x(), (x + bldg.width.unwrap() as i16) ,bldg.y(), (y + hei as i16));
            }
        }
        let res = self.buildings.iter().find(|it| {
            it.b_type == filter
                && it.x() <= x
                && (it.x() + it.width() as i16) >= x
                && it.y() <= y
                && (it.y() + it.height() as i16) >= y
        });
        let res = self.buildings.iter().find(|it| {
            it.b_type == filter
                && it.x() <= x
                && (it.x() + it.width() as i16) >= x
                && it.y() <= y
                && (it.y() + it.height() as i16) >= y
        });
        res
    }

    pub fn add_building_from_coords(&mut self, x: i16, y: i16, width: u8, height: u8) {
        let new_bldg = Building {
            name: "Test12".to_string(),
            id: LayoutId::random(),
            pos_x: x,
            pos_y: y,
            district_id: 1,
            b_type: BuildingType::Uniform,
            width: Option::from(width),
            height: Option::from(height),
            texture: Some('█'),
            content: Some(vec![]),
        };
        self.buildings.push(new_bldg);
        self.update_graph();
    }

    pub fn add_road_from_coords(&mut self, x: i16, y: i16, width: u8, height: u8) {
        let new_road = Road {
            name: "Road12".to_string(),
            id: Default::default(),
            start_x: x,
            start_y: y,
            horizontal: if width >= height { true } else { false },
            width: if width >= height { 1 } else { 2 },
            length: if width >= height { width } else { height },
            pavement: '▓',
        };
        self.roads.push(new_road);
        self.update_graph();
    }

    pub fn replace_empty_building(&mut self, building_id: LayoutId) {
        let mut i = 0;
        let mut building: Option<&Building> = None;

        for bldg in &self.buildings {
            if bldg.id == building_id {
                building = Some(bldg);
                break;
                break;
            }

            i += 1
        }

        if !building.is_none() {
            let bldg = building.unwrap();
            //debug!("{:?}", bldg);

            let new_bldg = Building {
                name: "Test12".to_string(),
                id: LayoutId::random(),
                pos_x: bldg.pos_x,
                pos_y: bldg.pos_y,
                district_id: bldg.district_id,
                b_type: BuildingType::Uniform,
                width: Option::from(bldg.width()),
                height: Option::from(bldg.height()),
                texture: Some('▓'),
                content: Some(vec![]),
            };
            self.buildings.push(new_bldg);
            self.buildings.remove(i);
        }

        self.update_graph()
    }

    pub fn calculate_path(&mut self, start: &LayoutId, goal: &LayoutId) -> Vec<Option<Rect>> {
        let mut wonder_graph = Graph::new(&self);
        wonder_graph.start_dfs(&self);
        let path = wonder_graph.find_path_bfs(start, goal);

        let mut last_drawable: Option<Box<dyn Drawable>> = None;
        let mut intersections = vec![];
        if let Some(chemin) = path {
            //println!("Itinéraire trouvé:");
            for id in chemin {
                for bldg in self.buildings.iter().filter(|x| x.id == id) {
                    //println!("{:?}", bldg.name);
                    /*if last_drawable.is_some(){
                        println!("{:?}", intersection(&*last_drawable.unwrap(), &*Box::new(bldg.clone())))
                    }*/
                    if last_drawable.is_some() {
                        intersections.push(intersection(
                            &*last_drawable.unwrap(),
                            &*Box::new(bldg.clone()),
                        ));
                    }
                    last_drawable = Some(Box::new(bldg.clone()))
                }
                for rdg in self.roads.iter().filter(|x| x.id == id) {
                    //println!("{:?}", rdg.name);

                    /*if last_drawable.is_some(){
                        println!("{:?}", intersection(&*last_drawable.unwrap(), &*Box::new(rdg.clone())))
                    }*/
                    if last_drawable.is_some() {
                        intersections.push(intersection(
                            &*last_drawable.unwrap(),
                            &*Box::new(rdg.clone()),
                        ));
                    }
                    last_drawable = Some(Box::new(rdg.clone()))
                }
            }
        } else {
            return intersections;
        }
        intersections
    }
}
