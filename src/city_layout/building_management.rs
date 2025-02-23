use crate::city_layout::save_manage::save_layout;
use crate::engine::layout::{Building, Layout};
use base64::prelude::*;


pub fn add_bulding(building: Building, mut layout: Layout, save: bool) -> Layout {
    let _ = layout.buildings.push(building);
    dbg!(&layout.buildings);

    if save {
        save_layout(&layout);
    }
    layout
}

pub fn create_uniform_building(pos_x: i16, pos_y: i16, width : u8, height: u8, texture : char, name: String, layout: Layout) -> Building {
    let id = BASE64_STANDARD.encode(name.clone());
    let building = Building{name, id, district_id: 0, pos_x, pos_y, b_type: "uniform".to_string(), width: Option::from(width), height: Option::from(height), texture: Option::from(texture), content: None};
    building
}