use std::clone;

use sdl2::render::{Texture};

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



pub struct Entity<'a> {
    texture: Texture<'a>,
    p_x: i32,
    p_y: i32,
    s_x: i32,
    s_y: i32,
    type_: EntityTypes,
    health: i32,
    name: String,
    state: State
}

impl Entity<'_> {
    // create a new entity
    pub fn Init(
        texture: Texture, 
        x_coordinate: i32, 
        y_coordinate: i32,
        x_scale: i32, 
        y_scale: i32, 
        type_: EntityTypes,
        name: String
    ) -> Entity {
        Entity {
            texture,
            p_x: x_coordinate,
            p_y: y_coordinate,
            s_x: x_scale,
            s_y: y_scale,
            type_: type_,
            name: name,
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

impl<'a> Clone for Entity<'a> {
    fn clone(&self) -> Self {
        Entity {
            texture: self.texture.clone(),
            p_x: self.p_x.clone(),
            p_y: self.p_y.clone(),
            s_x: self.s_x.clone(),
            s_y: self.s_y.clone(),
            type_: self.type_.clone(),
            name: self.name.clone(),
            health: 100,
            state: State::Alive,
        }
    }

    fn clone_from(&mut self, source: &Self) {
        Self { texture: self.texture.clone(), p_x: self.p_x.clone(), p_y: self.p_y.clone(), s_x: self.s_x.clone(), s_y: self.s_y.clone(), type_: self.type_.clone(), health: self.health.clone(), name: self.name.clone(), state: self.state.clone() }
    }
}


trait Player {
    
}

impl Player for Entity<'_> {

}