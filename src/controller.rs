use sdl2::{event::Event, keyboard::Keycode, mouse::MouseButton, render::WindowCanvas, video::FullscreenType};

use crate::{GAME_SIZE_X, GAME_SIZE_Y, entity::Player};


#[derive(Debug, Copy, Clone)]
pub struct Controller {
}

impl Controller {
    pub fn new() -> Self {
        Self {}
    }

    pub fn handle(self, event: &Event, player: &mut Player, ) {
        match event {
            Event::KeyDown {
                keycode: Some(keycode),
                ..
            } => {
                player.movement.set_by_key(*keycode, true);
            }
            Event::KeyUp {
                keycode: Some(keycode),
                ..
            } => {
                player.movement.set_by_key(*keycode, false);
            },
            Event::MouseButtonDown {
                mouse_btn: MouseButton::Left, ..
            } => {
                player.attack()
            }
            _ => {}
        }
    }
}