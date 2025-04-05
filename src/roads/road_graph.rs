use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::ops::BitXor;
use crate::engine::drawable::Drawable;
use crate::engine::layout::{Building, Layout, LayoutId, Road, LAYOUT_ID_LENGTH};

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
    pub fn id(&self) -> &'a LayoutId {
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

struct Pair<'a>
{
    road_a: &'a LayoutId,
    road_b: &'a LayoutId
}

pub type Edge<'a> = Pair<'a>;

impl Pair<'_>
{
    fn new<'a>(a: &'a LayoutId, b: &'a LayoutId) -> Pair<'a> {
        Pair {
            road_a: a,
            road_b: b
        }
    }

    fn has(&self, value: &LayoutId) -> bool {
        value == self.road_a || value == self.road_b
    }

    fn other(&self, value: &LayoutId) -> &LayoutId {
        if value == self.road_a {
            self.road_b
        } else {
            self.road_a
        }
    }
}

impl Debug for Edge<'_>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} <=> {:?}", self.road_a, self.road_b)
    }
}

impl PartialEq for Edge<'_>
{
    fn eq(&self, other: &Self) -> bool {
        if self.road_a == other.road_a {
            self.road_b == other.road_b
        } else if self.road_b == other.road_a {
            self.road_a == other.road_b
        } else {
            false
        }
    }
}

impl Eq for Edge<'_> {}

impl Hash for Edge<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut merged = [0u8; LAYOUT_ID_LENGTH];
        for (i, (a, b)) in self.road_a.into_iter().zip(self.road_b.into_iter()).enumerate() {
            merged[i] = a.bitxor(b)
        }

        merged.hash(state)
    }
}

#[derive(Debug)]
pub struct Graph<'a> {
    nodes: HashMap<LayoutId, Node<'a>>,
    edges: HashSet<Edge<'a>>,
    building_connections: HashSet<Pair<'a>>
}

impl<'a> Graph<'a> {
    pub fn new(layout: &Layout) -> Graph {
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
            edges: edge_set,
            building_connections: HashSet::new()
        }
    }

    fn connected_to(&self, start: &LayoutId) -> HashSet<&LayoutId> {
        self.edges.iter().filter(|x| x.has(start)).map(|x| x.other(start)).collect::<HashSet<&LayoutId>>()
    }

    pub fn start_dfs(& mut self, layout: &'a Layout) {
        let mut connections = HashSet::new();
        for x in layout.buildings.iter().map(|x| &x.id) {
            let mut marks = HashSet::new();

            self.recursive_dfs_to_target(&mut marks, x);

            for y in layout.buildings.iter().map(|x| &x.id) {
                if x != y && marks.contains(y) {
                    connections.insert(Pair::new(x, y));
                }
            }
        }

        self.building_connections = connections
    }

    fn recursive_dfs_to_target(&'a self, mark: &mut HashSet<&'a LayoutId>, current: &'a LayoutId) {
        mark.insert(current);
        for x in self.connected_to(current) {
            if mark.contains(x) {
                continue
            }

            self.recursive_dfs_to_target(mark, x)
        }
    }

    pub fn are_connected(&self, building_id_a: &LayoutId, building_id_b: &LayoutId) -> bool {
        self.building_connections.contains(&Pair::new(building_id_a, building_id_b))
    }
}