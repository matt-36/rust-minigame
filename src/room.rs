use rand::{self, Rng};
use sdl2::{
    rect::Rect,
    render::{Canvas, WindowCanvas},
};
use std::collections::HashMap;

use crate::{GAME_SIZE_X, GAME_SIZE_Y};

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct RoomIndex(i32, i32);
impl From<(i32, i32)> for RoomIndex {
    fn from((a, b): (i32, i32)) -> Self {
        Self(a, b)
    }
}
impl From<sdl2::rect::Point> for RoomIndex {
    fn from(point: sdl2::rect::Point) -> Self {
        Self(point.x / GAME_SIZE_X as i32, point.y / GAME_SIZE_Y as i32)
    }
}

#[derive(Default)]
pub struct RoomManager {
    rooms: HashMap<RoomIndex, Room>,
}

impl RoomManager {
    pub fn get_room<T>(&mut self, index: T) -> &Room
    where
        RoomIndex: From<T>,
    {
        let index: RoomIndex = RoomIndex::from(index);
        self.rooms
            .entry(index.clone())
            .or_insert_with(|| RoomManager::generate_room());
        &self.rooms[&index]
    }
    fn generate_room() -> Room {
        Room {
            r#type: match rand::thread_rng().gen_range(0..100) {
                0..=20 => RoomType::new_chest(),
                _ => RoomType::Empty,
            },
        }
    }
}

pub struct Room {
    r#type: RoomType,
}

pub enum RoomType {
    Chest(Vec<Rect>),
    Empty,
}
impl crate::room::RoomType {
    pub fn new_chest() -> RoomType {
        let chests = vec![];
        //
        RoomType::Chest(chests)
    }
}

impl RoomType {
    fn get_extra_rects(&self) -> Vec<Rect> {
        match self {
            RoomType::Chest(chests) => {
                let mut chests = Vec::new();
                chests.push(Rect::new(x, y, width, height));
            RoomType::Empty => vec![],
        }
    }
}

const DOOR_WIDTH: u32 = 20;
const DOOR_HEIGHT: u32 = 200;

macro_rules! rect {
    ($x:expr, $y:expr, $w:expr, $h:expr) => {
        Rect::new($x, $y, $w, $h)
    };
}

lazy_static::lazy_static! {
    static ref DOOR_LEFT: Rect = rect!(0, ((GAME_SIZE_Y - DOOR_HEIGHT) / 2) as i32, DOOR_WIDTH, DOOR_HEIGHT);
    static ref DOOR_RIGHT: Rect = rect!((GAME_SIZE_X - DOOR_WIDTH) as i32, ((GAME_SIZE_Y - DOOR_HEIGHT) / 2) as i32, DOOR_WIDTH, DOOR_HEIGHT);
    static ref DOOR_TOP: Rect = rect!(((GAME_SIZE_X - DOOR_HEIGHT) / 2) as i32, 0, DOOR_HEIGHT, DOOR_WIDTH);
    static ref DOOR_BOTTOM: Rect = rect!(((GAME_SIZE_X - DOOR_HEIGHT) / 2) as i32, (GAME_SIZE_X - DOOR_WIDTH) as i32, DOOR_HEIGHT, DOOR_WIDTH,
    );
    static ref WALLS: Vec<Rect> = vec![

    ]
}

impl Room {
    fn get_rects(&self, sides: (bool, bool, bool, bool)) -> Vec<Rect> {
        let mut rects = vec![];
        macro_rules! add_door {
            ($side:tt $constant:expr) => {
                if sides.$side {
                    rects.push($constant)
                }
            };
        };
        add_door!(0 * DOOR_TOP);
        add_door!(1 * DOOR_LEFT);
        add_door!(2 * DOOR_BOTTOM);
        add_door!(3 * DOOR_RIGHT);
        self.r#type.get_extra_rects()
    }
}

impl From<RoomType> for Room {
    fn from(r#type: RoomType) -> Self {
        Self { r#type }
    }
}
