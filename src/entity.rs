use std::{time::Instant};

use sdl2::{EventPump, event::Event, keyboard::Keycode, rect::{Point, Rect}, render::{Texture, WindowCanvas}};

use crate::controller::Controller;

#[derive(Debug, Clone, Copy, Default)]
pub struct Movement(pub bool, pub bool, pub bool, pub bool);

impl Movement {
    pub fn get_speed(&self)->(i32, i32){
        ((self.1 as i32) - (self.3 as i32), (self.0 as i32) - (self.2 as i32))
    }
    pub fn should_play_animation(&self) -> bool {
        (self.0 ^ self.2) | (self.1 ^ self.3)
    }
    pub fn set_by_key(&mut self, keycode: Keycode, up_or_down: bool){
        match keycode {
            Keycode::W => self.2 = up_or_down,
            Keycode::A => self.3 = up_or_down,
            Keycode::S => self.0 = up_or_down,
            Keycode::D => self.1 = up_or_down,
            _ => {}
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Swing {
    pub start: Instant,
    pub iteration: u8
}


#[derive(Debug, Clone, Copy)]
pub struct Player {
    pub sprite: Rect,
    pub dest: Rect,
    pub movement: Movement,
    pub flip: bool,
    pub swing: Option<Swing>,
    pub swingsprite: Option<Rect>,
    pub id: i32,
    pub controller: Controller
}

impl Player {
    /// initialize a player
    pub fn new(
        sprite: Rect,
        id: i32,
    ) -> Self {
        let mut dest = Rect::new(0, 0, sprite.width() * 4, sprite.height() * 4);
        dest.center_on(Point::new(60 as i32 / 2,60 as i32 / 2));
        let controller = Controller::new();
        Self {
            sprite,
            dest,
            movement: Movement::default(),
            flip: false,
            swing: None,
            swingsprite: None,
            id,
            controller
        }
    }
    fn swing_anim(&self, ticks: u32, fpm: u32) -> i32 {
        let x = 32 * ((ticks / 100) % fpm) as i32;
        println!("{:?}", x);
        x
    }
    pub fn attack(&mut self) {
        self.swing = Some(Swing{start: Instant::now(), iteration: 0});
        self.swingsprite = Some(Rect::new(0, 0, 32, 32))
    }

    pub fn handle(&mut self, canvas: &mut WindowCanvas, event: &Event, fullscreen: &mut bool) {
        self.controller.handle(event, canvas, self, fullscreen);
    }

    pub fn render(&mut self, canvas: &mut WindowCanvas, texture: &Texture) {

        
        
        self.flip = match (self.movement.1 as i8) - (self.movement.3 as i8) {
            -1 => true,
            1 => false,
            _ => self.flip
        };
        
        canvas.copy_ex(
            texture, 
            Some(self.sprite), 
            Some(self.dest), 
            0.0, 
            None, 
            self.flip, 
            false
        ).expect("failed to render player");
        
    }
}