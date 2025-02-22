use crate::engine::test::{BuildingDrawable, RoadDrawable};
use ansi_term::Color::{Blue, Green, Red};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Building {
    name: String,
    id: String,
    pos_x: i16,
    pos_y: i16,
    b_type: String,
    width: Option<u8>,
    height: Option<u8>,
    texture: Option<char>,
    content: Option<Vec<String>>,
}

impl Building {
    pub fn to_drawable(&self) -> BuildingDrawable {
        BuildingDrawable {
            xpos: self.pos_x,
            ypos: self.pos_y,
            color: if self.b_type == "empty_space" {
                Green
            } else {
                Blue
            },
            content: self.content.clone(),
            texture: self.texture,
            width: self.width,
            height: self.height,
            b_type: self.b_type.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
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

impl Road {
    pub fn to_drawable(&self) -> RoadDrawable {
        RoadDrawable {
            start_x: self.start_x,
            start_y: self.start_y,
            horizontal: self.horizontal,
            pavement: self.pavement,
            width: self.width,
            length: self.length,
            color: Red,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
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

    pub fn add_building(&mut self, building: Building) {
        self.buildings.push(building);
    }

    pub fn add_road(&mut self, road: Road) {
        self.roads.push(road);
    }

    pub fn get_buildings(&self) -> &Vec<Building> {
        &self.buildings
    }

    pub fn get_roads(&self) -> &Vec<Road> {
        &self.roads
    }

    pub fn get_road_drawables(&self) -> Vec<RoadDrawable> {
        self.roads.iter().map(|road| road.to_drawable()).collect()
    }

    pub fn get_building_drawables(&self) -> Vec<BuildingDrawable> {
        self.buildings
            .iter()
            .map(|building| building.to_drawable())
            .collect()
    }
}
