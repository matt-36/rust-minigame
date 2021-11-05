use std::clone;

use sdl2::{render::{Texture}};

#[derive(Copy, Clone)]
pub enum EntityTypes {
    PlayerType,
    MobType,
    CollisionType
}

#[derive(Copy, Clone)]
pub enum State {
    Alive,
    Dead,
    Respawning,
}


pub struct Entity {
    texture: (),
    pub p_x: i32,
    pub p_y: i32,
    pub s_x: i32,
    pub s_y: i32,
    pub type_: EntityTypes,
    pub health: i32,
    pub state: State
}

impl Entity {
    // create a new entity
    pub fn new(
        texture: &Texture, 
        x_coordinate: i32, 
        y_coordinate: i32,
        x_scale: i32, 
        y_scale: i32, 
        type_: EntityTypes
    ) -> Entity {
        Entity {
            texture,
            p_x: x_coordinate,
            p_y: y_coordinate,
            s_x: x_scale,
            s_y: y_scale,
            type_,
            health: 100,
            state: State::Alive
        }
    }

    pub fn attack(&self, who: Entity) {
        'attacking: loop {
            
        }
    }

    pub fn move_to(mut self, x_coordinate: i32, y_coordinate: i32) {
        self.p_x = x_coordinate;
        self.p_y = y_coordinate;
    }

    /// Get a reference to the entity's texture.
    pub fn get_texture(&self) -> &Texture {
        &self.texture
    }
}


trait Player {
    
}

impl Player for Entity<'_> {

}