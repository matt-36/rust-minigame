use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{BlendMode, TextureCreator, WindowCanvas};
use sdl2::ttf::Sdl2TtfContext;
use sdl2::video::WindowContext;

use crate::rgame::menu::Menu;
// use crate::room::{Room, RoomManager};
use crate::constants::*;
use crate::rgame::room::RoomManager;
use crate::traits::render::{Render, RenderType};
use crate::types::collision::Collider;
use crate::types::controller::Controller;
use crate::types::entity::Player;
pub struct Game<'a> {
    pub players: Vec<Player<'a>>,
    pub colliders: Vec<Collider<'a>>,
    pub texture_creator: &'a TextureCreator<WindowContext>,
    pub camera: Rect,
    pub rooms: RoomManager,
}

impl<'a> Game<'a> {
    pub fn new(texture_creator: &'a TextureCreator<WindowContext>) -> Game<'a> {
        let players = Vec::new();
        let colliders = Vec::new();
        let camera = Rect::new(0, 0, GAME_SIZE_X, GAME_SIZE_Y);
        Self {
            players,
            colliders,
            texture_creator,
            camera,
            rooms: Default::default(),
        }
    }
    pub fn add_player(&mut self, player: Player<'a>) {
        self.players.push(player);
    }

    pub fn add_collider(&mut self, collider: Collider<'a>) {
        self.colliders.push(collider);
    }
    pub fn tick(&mut self, ticks: u32) {
        for player in self.players.iter_mut() {
            let x = player.dest.x;
            let y = player.dest.y;
            let speed = player.movement.get_speed();
            macro_rules! collision_loop {
                ($axis:tt, $label:tt) => {
                    $label: loop {
                        collision_loop!(self.rooms.get_room_rects_offset(player.dest.center()), $axis, $label);
                        collision_loop!(self.colliders.iter().map(|coll|coll.dest), $axis, $label);
                        break $label;
                    }
                };
                ($arr:expr, $axis:tt, $label:tt) => {
                    for item in $arr {
                        if player.dest.has_intersection(item) {
                            player.dest.$axis = $axis;
                            break $label;
                        }
                    }
                };
            }

            player.dest.x += speed.0 * 2;
            collision_loop!(x, 'x_collision);
            player.dest.y += speed.1 * 2;
            collision_loop!(y, 'y_collision);

            if player.movement.should_play_animation()
                && ((x != player.dest.x) || y != player.dest.y)
            {
                player.sprite.x = (32 * (((ticks as f32 / 120f32) % 4f32) as f32) as i32) + 8;
            }
        }
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

    pub fn rendermenu(&self, canvas: &mut WindowCanvas, menu: Menu) {
        menu.render(canvas);
    }

    pub fn renderui(&mut self, canvas: &mut WindowCanvas, ttf_context: &Sdl2TtfContext) {
        self.render_minimap(canvas);
        for player in self.players.iter() {
            let font = ttf_context
                .load_font("assets/font/ARCADECLASSIC.TTF", 256)
                .unwrap();
            let font_surface = font
                .render(&player.health.to_string() as &str)
                .blended(Color::RGBA(0, 255, 0, 255))
                .map_err(|e| e.to_string())
                .unwrap();
            let font_tex = self
                .texture_creator
                .create_texture_from_surface(font_surface)
                .unwrap();
            let x = self.camera.right() - (100 * player.id);
            let y = self.camera.top() + 6;
            let mut dest = Rect::new(x, y, 32, 32);
            dest.x -= self.camera.x;
            dest.y -= self.camera.y;
            let mut src = player.sprite;
            src.x = 8;
            src.h = (src.h as f32 / 1.5) as i32;
            canvas.copy(player.texture, src, dest).unwrap();
            canvas
                .copy(
                    &font_tex,
                    None,
                    Some(Rect::new(
                        (x + 30) - self.camera.x,
                        (y + 5) - self.camera.y,
                        50,
                        30,
                    )),
                )
                .expect("drawing text failed");
        }
    }

    pub fn render_minimap(&mut self, canvas: &mut WindowCanvas) {
        let prev_viewport = canvas.viewport();
        let prev_blend_mode = canvas.blend_mode();
        canvas.set_blend_mode(BlendMode::Blend);
        canvas.set_viewport(Rect::new(
            MINIMAP_POS_X,
            MINIMAP_POS_Y,
            MINIMAP_SIZE_X + 2 * MINIMAP_MARGIN_X,
            MINIMAP_SIZE_Y + 2 * MINIMAP_MARGIN_Y,
        ));
        let minimap_background = Rect::new(
            0,
            0,
            MINIMAP_SIZE_X + 2 * MINIMAP_MARGIN_Y,
            MINIMAP_SIZE_Y + 2 * MINIMAP_MARGIN_X,
        );
        canvas.set_draw_color(Color::RGBA(0, 0, 0, 127));
        canvas.fill_rect(minimap_background).unwrap();
        canvas.set_draw_color(Color::RGB(0, 255, 0));
        canvas.draw_rect(minimap_background).unwrap();
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

        let mut rects: Vec<Rect> = vec![];
        for player in self.players.iter() {
            rects.push(player.dest);
        }
        for collider in self.colliders.iter() {
            rects.push(collider.dest);
        }
        rects.push(self.camera);

        macro_rules! minmax {
            ($varname:ident $more_or_less:tt $method:ident) => {
                let $varname = rects.iter()
                    .reduce(|r1, r2| if r1.$method() $more_or_less r2.$method() { r1 } else { r2 })
                    .unwrap().$method() as f32;
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
        let offset_x = (dist_x / ratio_x - dist_x / ratio) as i32 / 2 + MINIMAP_MARGIN_X as i32;
        let offset_y = (dist_y / ratio_y - dist_y / ratio) as i32 / 2 + MINIMAP_MARGIN_Y as i32;

        canvas.set_draw_color(Color::RGB(255, 255, 255));

        macro_rules! get_draw_rect {
            ($rect:expr) => {{
                let x = (($rect.left() as f32 - min_x) / ratio) as i32 + offset_x;
                let y = (($rect.top() as f32 - min_y) / ratio) as i32 + offset_y;
                let w = ($rect.width() as f32 / ratio) as u32;
                let h = ($rect.height() as f32 / ratio) as u32;
                Rect::new(x, y, w, h)
            }};
        }

        canvas.draw_rect(get_draw_rect!(self.camera)).unwrap();

        for rect in rects {
            canvas.draw_rect(get_draw_rect!(rect)).unwrap();
        }
        for player in self.players.iter() {
            let tmp = get_draw_rect!(player.dest);
            let mut src = player.sprite;
            src.x = 8;
            src.h = (src.h as f32 / 1.5) as i32;
            canvas
                .copy(
                    player.texture,
                    src,
                    Rect::from_center(
                        tmp.center(),
                        (src.width() as f32 / 1.5) as u32,
                        (src.height() as f32 / 1.5) as u32,
                    ),
                )
                .unwrap();
        }

        for index in self.rooms.rooms.iter().map(|thing| thing.0.clone()).collect::<Vec<_>>() {
            for wall in self.rooms.get_room_rects_offset(index.clone()) {
                canvas.draw_rect(get_draw_rect!(wall)).unwrap();
            }
        }
        // for collider in self.colliders.iter() {
        //     // src.set_height(10);
        //     // src.set_width(10);
        //     // canvas
        //     //     .copy(
        //     //         self.textures.get(&collider.id).unwrap(),
        //     //         collider.sprite,
        //     //         get_draw_rect!(collider.sprite),
        //     //     )
        //     //     .unwrap();
        // }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.set_viewport(prev_viewport);
        canvas.set_blend_mode(prev_blend_mode);
    }

    pub fn update_camera_position(&mut self) {
        let player = self.players.first().unwrap();
        let player_pos = player.dest.center();
        if player_pos.x > self.camera.right() {
            self.camera.x += GAME_SIZE_X as i32;
        }
        if player_pos.y > self.camera.top() {
            self.camera.y += GAME_SIZE_Y as i32;
        }
        if player_pos.x < self.camera.left() {
            self.camera.x -= GAME_SIZE_X as i32;
        }
        if player_pos.y < self.camera.top() {
            self.camera.y -= GAME_SIZE_Y as i32;
        }
    }

    pub fn update(
        &mut self,
        canvas: &mut WindowCanvas,
        ttf_context: &Sdl2TtfContext,
    ) -> Result<(), ()> {
        for collider in &mut self.colliders {
            collider.render(
                canvas,
                RenderType::Canvas {
                    camera: &self.camera,
                },
            );
        }
        for player in &mut self.players {
            // player.render_old(canvas, &self.camera);
            player.pre_render();
            player.render(
                canvas,
                RenderType::Canvas {
                    camera: &self.camera,
                },
            );
            if player.showhitbox {
                player.render(
                    canvas,
                    RenderType::HitBox {
                        camera: &self.camera,
                    },
                );
            }
        }
        self.rooms.get_room(self.camera.center()).render(
            canvas,
            RenderType::Canvas {
                camera: &self.camera,
            },
        );
        self.update_camera_position();
        // the textures are loaded in main function
        // with game
        self.renderui(canvas, ttf_context);

        Ok(())
    }
}
