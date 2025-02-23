use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{BufReader, Write};
use std::ptr::write;
use crate::engine::layout::Layout;

pub fn load_savegame() -> Layout {
    let layout = fs::read_to_string("savegame/save_layout.json");
    let layout_obj: Layout = serde_json::from_str(&layout.unwrap()).unwrap();
    layout_obj
}

pub fn save_layout(layout : &Layout){
    let layout_str = serde_json::to_string(&layout);
    let mut layout_save = File::create("savegame/save_layout.json");

    layout_save.unwrap().write_all(layout_str.unwrap().as_bytes());

    return;
}