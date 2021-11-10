use std::{marker::PhantomData, time::Instant};

use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::Color,
    rect::{Point, Rect},
    render::{BlendMode, Texture, WindowCanvas},
};

use crate::controller::{Controller, Moveset};

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

#[derive(Debug)]
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
}

impl<'a> Player<'a> {
    /// initialize a player
    pub fn new(sprite: Rect, id: i32) -> Self {
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

    // pub fn handle(&mut self, event: &Event) {

    // }

    pub fn render(&mut self, canvas: &mut WindowCanvas, texture: &Texture, camera: &Rect) {
        let center_pos = self
            .dest
            .center()
            .offset(-camera.x, -(self.dest.height() as i32) / 2 - 10 - camera.y);
        let healthbar = Rect::from_center(center_pos, self.health / 2, self.dest.height() / 4);
        self.flip = match (self.movement.1 as i8) - (self.movement.3 as i8) {
            -1 => true,
            1 => false,
            _ => self.flip,
        };
        if !self.movement.should_play_animation() {
            self.sprite.x = 8;
        }
        let mut dest = self.dest;
        dest.x -= camera.x;
        dest.y -= camera.y;
        canvas
            .copy_ex(
                texture,
                Some(self.sprite),
                Some(dest),
                0.0,
                None,
                self.flip,
                false,
            )
            .expect("failed to render player");

        if self.showhitbox {
            canvas.set_draw_color(Color::RGB(255, 0, 0));
            canvas.draw_rect(dest).expect("drawing rect failed");
        }
        canvas.set_blend_mode(BlendMode::Blend);
        if self.health == 100 {
            canvas.set_draw_color(Color::RGBA(0, 255, 0, 163));
        } else if self.health <= 75 {
            canvas.set_draw_color(Color::RGBA(122, 123, 0, 163));
        } else if self.health <= 50 {
            canvas.set_draw_color(Color::RGBA(200, 50, 0, 163));
        } else if self.health <= 25 {
            canvas.set_draw_color(Color::RGBA(255, 0, 0, 163));
        }
        canvas.fill_rect(healthbar);
        canvas.set_draw_color(Color::RGB(0, 0, 0));
    }
}

impl Controller for Player<'_> {
    // sdl2::event::EventType
    fn handle(&mut self, event: &Event) {
        // my intellisense not work and itd terrible
        match event {
            // doesn't format
            Event::KeyDown {
                keycode: Some(keycode),
                ..
            }
            | Event::KeyUp {
                keycode: Some(keycode),
                ..
            } => self.movement.set_by_key(
                *keycode,
                matches!(event, Event::KeyDown { .. }),
                &self.moveset,
            ), // somewhere there is a big error here, probably syntactical
            _ => {} // moveset needs
        }
    }
    fn set_moveset(&mut self, moveset: Moveset) {
        self.moveset = moveset;
    }
    fn get_moveset(&self) -> Moveset {
        self.moveset
    }
}
