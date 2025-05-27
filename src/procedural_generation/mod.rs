pub mod buildings;
pub mod roads;

use crate::engine::{drawable::Drawable, layout::Layout};

use buildings::create_building_next_to_road;
use rand::{rngs::*, seq::*, Rng};
use roads::{create_road_adj_to_building, create_road_next_to_road};

pub fn generate_next_step<'a>(layout: &mut Layout, rng: &mut ThreadRng) {
    layout.roads.shuffle(rng);
    layout.buildings.shuffle(rng);

    let buildings: Vec<_> = layout.buildings.clone();
    // if graph.connected_to(&building.get_building_uuid()).len() == 0 {

    // }

    for building in buildings {
        create_road_adj_to_building(&building, layout, rng, (4, 10));

        let road = layout.roads.iter().choose(rng).unwrap().clone();

        create_road_next_to_road(&road, layout, rng);

        if rng.random_bool(0.4) {
            create_building_next_to_road(&road, layout, rng);
        }
    }
}

#[derive(PartialEq)]
pub enum AreaPartition {
    Building,
    Roads,
    All,
}

pub fn is_area_free(
    x: i16,
    y: i16,
    width: u8,
    height: u8,
    layout: &Layout,
    partition: AreaPartition,
) -> bool {
    let (x1, y1, x2, y2) = (x, y, x + width as i16, y + height as i16);

    if partition != AreaPartition::Roads {
        for building in &layout.buildings {
            let bx1 = building.pos_x;
            let by1 = building.pos_y;
            let bx2 = bx1 + building.width() as i16;
            let by2 = by1 + building.height() as i16;

            if x1 < bx2 && x2 > bx1 && y1 < by2 && y2 > by1 {
                return false;
            }
        }
    }

    if partition != AreaPartition::Building {
        for road in &layout.roads {
            let rx1 = road.start_x;
            let ry1 = road.start_y;

            let (rw, rh) = if road.is_horizontal() {
                (road.length() as i16, road.width() as i16)
            } else {
                (road.width() as i16, road.length() as i16)
            };

            let rx2 = rx1 + rw;
            let ry2 = ry1 + rh;

            if x1 < rx2 && x2 > rx1 && y1 < ry2 && y2 > ry1 {
                return false;
            }
        }
    }

    true
}
