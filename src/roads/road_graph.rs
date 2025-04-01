use crate::engine::layout::{Building, Layout, Road};

#[derive(Debug)]
enum AttachedBuilding<'a> {
    OneBuilding(&'a Building),
    TwoBuildings(&'a Building, &'a Building)
}

#[derive(Debug)]
pub struct RoadNode<'a> {
    buildings: AttachedBuilding<'a>,
    position: (i16, i16)
}

#[derive(Debug)]
pub struct Edge {
    side_a: (i16, i16),
    side_b: (i16, i16)
}

pub struct RoadGraph<'a> {
    nodes: Vec<RoadNode<'a>>,
    edges: Vec<Edge>
}

fn touch_point(road_a: &Road, road_b: &Road) -> Option<(i16, i16)> {
    unimplemented!()
}

impl RoadNode<'_> {
    fn new(layout: &Layout) -> Self {
        for road_a in layout.roads.iter() {
            for road_b in layout.roads.iter() {
                if road_a.id == road_b.id {
                    continue
                }

                // touch_point()
            }
        }

        unimplemented!()
    }
}