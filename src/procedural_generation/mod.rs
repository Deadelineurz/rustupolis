pub mod buildings;
pub mod roads;

use crate::{engine::{core::LockableEngine, drawable::Drawable, layout::Layout}, lock_read, lock_write, population::{self, Population}};

use buildings::create_building_next_to_road;
use rand::{rngs::*, seq::*, Rng};
use roads::*;

pub fn generate_next_step<'a>(engine: &LockableEngine, rng: &mut ThreadRng) {

    lock_write!(engine |> w);
    w.layout.roads.shuffle(rng);
    w.layout.buildings.shuffle(rng);

    let graph = w.layout.graph.as_ref();
    let population = &w.population;

    let binding = w.layout.buildings.clone();
    let full_buildings: Vec<_> = binding.iter().filter(|&b| b.is_overcrowded(population) || rng.random_bool(0.2) ).collect();
    // graph.connected_to(&building.get_building_uuid()).len() == 0

    for building in full_buildings.iter() {

        create_road_adj_to_building(&building, &mut w.layout, rng, (6, 10));

        let mut road = w.layout.roads.iter().choose(rng).unwrap().clone();

        create_extension_road_toward_building(&mut road, &mut w.layout, 20, '░');

        create_road_next_to_road(&road, &mut w.layout, rng);

        create_building_next_to_road(&road, &mut w.layout, rng);
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
