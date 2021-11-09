use sdl2::{
    event::Event, keyboard::Keycode
};


#[derive(Debug, Clone, Copy)]
pub struct Moveset {
    pub up: Keycode,
    pub left: Keycode,
    pub down: Keycode,
    pub right: Keycode,
}

impl Default for Moveset {
    fn default() -> Self {
        Self {
            up: Keycode::W,
            left: Keycode::A, // ye it was cos it was diff type
            down: Keycode::S,
            right: Keycode::D,
        }
    }
}

// #[derive(Default, Debug, Clone)]
// pub struct Controller {
//     moveset: Moveset,
// }

pub trait Controller {
    fn handle(&mut self, event: &Event);
    fn set_moveset(&mut self, moveset: Moveset);
    fn get_moveset(&self) -> Moveset;
}

// impl Controller {
//     pub fn new(moveset: Option<Moveset>) -> Self {
//         // put it on the struct?
//         Self {
//             moveset: moveset.unwrap_or_default(),
//         }
//     }
//     pub fn handle(&self, event: &Event, player: &mut Player) {
//         match event {
//             Event::KeyDown {
//                 keycode: Some(keycode),
//                 ..
//             } => {
//                 player.movement.set_by_key(*keycode, true);
//             }
//             Event::KeyUp {
//                 keycode: Some(keycode),
//                 ..
//             } => {
//                 player.movement.set_by_key(*keycode, false, self.moveset);
//             }
//             Event::MouseButtonDown {
//                 mouse_btn: MouseButton::Left,
//                 ..
//             } => player.attack(),
//             _ => {}
//         }
//     }
// }
