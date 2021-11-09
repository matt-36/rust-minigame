use sdl2::{rect::Rect, render::{Texture, WindowCanvas}};



#[derive(Clone, Copy)]
pub struct Colider {
    pub sprite: Rect,
    pub dest: Rect,
    pub id: i32,
}

impl Colider {
    pub fn new(
        sprite: Rect,
        dest: Rect,
        id: i32
    ) -> Self {
        Self {
            sprite,
            dest,
            id
        }
    }

    pub fn render(self, canvas: &mut WindowCanvas, texture: &Texture) {
        canvas.copy(texture, self.sprite, self.dest);
    }
}

pub fn AABB(A: Rect, B: Rect) -> bool {
    A.x + A.width() as i32 -30 >= B.x
        && B.x + B.width() as i32 -30 >= A.x
        && A.y + A.height() as i32 -28 >= B.y
        && B.y + B.height() as i32 -24 >= A.y
}
