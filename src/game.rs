use sdl2::image::LoadTexture;
use sdl2::render::TextureCreator;

use crate::entity::{Entity, EntityTypes};
use crate::events::EventHandler;

struct Game<'a> {
    players: Vec<Entity<'a>>,
    entities: Vec<Entity<'a>>,
    event_handler: EventHandler
}
impl <'a> Game<'a> {
    pub fn Init(event_handler: EventHandler) -> Game<'static> {
        Game { 
            players: Vec::new(), 
            entities: Vec::new(), 
            event_handler: event_handler
        }
    }
    pub fn update(mut self) {
        for event in self.event_handler.clone().IterIncoming().iter_mut() {
            let x = self.event_handler.clone().handle(event.clone()).unwrap();
            match x["type"].as_u64().expect("failed to parse data type") {
                1 => {
                    if x["data"]["targettype"] == "entity" {
                        let m_x = x["data"]["x"].as_i64().expect("failed to parse x coordinate") as i32;
                        let m_y = x["data"]["y"].as_i64().expect("failed to parse y coordinate") as i32;
                        // self.entities.get(x["data"]["targetid"].as_i64().expect("failed to parse entity id") as usize).unwrap().move_to(m_x,m_y);

                    }
                }
                _ => {

                }
            }
        }
    }

    pub fn spawn(mut self, ent: Entity<'a>) {
        self.entities.push(ent);
    }
}