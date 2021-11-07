use sdl2::{rect::Rect, render::{Texture, WindowCanvas}};

use crate::entity::Player;

#[derive(Clone, Copy)]
pub struct Colider {
    pub sprite: Rect,
    pub dest: Rect,
}

impl Colider {
    pub fn new(
        sprite: Rect,
        dest: Rect
    ) -> Self {
        Self {
            sprite,
            dest
        }
    }

    pub fn render(self, canvas: &mut WindowCanvas, texture: &Texture) {
        canvas.copy(texture, self.sprite, self.dest);
    }
}

pub fn AABB(A: Rect, B: Rect) -> bool {
    A.x + A.width() as i32 >= B.x
        && B.x + B.width() as i32 >= A.x
        && A.y + A.height() as i32 >= B.y
        && B.y + B.height() as i32 >= A.y
}
