use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use crate::engine::drawable::Drawable;
use crate::engine::layout::{Building, Layout, LayoutId, Road};

struct Rect {
    x: i16,
    y: i16,
    width: u8,
    height: u8
}

impl Rect {
    fn grow_by_one(&self) -> Self {
        Rect {
            x: self.x - 1,
            y: self.y - 1,
            width: self.width + 2,
            height: self.height + 2
        }
    }

    fn overlap(&self, other: Rect) -> bool {
        if self.x > other.x + other.width as i16 {
            return false
        }

        if self.y > other.y + other.height as i16 {
            return false
        }

        if self.x + (self.width as i16) < other.x {
            return false
        }

        if self.y + (self.height as i16) < other.y {
            return false
        }

        true
    }
}

#[derive(Copy, Clone)]
enum Node<'a> {
    Building(&'a Building),
    Road(&'a Road)
}

impl Debug for Node<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Building(x) => {
                f.debug_struct("Building").field("id", &x.id).finish()
            }
            Node::Road(r) => {
                f.debug_struct("Road").field("id", &r.id).finish()
            }
        }
    }
}

impl<'a> Node<'a> {
    pub fn id(&self) -> &'a String {
        match self {
            Node::Building(b) => &b.id,
            Node::Road(r) => &r.id
        }
    }

    pub fn rect(&self) -> Rect {
        match self {
            Node::Road(r) => {
                Rect {
                    x: r.x(),
                    y: r.y(),
                    width: r.width(),
                    height: r.height()
                }
            }
            Node::Building(b) => {
                Rect {
                    x: b.x(),
                    y: b.y(),
                    width: b.width(),
                    height: b.height()
                }
            }
        }
    }
}

struct Edge<'a> {
    road_a: &'a LayoutId,
    road_b: &'a LayoutId
}

impl Debug for Edge<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} <=> {:?}", self.road_a, self.road_b)
    }
}

impl PartialEq for Edge<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.road_a == other.road_a && self.road_b == other.road_b
    }
}

impl Eq for Edge<'_> {

}

impl Hash for Edge<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut id = self.road_a.clone();
        id += self.road_b;
        id.hash(state);
    }
}

#[derive(Debug)]
pub struct Graph<'a> {
    nodes: HashMap<LayoutId, Node<'a>>,
    edges: HashSet<Edge<'a>>
}

impl Graph<'_> {
    pub fn new<'a>(layout: &'a Layout) -> Graph<'a> {
        let mut nodes: HashMap<LayoutId, Node> = HashMap::new();
        let mut edge_set: HashSet<Edge> = HashSet::new();

        for layout_member in &layout.roads {
            nodes.insert(layout_member.id.clone(), Node::Road(layout_member));
        }

        for layout_member in &layout.buildings {
            nodes.insert(layout_member.id.clone(), Node::Building(layout_member));
        }

        for road in &layout.roads {
            let road_node = nodes.get(&road.id).unwrap();
            let ctangle = road_node.rect().grow_by_one();

            for other_road in &layout.roads {
                let other_node = nodes.get(&other_road.id).unwrap();
                if road_node.id() == other_node.id() {
                    continue
                }

                let other_rect = other_node.rect();

                if ctangle.overlap(other_rect) {
                    edge_set.insert(Edge {
                        road_a: road_node.id(),
                        road_b: other_node.id()
                    });
                }
            }

            for other_building in &layout.buildings {
                let other_node = nodes.get(&other_building.id).unwrap();
                let other_rect = other_node.rect();

                if ctangle.overlap(other_rect) {
                    edge_set.insert(Edge {
                        road_a: road_node.id(),
                        road_b: other_node.id()
                    });
                }
            }
        }

        Graph {
            nodes: nodes.clone(),
            edges: edge_set
        }
    }
}