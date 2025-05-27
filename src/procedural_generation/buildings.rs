use rand::{rngs::ThreadRng, seq::SliceRandom, Rng};

use crate::engine::layout::{Building, Layout, Road, TERMINAL_RATIO};

use super::{is_area_free, AreaPartition};

pub fn create_building_next_to_road(road: &Road, layout: &mut Layout, rng: &mut ThreadRng) -> bool {
    let road_area = road.get_area();
    let mut candidates = Vec::new();

    for &(x, y) in &road_area {
        candidates.push((x + 1, y));
        candidates.push((x - 1, y));
        candidates.push((x, y + 1));
        candidates.push((x, y - 1));
    }

    candidates.shuffle(rng);

    for (mut bx, mut by) in candidates {
        let width: u8 = 5 * TERMINAL_RATIO;
        let height: u8 = 5;

        if rng.random_bool(0.5) {
            bx -= (width - 1) as i16
        }
        if rng.random_bool(0.5) {
            by -= (height - 1) as i16
        }

        if is_area_free(bx, by, width, height, layout, AreaPartition::All) {
            let building = Building::new_at(bx, by, width, height);

            layout.add_building(building);
            return true;
        }
    }
    false
}
