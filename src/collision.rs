use std::marker::PhantomData;

use sdl2::{
    rect::Rect,
    render::{Texture, WindowCanvas},
};

#[derive(Debug)]

pub struct Collider<'a> {
    pub sprite: Rect,
    pub dest: Rect,
    pub id: i32,
    phantom: PhantomData<&'a i32>,
}

impl<'a> Collider<'a> {
    pub fn new(sprite: Rect, dest: Rect, id: i32) -> Self {
        Self {
            sprite,
            dest,
            id,
            phantom: PhantomData,
        }
    }

    pub fn render(&self, canvas: &mut WindowCanvas, texture: &Texture, camera: &Rect) {
        let mut dest = self.dest;
        dest.offset(-camera.x, -camera.y);
        canvas.copy(
            texture, 
            Some(self.sprite), 
            Some(dest)
        ).expect("rendering failed");
    }
}

pub fn aabb(a: Rect, b: Rect) -> bool {
    a.has_intersection(b)
    // a.x + a.width() as i32 >= b.x
    //     && b.x + b.width() as i32 >= a.x
    //     && a.y + a.height() as i32 >= b.y
    //     && b.y + b.height() as i32 >= a.y // watch when i try to change the hitbox size
}

// pub fn test_aabb(a: Rect, b: Rect) -> bool{
//     // Rect::intersection(&self, other)
    
//     Rect::has_intersection(&self, other)
//     ;true
// }