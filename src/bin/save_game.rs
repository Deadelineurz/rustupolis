use rustupolis::city_layout::save_manage::save_layout;
use rustupolis::engine::layout::Layout;

fn main() {
    let layout = include_str!("../initial_data/layout.json");

    let layout_obj: Layout = serde_json::from_str(layout).unwrap();

    save_layout(&layout_obj)
}