use sdl2::{event::Event, keyboard::Keycode, mouse::MouseButton, render::WindowCanvas, video::FullscreenType};

use crate::{GAME_SIZE_X, GAME_SIZE_Y, entity::Player};


#[derive(Debug, Copy, Clone)]
pub struct Controller {
}

impl Controller {
    pub fn new() -> Self {
        Self {}
    }

    pub fn handle(self, event: &Event, canvas: &mut WindowCanvas, player: &mut Player, fullscreen: &mut bool) {
        match event {
            Event::KeyDown {
                keycode: Some(Keycode::F11),
                ..
            } => {
                // canvas.window_mut().set_size(1920, 1080).expect("fuck");
                let window = canvas.window_mut();
                window
                    .set_fullscreen(if *fullscreen {
                        FullscreenType::Desktop
                    } else {
                        FullscreenType::Off
                    })
                    .expect("fuck");
                let size = window.size();
                canvas
                    .set_scale(
                        (size.0 as f32) / (GAME_SIZE_X as f32),
                        (size.1 as f32) / (GAME_SIZE_Y as f32),
                    )
                    .expect("fuck");
                let nfullscreen = !*fullscreen;
                *fullscreen = nfullscreen;
            }

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