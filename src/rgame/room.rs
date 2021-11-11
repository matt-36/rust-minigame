use crate::{
    constants::*,
    traits::render::{Render, RenderType},
};
use rand::{self, Rng};
use sdl2::{
    pixels::Color,
    rect::Rect,
    render::{Canvas, WindowCanvas},
};
use std::collections::HashMap;

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct RoomIndex(i32, i32);
impl From<(i32, i32)> for RoomIndex {
    fn from((a, b): (i32, i32)) -> Self {
        Self(a, b)
    }
}
impl From<sdl2::rect::Point> for RoomIndex {
    fn from(point: sdl2::rect::Point) -> Self {
        Self(
            (point.x as f32 / GAME_SIZE_X as f32).floor() as i32,
            (point.y as f32 / GAME_SIZE_Y as f32).floor() as i32,
        )
    }
}

#[derive(Default)]
pub struct RoomManager {
    pub rooms: HashMap<RoomIndex, Room>,
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
    pub fn get_room_rects_offset<T: Clone>(&mut self, index: T) -> Vec<Rect>
    where
        RoomIndex: From<T>,
    {
        let mut rects = self
            .get_room(index.clone())
            .get_rects((false, false, false, false));
        let index: RoomIndex = index.into();
        rects.iter_mut().for_each(|rect| {
            rect.offset(index.0 * GAME_SIZE_X as i32, index.1 * GAME_SIZE_Y as i32);
        });
        rects
    }
    fn generate_room() -> Room {
        let test = Room {
            r#type: match rand::thread_rng().gen_range(0..100) {
                0..=20 => RoomType::rand_chest_room(),
                _ => RoomType::Empty,
            },
        };
        println!("{:?}", test.r#type);
        test
    }
}

pub struct Room {
    r#type: RoomType,
}

#[derive(Debug)]
pub enum RoomType {
    Chest(Vec<Rect>),
    Empty,
}
impl RoomType {
    pub fn rand_chest_room() -> RoomType {
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
                // chests.push(Rect::new(x, y, width, height));
                chests
            }
            RoomType::Empty => vec![],
            _ => vec![],
        }
    }
}

impl Room {
    pub fn get_rects(&self, sides: (bool, bool, bool, bool)) -> Vec<Rect> {
        let mut rects = vec![
            *WALL_TLT, *WALL_TLL, *WALL_BLL, *WALL_BLB, *WALL_BRB, *WALL_BRR, *WALL_TRR, *WALL_TRT,
        ];
        macro_rules! add_door {
            ($side:tt $constant:expr) => {
                if sides.$side {
                    rects.push($constant) // no errors
                }
            };
        }
        add_door!(0 * DOOR_T);
        add_door!(1 * DOOR_L);
        add_door!(2 * DOOR_B);
        add_door!(3 * DOOR_R);
        self.r#type
            .get_extra_rects()
            .into_iter()
            .for_each(|r| rects.push(r));
        rects
    }
    pub fn get_rects_offset(
        &self,
        sides: (bool, bool, bool, bool),
        offset_rect: Rect,
    ) -> Vec<Rect> {
        self.get_rects(sides)
            .into_iter()
            .map(|rect| {
                let mut rect = rect.clone();
                rect.offset(offset_rect.x, offset_rect.y);
                rect
            })
            .collect::<Vec<_>>()
    }
}

impl From<RoomType> for Room {
    fn from(r#type: RoomType) -> Self {
        Self { r#type }
    }
}

impl Render for Room {
    fn render<T: sdl2::render::RenderTarget>(
        &self,
        canvas: &mut Canvas<T>,
        r#type: RenderType,
    ) -> Option<()> {
        match r#type {
            RenderType::Canvas { .. } => {
                canvas.set_draw_color(Color::RGB(127, 127, 127));
                canvas
                    .draw_rects(&self.get_rects((false, false, false, false)))
                    .unwrap();
            }
            RenderType::Minimap { options } => {}
            _ => {}
        }
        Some(())
    }
}
