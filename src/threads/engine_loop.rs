use crate::engine::core::{Engine, LockableEngine};
use crate::engine::layout::{BuildingType, LayoutId};
use crate::utils::interruptible_sleep::InterruptibleSleep;
use crate::{lock_read, lock_unlock, lock_write, return_on_cancel};
use std::env;
use std::sync::mpsc::Receiver;
use std::sync::Arc;
use std::thread::{Scope, ScopedJoinHandle};
use std::time::Duration;
use ansi_term::{Color, Colour};
use log::{debug, trace};
use serde::Deserialize;
use termion::cursor::Right;
use termion::event::Key::Left;
use termion::event::{Key, MouseButton};
use crate::engine::drawable::{Drawable, DrawableType};
use crate::engine::drawable::DrawableType::{Building, BuildingEmpty, Road};
use crate::engine::keybinds::Clickable;
use crate::population::Population;
use crate::threads::engine_loop::SelectionType::Void;
use crate::ui::colors::{A_RUST_COLOR_1, A_SAND_COLOR, A_UI_WHITE_DARK_COLOR};

#[derive(Copy, Deserialize, Clone, Debug, PartialEq)]
pub enum SelectionType {
    Building,
    Road,
    Void
}
#[derive(Copy, Deserialize, Clone, Debug)]
pub struct Selection {
    pub(crate) pos_x: i16,
    pub(crate) pos_y: i16,
    pub(crate) width: u8,
    pub(crate) height: u8,
    pub(crate) sel_type: SelectionType
}

impl Drawable for Selection {
    fn x(&self) -> i16 { self.pos_x }
    fn y(&self) -> i16 { self.pos_y }
    fn width(&self) -> u8 { self.width }
    fn height(&self) -> u8 { self.height }
    fn shape(&self) -> String {
        let mut str: String = "".to_string();
        for i in 0..self.height {

            str += &*(
                ("␥").to_string()
                .repeat(self.width as usize));
            str += "\n";
        }
        str
    }
    fn color(&self, pop: &Population) -> ansi_term::Color {A_UI_WHITE_DARK_COLOR}
    fn id(&self) -> LayoutId {LayoutId::random()}
    fn d_type(&self) -> DrawableType {DrawableType::Selection}
}
impl Clickable for Selection {
    fn infos(&self, engine: &Engine) -> Option<Vec<String>> {
        Some(vec![
            String::from("".to_string()),
        ])
    }
}

fn check_click_target(input : (i16,i16), engine: &LockableEngine) -> Option<DrawableType> {
    let mut dtype = Option::from(None);
    lock_read!(engine |> eng);
    let drwb = eng.get_drawable_for_coordinates(input.0, input.1);
    if drwb.is_some() {
        dtype = Option::from(drwb.unwrap().d_type());
    }
    lock_unlock!(eng);
    dtype
}

fn get_click_target_id(input : (i16,i16), engine: &LockableEngine) -> Option<LayoutId> {
    let mut id = Option::from(None);
    lock_read!(engine |> eng);
    let drwb = eng.get_drawable_for_coordinates(input.0, input.1);
    if drwb.is_some() {
        id = Option::from(drwb.unwrap().id());
    }
    lock_unlock!(eng);
    id
}

fn calculate_road_coords(start : (i16,i16), end: (i16,i16)) -> ((i16,i16),(i16,i16)) {
    let x1 = start.0;
    let y1 = start.1;
    let x2 = end.0;
    let y2 = end.1;
    debug!("calculate road coord {:?}", (start, end));
    let mut new_end_coords = (x2, y2);
    debug!("calculate end coord cleaned {:?}", ((x2-x1).abs(), (y2-y1).abs()));

    if (x2-x1).abs() >= (y2-y1).abs() {new_end_coords = (x2, y1)}
    else if (y2-y1).abs() > (x2-x1).abs() {new_end_coords = (x1, y2)}
    (start, new_end_coords)
}


pub fn engine_loop<'scope, 'env>(
    s: &'scope Scope<'scope, 'env>,
    engine: LockableEngine,
    stop_var: Arc<InterruptibleSleep>,
    click_receiver: Receiver<(i16, i16, (Option<MouseButton>, Option<Key>))>,
    key_receiver: Receiver<Key>
) -> ScopedJoinHandle<'scope, ()> {
    s.spawn(move || {
        let mut inputs = vec![];

        fn check_inputs(inputs: &mut (Vec<(i16, i16, (Option<MouseButton>, Option<Key>))>), engine: &LockableEngine) {
            let n = inputs.iter().count();

            if inputs[0].2.0.is_some() && inputs[0].2.0.unwrap() == MouseButton::Left && check_click_target((inputs[0].0, inputs[0].1), engine) == Option::from(BuildingEmpty) {
                let boolean = replace_building_from_coords(inputs[0].0, inputs[0].1, engine, BuildingType::EmptySpace);
                if boolean {
                    *inputs = vec![];
                }

                return;
            }
            if inputs[0].2.1.is_some() && inputs[0].2.1.unwrap() == Key::Esc {
                *inputs = vec![];
                lock_write!(engine |> eng);
                eng.layout.selections = vec![];
                eng.refresh_drawables();
                eng.refresh();
                lock_unlock!(eng);
                return;
            }
            if inputs.iter().count() >=2 {
                if inputs[0].2.0.is_some()  && inputs[1].2.0.is_some() { // 2 actions keybinds
                    if inputs[0].2.0.unwrap() == MouseButton::Right && inputs[1].2.0.unwrap() == MouseButton::Left {
                        // Cleaned the current selections
                        lock_write!(engine |> eng);
                        eng.layout.selections = vec![];
                        eng.refresh_drawables();
                        eng.refresh();
                        lock_unlock!(eng);

                        let left_click_target = check_click_target((inputs[1].0, inputs[1].1), engine);
                        if left_click_target == Option::from(None) {
                            let drwbl = Selection {
                                sel_type: Void,
                                pos_x: if inputs[1].0 > inputs[0].0 {inputs[0].0} else {inputs[1].0},
                                pos_y: if inputs[1].1 > inputs[0].1 {inputs[0].1} else {inputs[1].1},
                                width: if inputs[1].0 > inputs[0].0 {(inputs[1].0 - inputs[0].0) as u8} else { (inputs[0].0 - inputs[1].0) as u8 },
                                height: if inputs[1].1 > inputs[0].1 {(inputs[1].1 - inputs[0].1) as u8} else { (inputs[0].1 - inputs[1].1) as u8 }
                            };
                            lock_write!(engine |> eng);
                            eng.layout.selections.push(drwbl);
                            eng.refresh_drawables();
                            eng.refresh();
                            lock_unlock!(eng);
                        }
                        else if left_click_target == Option::from(Road) {

                            let (start,end) = calculate_road_coords((inputs[1].0, inputs[1].1),(inputs[0].0, inputs[0].1));
                            debug!("road clicked {:?}", (start, end));
                            let drwbl = Selection {
                                sel_type: SelectionType::Road,
                                pos_x: if start.0 > end.0 {end.0} else {start.0},
                                pos_y: if start.1 > end.1 {end.1} else {start.1},
                                width: if start.1 == end.1 {(start.0 - end.0).abs() as u8} else { 2 },
                                height: if start.0 == end.0 {(start.1 - end.1).abs() as u8} else { 1 }
                            };
                            lock_write!(engine |> eng);
                            eng.layout.selections.push(drwbl);
                            eng.refresh_drawables();
                            eng.refresh();
                            lock_unlock!(eng);
                        }
                    }

                    if inputs[0].2.0.unwrap() == MouseButton::Middle && inputs[1].2.0.unwrap() == MouseButton::Middle {
                        // GPS
                        // Cleaned the current selections
                        lock_write!(engine |> eng);

                        let mut gps_roads_index = vec![];
                        let mut i = 0;
                        eng.layout.roads.iter().for_each(|r| {if r.name.contains("GPS") {gps_roads_index.push(i);} i+=1;});

                        for index in gps_roads_index {
                            eng.layout.roads.remove(index);
                        }
                        eng.refresh_drawables();
                        eng.refresh();
                        lock_unlock!(eng);

                        let click_type_1 = check_click_target((inputs[0].0, inputs[0].1), engine);
                        let click_type_2 = check_click_target((inputs[1].0, inputs[1].1), engine);

                        let click_1 = get_click_target_id((inputs[0].0, inputs[0].1), engine);
                        let click_2 = get_click_target_id((inputs[1].0, inputs[1].1), engine);

                        println!("{:?}", inputs);

                        if click_type_1 == Option::from(Building) && click_type_2 == Option::from(Building) && click_1.is_some() && click_2.is_some() {
                            let mut to_highlight = vec![];
                            lock_write!(engine |> eng);

                            let intersections = eng.layout.calculate_path(&click_1.unwrap(), &click_2.unwrap());

                            for (i, window) in intersections.windows(2).enumerate() {
                                if let [Some(inter), Some(inter2)] = window {
                                    let hori = !(inter.x == inter2.x || (inter.x - inter2.x).abs() <= 3);
                                    //println!("GPS{:} horiz : {:}", i, hori);
                                    to_highlight.push(crate::engine::layout::Road {
                                        name: format!("GPS{}", i),
                                        id: LayoutId::random(),
                                        start_x: if inter.x > inter2.x {inter2.x} else { inter.x },
                                        start_y: if inter.y > inter2.y {inter2.y} else { inter.y },
                                        horizontal: if hori {true} else { false },
                                        width: if hori {inter.height -1 } else { inter.width },
                                        length: if hori {
                                            (inter2.x - inter.x).abs() as u8
                                        } else {
                                            (inter2.y - inter.y).abs() as u8
                                        },
                                        pavement: '░',
                                    });
                                }
                            }

                            to_highlight.iter().for_each(|x| eng.layout.add_road(x.clone()));
                            eng.refresh_drawables();
                            eng.refresh();
                            lock_unlock!(eng);
                        }
                        *inputs = vec![];
                    }
                }
                if inputs.len() >= 3 && inputs[0].2.1.is_some() && inputs[1].2.0.is_some()  && inputs[2].2.0.is_some() {
                    let mut sel = Option::from(None);
                    lock_write!(engine |> eng);
                    sel = Option::from((eng.layout.selections[0].clone()));
                    eng.layout.selections = vec![];
                    eng.refresh_drawables();
                    eng.refresh();
                    lock_unlock!(eng);
                    if sel.is_some() {
                        if inputs[0].2.1.unwrap() == Key::Char('\n') && inputs[1].2.0.unwrap() == MouseButton::Right && inputs[2].2.0.unwrap() == MouseButton::Left {
                            if sel.unwrap().sel_type == SelectionType::Void{
                                add_building_from_coords(
                                    if inputs[2].0 > inputs[1].0 {inputs[1].0} else {inputs[2].0},
                                    if inputs[2].1 > inputs[1].1 {inputs[1].1} else {inputs[2].1},
                                    if inputs[2].0 > inputs[1].0 {(inputs[2].0 - inputs[1].0) as u8} else { (inputs[1].0 - inputs[2].0) as u8 },
                                    if inputs[2].1 > inputs[1].1 {(inputs[2].1 - inputs[1].1) as u8} else { (inputs[1].1 - inputs[2].1) as u8 },
                                    engine) ;
                            }
                            else if sel.unwrap().sel_type == SelectionType::Road {
                                let (mut start_x, mut start_y, mut width, mut height) = (0,0,0,0);
                                let selection = sel.unwrap();
                                (start_x, start_y, width, height) = (selection.pos_x, selection.pos_y, selection.width, selection.height);
                                add_road_from_coords(start_x, start_y, width, height, engine);
                            }

                            *inputs = vec![];

                        }
                    }
                }
            }
        }
        fn replace_building_from_coords(x: i16, y: i16, engine: &LockableEngine, filter: BuildingType) -> bool{
            trace!("Replacing");
            lock_write!(engine |> engine_write);
            let to_delete = {
                let drwbl = engine_write.layout.get_building_for_coordinates(x, y, filter);
                if let Some(drwbl) = drwbl {
                    Option::from(drwbl.id)
                }
                else {
                    Option::None
                }
            };
            if let Some(to_del) = to_delete {
                engine_write.layout.replace_empty_building(to_del);
                engine_write.refresh();
                drop(engine_write);
                true
            }
            else {
                drop(engine_write);
                false
            }
        }

        pub fn add_building_from_coords(x: i16, y: i16, width: u8, height: u8, engine: &LockableEngine) {
            lock_write!(engine |> e);
            e.layout.add_building_from_coords(x, y, width, height);
            e.refresh();
        }

        pub fn add_road_from_coords(x: i16, y: i16, width: u8, height: u8, engine: &LockableEngine) {
            lock_write!(engine |> e);
            e.layout.add_road_from_coords(x, y, width, height);
            e.refresh();
        }

        for (x, y, click_type) in click_receiver {
            inputs.insert(0, (x,y, click_type));

            check_inputs(&mut inputs, &engine);
            //
            /*if click_type == MouseButton::Right {
                add_building_from_coords(x,y,10,10, &engine);
            }
            else {
                replace_building_from_coords(x,y, &engine, BuildingType::EmptySpace);
            }*/
        }
        /*for key in key_receiver {
            debug!("{:? }", key);
            if key == Key::Insert{

                check_inputs(&mut inputs, &engine);
            }
        }*/
    })
}