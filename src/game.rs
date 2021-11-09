use std::collections::HashMap;

use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;


use crate::controller::Controller;
use crate::entity::Player;
use crate::collision::{Colider, AABB};


pub struct Game<'a> {
    pub players: Vec<Player<'a>>,
    pub colliders: Vec<Colider<'a>>,
    pub textures: HashMap<i32, Texture<'a>>,
    pub texture_creator: TextureCreator<WindowContext>
}

impl <'a> Game <'a> {
    pub fn new(texture_creator: TextureCreator<WindowContext>) -> Game<'a> {
        let players = Vec::new();
        let colliders = Vec::new();
        let textures = HashMap::new();
        Self {
            players,
            colliders,
            textures,
            texture_creator
        }
    }
    pub fn add_player(&'a mut self, player: Player, filename: &str) {
        let texture: Texture<'a> = self.texture_creator.load_texture(filename).unwrap();
        self.textures.entry(player.id).or_insert(texture);
    }
    pub fn add_collider(&mut self, collider: Colider, texture: Texture<'a>) {
        self.textures.entry(collider.id).or_insert(texture);
    }

    pub fn players(self) -> Vec<Player<'a>> {
        self.players
    }
    pub fn colliders(self) -> Vec<Colider<'a>> {
        self.colliders
    }

    pub fn handle(&mut self, event: &Event) {
        for player in self.players.iter_mut() {
            player.handle(event)
        }
    }

    pub fn update(&mut self, canvas: &mut WindowCanvas) -> Result<(), ()> {
        for collider in &mut self.colliders {
            let texture = self.textures.get(&collider.id).unwrap();
            collider.render(canvas, texture);
        }
        for player in &self.players {
            
        }

        Ok(())
    }
}