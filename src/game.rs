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

const MINIMAP_POS_X: i32 = 0;
const MINIMAP_POS_Y: i32 = 0;
const MINIMAP_SIZE_X: u32 = GAME_SIZE_X / 4;
const MINIMAP_SIZE_Y: u32 = GAME_SIZE_Y / 4;
const MINIMAP_MARGIN_X: u32 = 16;
const MINIMAP_MARGIN_Y: u32 = 16;

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
        let x = Rect::new(
            MINIMAP_POS_X,
            MINIMAP_POS_Y,
            MINIMAP_SIZE_X + 2 * MINIMAP_MARGIN_X,
            MINIMAP_SIZE_Y + 2 * MINIMAP_MARGIN_Y,
        );
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.fill_rect(x).unwrap();
        canvas.set_draw_color(Color::RGB(0, 255, 0));
        canvas.draw_rect(x).unwrap();

        /*
         *  minimum border
         *     U
         * /////////////////////////////////////////s//
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
         * /////////////////////////////////////////s//
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

        macro_rules! minmax {
            ($varname:ident $more_or_less:tt $method:ident) => {
                let $varname = rects
                    .iter()
                    .reduce(|r1, r2| if r1.$method() $more_or_less r2.$method() { r1 } else { r2 })
                    .unwrap()
                    .$method() as f32;
            };
        }
        minmax!(min_x < left);
        minmax!(max_x > right);
        minmax!(min_y < top);
        minmax!(max_y > bottom);
        let (dist_x, dist_y) = ((max_x - min_x), (max_y - min_y));
        let (ratio_x, ratio_y) = (
            dist_x / MINIMAP_SIZE_X as f32,
            dist_y / MINIMAP_SIZE_Y as f32,
        );
        let ratio = ratio_x.max(ratio_y);

        // apply offset
        let offset_x = (dist_x / ratio_x - dist_x / ratio) as i32 / 2
            + MINIMAP_POS_X
            + MINIMAP_MARGIN_X as i32;
        let offset_y = (dist_y / ratio_y - dist_y / ratio) as i32 / 2
            + MINIMAP_POS_Y
            + MINIMAP_MARGIN_Y as i32;

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        for rect in rects.iter() {
            let x = ((rect.left() as f32 - min_x) / ratio) as i32 + offset_x;
            let y = ((rect.top() as f32 - min_y) / ratio) as i32 + offset_y;
            let w = (rect.width() as f32 / ratio) as u32;
            let h = (rect.height() as f32 / ratio) as u32;
            canvas.draw_rect(Rect::new(x, y, w, h)).unwrap();
        }
        canvas.set_draw_color(Color::RGB(0, 0, 0));
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
