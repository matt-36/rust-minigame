use std::marker::PhantomData;

use sdl2::{
    rect::Rect,
    render::{Texture, WindowCanvas},
};

use crate::traits::render::{Render, RenderType};

pub struct Collider<'a> {
    pub sprite: Rect,
    pub dest: Rect,
    pub id: i32,
    phantom: PhantomData<&'a i32>,
    #[cfg(not(feature = "unsafe_textures"))]
    pub texture: &'a Texture<'a>,
    #[cfg(feature = "unsafe_textures")]
    pub texture: Texture,

}

impl<'a> Collider<'a> {
    pub fn new(
        sprite: Rect, 
        dest: Rect, 
        id: i32, 
        #[cfg(not(feature = "unsafe_textures"))]
        texture: &'a Texture<'_>,
        #[cfg(feature = "unsafe_textures")]
        texture: Texture
    ) -> Self {
        Self {
            sprite,
            dest,
            id,
            phantom: PhantomData,
            texture,
        }
    }

    // pub fn render(&self, canvas: &mut WindowCanvas, camera: &Rect) {
    //     let mut dest = self.dest;
    //     dest.offset(-camera.x, -camera.y);
    //     canvas
    //         .copy(self.texture, Some(self.sprite), Some(dest))
    //         .expect("rendering failed");
    // }
}

impl Render for Collider<'_> {
    fn render<T: sdl2::render::RenderTarget>(
        &self,
        canvas: &mut sdl2::render::Canvas<T>,
        r#type: crate::traits::render::RenderType<'_>,
    ) -> Option<()> {
        match r#type {
            RenderType::Canvas { camera } => {
                if self.dest.has_intersection(*camera) {
                    canvas
                        .copy(
                            self.texture,
                            Some(self.sprite),
                            Some(crate::offset!(-;self.dest, camera)),
                        )
                        .expect("rendering failed");
                }
            }
            _ => {}
        }
        Some(())
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
