use std::collections::HashMap;
use std::convert::TryInto;

use rand::Rng;
use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;

use crate::collision::{aabb, Collider};
use crate::controller::Controller;
use crate::entity::Player;
use crate::{GAME_SIZE_X, GAME_SIZE_Y};

pub struct Game<'a> {
    pub players: Vec<Player<'a>>,
    pub colliders: Vec<Collider<'a>>,
    pub textures: HashMap<i32, Texture<'a>>,
    pub texture_creator: &'a TextureCreator<WindowContext>,
}

const MINIMAP_SIZE_X: f32 = GAME_SIZE_X as f32 / 4f32;
const MINIMAP_SIZE_Y: f32 = GAME_SIZE_Y as f32 / 4f32;

impl<'a> Game<'a> {
    pub fn new(texture_creator: &'a TextureCreator<WindowContext>) -> Game<'a> {
        let players = Vec::new();
        let colliders = Vec::new();
        let textures = HashMap::new();
        Self {
            players,
            colliders,
            textures,
            texture_creator,
        }
    }
    pub fn add_player(&mut self, player: Player<'a>, filename: &str) {
        println!("{} {}", player.id, filename); // its the same texture but the src rects have different y coordinates
        self.textures
            .entry(player.id)
            .or_insert(self.texture_creator.load_texture(filename).unwrap().into());
        self.players.push(player);
        // println!("{:?}", );
    } // texture? is that needed? ye colliders can have textures... we need
      //to add decoration one also
      // how about having filename be option?
      // then we can invis hitboxes
      // ye lets do it
    pub fn add_collider(&mut self, collider: Collider<'a>, filename: Option<&str>) {
        if let Some(filename) = filename {
            self.textures
                .entry(collider.id)
                .or_insert(self.texture_creator.load_texture(filename).unwrap().into());
        };
        self.colliders.push(collider);
        // println!("{:?}", );
    }
    pub fn tick(&mut self, ticks: u32) {
        for player in self.players.iter_mut() {
            let tmp_x = player.dest.x;
            let tmp_y = player.dest.y;
            {
                let speed = player.movement.get_speed();
                player.dest.x += speed.0 * 2;
                player.dest.y += speed.1 * 2;
                if player.movement.should_play_animation() {
                    player.sprite.x = (32 * (((ticks as f32 / 120f32) % 4f32) as f32) as i32) + 8;
                } // now we just need to seperate controller by arrow and wasd
            } // before we got speed and multiplied it by the movement speed constant
            for collider in self.colliders.iter_mut() {
                // oh my idea is to ute player pos before collider for and then check each colliders AABB
                if aabb(player.dest, collider.dest) {
                    // rust can break loops with values
                    player.dest.x = tmp_x;
                    player.dest.y = tmp_y;
                    break;
                }
            }
            // checks points on player and collider and see if they collide
            //
        }
    }

    pub fn players(self) -> Vec<Player<'a>> {
        self.players
    }
    pub fn colliders(self) -> Vec<Collider<'a>> {
        self.colliders
    }

    pub fn handle(&mut self, event: &Event) {
        for player in self.players.iter_mut() {
            player.handle(event)
        }
    }

    pub fn togglehitboxes(&mut self) {
        for player in &mut self.players {
            player.togglehitbox()
        }
    }

    pub fn render_minimap(&mut self, canvas: &mut WindowCanvas) {
        let x = Rect::new(0, 0, GAME_SIZE_X / 4, GAME_SIZE_Y / 4);
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.fill_rect(x);
        canvas.set_draw_color(Color::RGB(0, 255, 0));
        canvas.draw_rect(x);

        /**
         *  minimum border
         *     U
         * ////////////////////////////////////////////
         * ///                                      s//  < minimum border
         * ///   P                                  s//
         * ///                                      s//
         * ///                                      s//
         * ///                                      s//
         * ///                                      s//
         * ///                         xxxxxxxxxx   s//
         * ///                         x        x   s//
         * ///                         x        x   s//
         * ///                         xxxxxxxxxx   s//
         * ///                                      s//
         * ////////////////////////////////////////////
         *
         * give a number of rects to it, then it will render each one somewhere
         * give colour by id or something
         */
        // println!("\n\n\n");
        let mut rects: Vec<Rect> = vec![];
        for player in self.players.iter() {
            rects.push(player.dest);
        }
        for collider in self.colliders.iter() {
            rects.push(collider.dest);
        }
        rects.push(canvas.viewport());

        // println!("{:?}", canvas.viewport());

        // rects.iter().for_each(|r| {
        //     println!(
        //         "{:?}",
        //         (r.x, r.y, r.x + r.width() as i32, r.y + r.height() as i32)
        //     )
        // });

        let min_x = rects
            .iter()
            .reduce(|r1, r2| if r1.left() < r2.left() { r1 } else { r2 })
            .unwrap()
            .left() as f32;
        let max_x = rects
            .iter()
            .reduce(|r1, r2| if r1.right() > r2.right() { r1 } else { r2 })
            .unwrap()
            .right() as f32;
        // let max_x = max_x.x as f32 + max_x.width() as f32;

        let min_y = rects
            .iter()
            .reduce(|r1, r2| if r1.top() < r2.top() { r1 } else { r2 })
            .unwrap()
            .top() as f32;
        let max_y = rects
            .iter()
            .reduce(|r1, r2| if r1.bottom() > r2.bottom() { r1 } else { r2 })
            .unwrap()
            .bottom() as f32;
        // let max_y = max_y.y as f32 + max_y.height() as f32;
        // println!("");
        // rects
        //     .iter()
        //     .for_each(|r| println!("{}, {}", r.y, r.y + r.height() as i32));
        // println!("{}\n", max_y);

        canvas.draw_rect(Rect::new(
            0,
            0,
            MINIMAP_SIZE_X as u32,
            MINIMAP_SIZE_Y as u32,
        ));
        // println!("{:?}", (min_x, min_y, max_x, max_y));

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        for rect in rects.iter() {
            let x = ((rect.left() as f32 - min_x) / (max_x - min_x) * MINIMAP_SIZE_X) as i32;
            let y = ((rect.top() as f32 - min_y) / (max_y - min_y) * MINIMAP_SIZE_Y) as i32;
            let w = (rect.width() as f32 / (max_x - min_x) * MINIMAP_SIZE_X) as u32;
            let h = (rect.height() as f32 / (max_y - min_y) * MINIMAP_SIZE_Y) as u32;
            canvas.draw_rect(Rect::new(x, y, w, h)).unwrap();
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        // canvas.set_draw_color(Color::RGB(0, 0, 0));
        // for player in self.players.iter() {
        //     let mut y = rand::thread_rng();
        //     canvas.set_draw_color(Color::RGB(
        //         y.gen_range(0..255),
        //         y.gen_range(0..255),
        //         y.gen_range(0..255),
        //     ));
        //     canvas.draw_rect(Rect::new(
        //         player.dest.x / 12 + x.width() as i32 / 2,
        //         player.dest.y / 12 + x.width() as i32 / 2,
        //         player.dest.width() / 12,
        //         player.dest.height() / 12,
        //     ));
        //     canvas.set_draw_color(Color::RGB(0, 0, 0));
        // }
    }

    pub fn update(&mut self, canvas: &mut WindowCanvas) -> Result<(), ()> {
        for collider in &mut self.colliders {
            let texture = self.textures.get(&collider.id).unwrap();
            collider.render(canvas, texture);
        }
        for player in &mut self.players {
            let texture = self.textures.get(&player.id).unwrap();
            player.render(canvas, texture);
        }
        // the textures are loaded in main function
        // with game
        self.render_minimap(canvas);

        Ok(())
    }
}
