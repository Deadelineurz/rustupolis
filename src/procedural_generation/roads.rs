use log::debug;
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
        candidates.push((x, pos_y + height - 1, true));
    }

    for y in (pos_y + 0)..(pos_y + height - 0) {
        candidates.push((pos_x - ROAD_WIDTH, y, false));
    }

    for y in (pos_y + 0)..(pos_y + height - 0) {
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
        let range = rng.random_range(6..10);
        let length = if horizontal {
            range * TERMINAL_RATIO
        } else {
            range
        };
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
            AreaPartition::Building,
        ) {
            let road = Road::new((start_x, start_y), length, road_width, horizontal, pavement);
            layout.add_road(road);
            return true;
        }
    }

    false
}

pub fn create_extension_road_toward_building(
    road: &Road,
    layout: &mut Layout,
    max_steps: i16,
    pavement: char,
) {
    let (start_x, start_y) = (road.start_x, road.start_y);
    let is_horizontal = road.is_horizontal();
    let (length, width) = (road.length() as i16, road.width() as i16);

    let (fixed_axis, variable_start) = if is_horizontal {
        (start_y, start_x)
    } else {
        (start_x, start_y)
    };

    for &dir in &[-1, 1] {
        let mut pos = if dir == -1 {
            variable_start - 1
        } else {
            variable_start + length
        };

        for steps in 0..max_steps {
            let (x, y) = if is_horizontal {
                (pos, fixed_axis)
            } else {
                (fixed_axis, pos)
            };

            let mut hit_building = false;
            for building in &layout.buildings {
                let bx = building.pos_x;
                let by = building.pos_y;
                let bw = building.width() as i16;
                let bh = building.height() as i16;

                if x >= bx && x < bx + bw && y >= by && y < by + bh {
                    hit_building = true;
                    break;
                }
            }

            if hit_building {
                let extension_length = steps as u8;
                if extension_length == 0 {
                    break;
                }

                let (new_x, new_y) = if is_horizontal {
                    if dir == -1 {
                        (variable_start - extension_length as i16, fixed_axis)
                    } else {
                        (variable_start + length, fixed_axis)
                    }
                } else {
                    if dir == -1 {
                        (fixed_axis, variable_start - extension_length as i16)
                    } else {
                        (fixed_axis, variable_start + length)
                    }
                };

                let new_road = Road::new(
                    (new_x, new_y),
                    extension_length,
                    if is_horizontal { 1 } else { TERMINAL_RATIO },
                    is_horizontal,
                    pavement,
                );

                layout.add_road(new_road);
                break;
            }

            pos += dir;
        }
    }
}

enum Orientation {
    Horizontal,
    Vertical,
}
