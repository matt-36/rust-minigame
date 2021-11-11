use sdl2::{rect::Rect, render::WindowCanvas};

pub struct Menu {
    buttons: Vec<Rect>
}

impl Menu {
    pub fn new() -> Self {
        Self {
            buttons: Vec::new()
        }
    }
    pub fn render(&self, canvas: &mut WindowCanvas) {

    }
}