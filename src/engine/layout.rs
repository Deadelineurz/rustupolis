use std::fs::read_to_string;
use std::ptr::null;
use ansi_term::Color::{Blue, Green, Red};
use serde::{Serialize, Deserialize};
use serde_json::Value;
use crate::engine::test::{BuildingDrawable, RoadDrawable};


#[derive(Serialize, Deserialize, Debug)]
pub struct Building {
    name : String,
    id: String,
    pos_x : i16,
    pos_y : i16,
    b_type : String,
    width : Option<u8>,
    height : Option<u8>,
    texture : Option<char>,
    content: Option<Vec<String>>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Road {
    name : String,
    id : String,
    start_x : i16,
    start_y : i16,
    horizontal : bool,
    width : u8,
    length : u8,
    pavement : char
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Layout {
    pub buildings : Vec<Building>,
    pub roads : Vec<Road>
}

pub fn read_layout() -> Layout {
    let layout = include_str!("../initial_data/layout.json");

    let layout_obj : Layout = serde_json::from_str(layout).expect("TODO: panic message");

    layout_obj
}

pub fn layout_get_buildings(layout : &Layout) -> &Vec<Building> {
    &layout.buildings
}

pub fn layout_get_roads(layout : Layout) -> Vec<Road> {
    layout.roads
}

pub fn drawables_from_buildings(buildings: Vec<Building>) -> Vec<BuildingDrawable> {
    let mut drawables = vec![];

    for building in buildings {
        let drawable = BuildingDrawable { xpos: building.pos_x, ypos: building.pos_y, color: if building.b_type == "empty_space" {Green} else { Blue }, content: building.content, texture : building.texture, width : building.width, height: building.height, b_type: building.b_type };
        drawables.push(drawable)
    }

    drawables
}

pub fn drawables_from_roads(roads: Vec<Road>) -> Vec<RoadDrawable> {
    let mut drawables = vec![];

    for road in roads {
        let drawable = RoadDrawable { start_x : road.start_x, start_y : road.start_y, horizontal: road.horizontal, pavement: road.pavement, width: road.width, length: road.length, color: Red };
        drawables.push(drawable)
    }

    drawables
}