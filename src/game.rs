use std::collections::HashMap;

use sdl2::render::{Texture, WindowCanvas};


use crate::controller::Controller;
use crate::entity::Player;
use crate::collision::{Colider, AABB};



pub struct Game<'a> {
    pub players: Vec<Player>,
    pub colliders: Vec<Colider>,
    pub textures: HashMap<i32, Texture<'a>>
}

impl <'a> Game <'a> {
    pub fn new() -> Self {
        let players = Vec::new();
        let colliders = Vec::new();
        let textures = HashMap::new();
        Self {
            players,
            colliders,
            textures
        }
    }
    pub fn add_player(&mut self, player: Player, texture: Texture<'a>) {
        self.textures.entry(player.id).or_insert(texture);
    }
    pub fn add_collider(&mut self, collider: Colider, texture: Texture<'a>) {
        self.textures.entry(collider.id).or_insert(texture);
    }


    pub fn update(&mut self, canvas: &mut WindowCanvas) -> Result<(), ()> {
        for collider in &self.colliders {
            let texture = self.textures.get(&collider.id).unwrap();
            collider.render(canvas, texture);
        }


        Ok(())
    }
}