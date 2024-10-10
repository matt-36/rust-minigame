use sdl2::{
    rect::Rect,
    render::{Canvas, RenderTarget},
};

use crate::types::minimap::Minimap;

pub enum RenderType<'a> {
    Canvas {
        camera: &'a Rect,
    },
    HitBox {
        camera: &'a Rect,
    },
    Minimap{
        options: &'a Minimap
    },
    Placeholder,
}

pub trait Render {
    fn render<T: RenderTarget>(&self, canvas: &mut Canvas<T>, r#type: RenderType<'_>) -> Option<()>;
}
