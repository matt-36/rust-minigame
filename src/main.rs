extern crate sdl2;
extern crate serde_json;
extern crate serde;

mod animation;
mod entity;
mod game;
mod events;
use std::path::Path;
use std::time::Duration;

use entity::Entity;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};
use sdl2::image::{InitFlag, LoadTexture};


fn something(ticks: u32, fpm: u32) -> i32 {
    32 * ((ticks / 100) % fpm) as i32
}


fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;
    // the window is the representation of a window in your operating system,
    // however you can only manipulate properties of that window, like its size, whether it's
    // fullscreen, ... but you cannot change its content without using a Canvas or using the
    // `surface()` method.
    let window = video_subsystem
        .window(
            "mountain of doom",
            800,
            600,
        )
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    // the canvas allows us to both manipulate the property of the window and to change its content
    // via hardware or software rendering. See CanvasBuilder for more info.
    let mut canvas = window
        .into_canvas()
        .target_texture()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    println!("Using SDL_Renderer \"{}\"", canvas.info().name);
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    // clears the canvas with the color we set in `set_draw_color`.
    canvas.clear();
    // However the canvas has not been updated to the window yet, everything has been processed to
    // an internal buffer, but if we want our buffer to be displayed on the window, we need to call
    // `present`. We need to call this everytime we want to render a new frame on the window.
    canvas.present();

    // this struct manages textures. For lifetime reasons, the canvas cannot directly create
    // textures, you have to create a `TextureCreator` instead.
    let texture_creator: TextureCreator<_> = canvas.texture_creator();
    let temp_surface = sdl2::surface::Surface::load_bmp(Path::new("assets/characters.bmp"))?;
    let texture = texture_creator
        .create_texture_from_surface(&temp_surface)
        .map_err(|e| e.to_string())?;
    let timer = sdl_context.timer()?;
    let mut player = Rect::new(0, 0, 32, 32);
    let mut dest_player = Rect::new(0, 0, 32 * 4, 32 * 4);
    dest_player.center_on(Point::new(60, 60));

    let mut event_pump = sdl_context.event_pump()?;
    let mut flip = false;
    'running: loop {
        // get the inputs here
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::End),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => {
                    player.x = something(timer.ticks(), 4u32);
                    dest_player.y-=4;
                },
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => {
                    flip = true;
                    player.x = something(timer.ticks(), 4u32);
                    dest_player.x-=4;
                },
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => {
                    player.x = something(timer.ticks(), 4u32);
                    dest_player.y+=4;
                },
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => {
                    flip = false;
                    player.x = something(timer.ticks(), 4u32);
                    dest_player.x+=4;
                },
                _ => {}
            }
        }

        // update the game loop here


        canvas.clear();
        canvas.copy_ex(
            &texture,
            Some(player),
            Some(dest_player),
            0.0,
            None,
            flip,
            false,
        )?;
        canvas.present();
        std::thread::sleep(Duration::from_millis(0));
    }
    Ok(())
}