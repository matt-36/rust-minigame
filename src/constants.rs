pub use super::config::*;
use sdl2::rect::Rect;

macro_rules! rect {
    ($x:expr, $y:expr, $w:expr, $h:expr) => {
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    };
}
// #[deny(unused_doc_comments)]
/**
 * L = Left
 * R = Right
 * T = Top
 * B = Bottom
 *
 *        top left                          top right
 *                    top            top
 *                   $$$$$$$------$$$$$$$
 *             left  #     door top     #  right
 *                   #                  #
 *                   |                  |
 *         door left |                  | door right
 *                   |                  |
 *                   #                  #
 *             left  #    door bottom   #  right
 *                   $$$$$$$------$$$$$$$
 *                    bottom      bottom
 *     bottom left                          bottom right
 */

lazy_static::lazy_static! {
    pub static ref DOOR_L: Rect = rect!(0, ((GAME_SIZE_Y - DOOR_HEIGHT) / 2), DOOR_WIDTH, DOOR_HEIGHT);
    pub static ref DOOR_R: Rect = rect!((GAME_SIZE_X - DOOR_WIDTH), ((GAME_SIZE_Y - DOOR_HEIGHT) / 2) as i32, DOOR_WIDTH, DOOR_HEIGHT);
    pub static ref DOOR_T: Rect = rect!(((GAME_SIZE_X - DOOR_HEIGHT) / 2), 0, DOOR_HEIGHT, DOOR_WIDTH);
    pub static ref DOOR_B: Rect = rect!(((GAME_SIZE_X - DOOR_HEIGHT) / 2), (GAME_SIZE_X - DOOR_WIDTH) as i32, DOOR_HEIGHT, DOOR_WIDTH);
    pub static ref WALL_TLT: Rect = rect!(0, 0, DOOR_T.left(), WALL_WIDTH);
    pub static ref WALL_TLL: Rect = rect!(0, WALL_WIDTH, WALL_WIDTH, DOOR_L.top() as u32 - WALL_WIDTH);
    pub static ref WALL_BLL: Rect = rect!(0, DOOR_L.bottom(), WALL_WIDTH, GAME_SIZE_Y - DOOR_L.bottom() as u32 - WALL_WIDTH);
    pub static ref WALL_BLB: Rect = rect!(0, GAME_SIZE_Y - WALL_WIDTH, DOOR_B.left(), WALL_WIDTH);
    pub static ref WALL_BRB: Rect = rect!(DOOR_B.right(), GAME_SIZE_Y - WALL_WIDTH, GAME_SIZE_X - DOOR_B.right() as u32, WALL_WIDTH);
    pub static ref WALL_BRR: Rect = rect!(GAME_SIZE_X - WALL_WIDTH, DOOR_R.bottom(), WALL_WIDTH, GAME_SIZE_Y - DOOR_R.bottom() as u32 - WALL_WIDTH);
    pub static ref WALL_TRR: Rect = rect!(GAME_SIZE_X - WALL_WIDTH, WALL_WIDTH, WALL_WIDTH, DOOR_R.top() as u32 - WALL_WIDTH);
    pub static ref WALL_TRT: Rect = rect!(DOOR_T.right(), 0, GAME_SIZE_X - DOOR_T.right() as u32, WALL_WIDTH);
}
