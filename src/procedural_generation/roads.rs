use rand::{rngs::ThreadRng, seq::SliceRandom, Rng};

use crate::engine::{
    drawable::Drawable,
    layout::{Building, Layout, Road, ROAD_WIDTH, TERMINAL_RATIO},
};

use super::{is_area_free, AreaPartition};

pub fn create_road_adj_to_building(
    building: &Building,
    layout: &mut Layout,
    rng: &mut ThreadRng,
    length_range: (u8, u8),
) -> bool {
    let width = building.width() as i16;
    let height = building.height() as i16;
    let pos_x = building.pos_x;
    let pos_y = building.pos_y;

    let mut candidates = Vec::new();

    for x in (pos_x + 2)..(pos_x + width - 2) {
        candidates.push((x, pos_y - 1, true));
    }

    for x in (pos_x + 2)..(pos_x + width - 2) {
        candidates.push((x, pos_y + height, true));
    }

    for y in (pos_y + 2)..(pos_y + height - 2) {
        candidates.push((pos_x - ROAD_WIDTH, y, false));
    }

    for y in (pos_y + 2)..(pos_y + height - 2) {
        candidates.push((pos_x + width, y, false));
    }

    candidates.shuffle(rng);

    for &(mut start_x, mut start_y, horizontal) in &candidates {
        let length = rng.random_range(length_range.0..length_range.1);
        let length = if horizontal {
            length * TERMINAL_RATIO
        } else {
            length
        };
        let road_width = if horizontal { 1 } else { TERMINAL_RATIO };

        if rng.random_bool(0.5) {
            if horizontal {
                start_x -= length as i16;
            } else {
                start_y -= length as i16;
            }
        }

        let area_width = if horizontal { length } else { road_width };
        let area_height = if horizontal { road_width } else { length };

        if is_area_free(
            start_x,
            start_y,
            area_width as u8,
            area_height as u8,
            layout,
            AreaPartition::All,
        ) {
            let road = Road::new((start_x, start_y), length, road_width, horizontal, '█');
            layout.add_road(road);
            return true;
        }
    }

    false
}

pub fn create_road_next_to_road(original: &Road, layout: &mut Layout, rng: &mut ThreadRng) -> bool {
    let horizontal = !original.is_horizontal();

    let mut candidates = original.get_area();

    candidates.shuffle(rng);

    for &(start_x, start_y) in &original.get_area() {
        let length = if horizontal { 10 * TERMINAL_RATIO } else { 10 };
        let road_width = if horizontal { 1 } else { TERMINAL_RATIO };
        let pavement = '█';

        let area_width = if horizontal { length } else { road_width };
        let area_height = if horizontal { road_width } else { length };

        if is_area_free(
            start_x,
            start_y,
            area_width as u8,
            area_height as u8,
            layout,
            AreaPartition::All,
        ) {
            let road = Road::new((start_x, start_y), length, road_width, horizontal, pavement);
            layout.add_road(road);
            return true;
        }
    }

    false
}
