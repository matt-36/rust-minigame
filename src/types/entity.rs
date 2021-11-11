use std::{marker::PhantomData, time::Instant};

use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::Color,
    rect::{Point, Rect},
    render::{BlendMode, Texture, WindowCanvas},
};

use crate::traits::render::{Render, RenderType};

use super::controller::{Controller, Moveset};

#[derive(Debug, Clone, Copy, Default)]
pub struct Movement(pub bool, pub bool, pub bool, pub bool);

impl Movement {
    pub fn get_speed(&self) -> (i32, i32) {
        // see unused
        (
            (self.1 as i32) - (self.3 as i32),
            (self.0 as i32) - (self.2 as i32),
        )
    }
    pub fn should_play_animation(&self) -> bool {
        (self.0 ^ self.2) | (self.1 ^ self.3)
    }
    pub fn set_by_key(&mut self, keycode: Keycode, up_or_down: bool, moveset: &Moveset) {
        match keycode {
            _ if keycode == moveset.up => self.2 = up_or_down,
            _ if keycode == moveset.left => self.3 = up_or_down,
            _ if keycode == moveset.down => self.0 = up_or_down,
            _ if keycode == moveset.right => self.1 = up_or_down,
            _ => {}
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Swing {
    pub start: Instant,
    pub iteration: u8,
}

pub struct Player<'a> {
    pub sprite: Rect,
    pub dest: Rect,
    pub movement: Movement,
    moveset: Moveset,
    pub flip: bool,
    pub swing: Option<Swing>,
    pub swingsprite: Option<Rect>,
    pub id: i32,
    pub showhitbox: bool,
    phantom: PhantomData<&'a i32>,
    pub health: u32,
    pub texture: &'a Texture<'a>,
}

impl<'a> Player<'a> {
    /// initialize a player
    pub fn new(sprite: Rect, id: i32, texture: &'a Texture) -> Self {
        let mut dest = Rect::new(0, 0, sprite.width() * 2, sprite.height() * 2);
        dest.center_on(Point::new(60 as i32 / 2, 60 as i32 / 2));
        Self {
            sprite,
            dest,
            movement: Movement::default(),
            flip: false,
            swing: None,
            swingsprite: None,
            id,
            showhitbox: false,
            phantom: PhantomData,
            moveset: Default::default(),
            health: 50,
            texture,
        }
    }
    pub fn togglehitbox(&mut self) {
        self.showhitbox = !self.showhitbox;
    }

    pub fn attack(&mut self) {
        self.swing = Some(Swing {
            start: Instant::now(),
            iteration: 0,
        });
        self.swingsprite = Some(Rect::new(0, 0, 32, 32))
    }

    // pub fn render_old(&mut self, canvas: &mut WindowCanvas, camera: &Rect) {
    //     let center_pos = self
    //         .dest
    //         .center()
    //         .offset(-camera.x, -(self.dest.height() as i32) / 2 - 10 - camera.y);
    //     let healthbar = Rect::from_center(center_pos, self.health / 2, self.dest.height() / 4);
    //     self.flip = match (self.movement.1 as i8) - (self.movement.3 as i8) {
    //         -1 => true,
    //         1 => false,
    //         _ => self.flip,
    //     };
    //     if !self.movement.should_play_animation() {
    //         self.sprite.x = 8;
    //     }
    //     let mut dest = self.dest;
    //     dest.x -= camera.x;
    //     dest.y -= camera.y;
    //     canvas
    //         .copy_ex(
    //             self.texture,
    //             Some(self.sprite),
    //             Some(dest),
    //             0.0,
    //             None,
    //             self.flip,
    //             false,
    //         )
    //         .expect("failed to render player");

    //     if self.showhitbox {
    //         canvas.set_draw_color(Color::RGB(255, 0, 0));
    //         canvas.draw_rect(dest).expect("drawing rect failed");
    //     }
    //     canvas.set_blend_mode(BlendMode::Blend);
    //     canvas.set_draw_color(match self.health {
    //         0..=25 => Color::RGBA(255, 0, 0, 163),
    //         26..=50 => Color::RGBA(200, 50, 0, 163),
    //         51..=75 => Color::RGBA(122, 123, 0, 163),
    //         76..=100 => Color::RGBA(0, 255, 0, 163),
    //         _ => Color::RGBA(0, 0, 0, 0),
    //     });
    //     canvas.fill_rect(healthbar).unwrap();
    //     canvas.set_draw_color(Color::RGB(0, 0, 0));
    // }
    pub fn pre_render(&mut self) {
        self.flip = match (self.movement.1 as i8) - (self.movement.3 as i8) {
            -1 => true,
            1 => false,
            _ => self.flip,
        };
        if !self.movement.should_play_animation() {
            self.sprite.x = 8;
        }
    }
}

impl Controller for Player<'_> {
    // sdl2::event::EventType
    fn handle(&mut self, event: &Event) {
        match event {
            Event::KeyUp {
                keycode: Some(keycode),
                ..
            }
            | Event::KeyDown {
                keycode: Some(keycode),
                ..
            } => self.movement.set_by_key(
                *keycode,
                matches!(event, Event::KeyDown { .. }),
                &self.moveset,
            ),
            _ => {}
        }
    }
    fn set_moveset(&mut self, moveset: Moveset) {
        self.moveset = moveset;
    }
    fn get_moveset(&self) -> Moveset {
        self.moveset
    }
}

impl Render for Player<'_> {
    fn render<T: sdl2::render::RenderTarget>(
        &self,
        canvas: &mut sdl2::render::Canvas<T>,
        r#type: RenderType,
    ) -> Option<()> {
        match r#type {
            RenderType::Canvas { camera } => {
                if camera.has_intersection(self.dest) {
                    canvas
                        .copy_ex(
                            self.texture,
                            Some(self.sprite),
                            Some(crate::offset!(-;self.dest, camera)),
                            0.0,
                            None,
                            self.flip,
                            false,
                        )
                        .expect("failed to render player");

                    //* healthbar
                    let healthbar = Rect::from_center(
                        self.dest
                            .center()
                            .offset(-camera.x, -(self.dest.height() as i32) / 2 - 10 - camera.y),
                        self.health / 2,
                        self.dest.height() / 4,
                    );
                    canvas.set_blend_mode(BlendMode::Blend);
                    canvas.set_draw_color(match self.health {
                        0..=25 => Color::RGBA(255, 0, 0, 163),
                        26..=50 => Color::RGBA(200, 50, 0, 163),
                        51..=75 => Color::RGBA(122, 123, 0, 163),
                        76..=100 => Color::RGBA(0, 255, 0, 163),
                        _ => Color::RGBA(0, 0, 0, 0),
                    });
                    canvas.fill_rect(healthbar).unwrap();
                }
            }
            RenderType::HitBox { camera } => {
                //* hitbox rendering
                if self.showhitbox {
                    canvas.set_draw_color(Color::RGB(255, 0, 0));
                    canvas
                        .draw_rect(crate::offset!(-; self.dest, camera))
                        .expect("drawing rect failed");
                }
            }
            RenderType::Minimap { options } => {
                let scaled_rect = crate::minimap_scaled_rect!(self.dest, options);
            }
            _ => {}
        };
        Some(())
    }
}
