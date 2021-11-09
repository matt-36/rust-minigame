extern crate sdl2;
extern crate serde;
extern crate serde_json;

mod animation;
mod entity;
mod events;
mod game;
mod collision;
mod controller;

use game::Game;
use rand;
use sdl2::mouse::MouseButton;
use sdl2::sys::random;
use sdl2::{event::Event, video::FullscreenType};
// use sdl2::gfx::framerate::FPSManager;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::TextureCreator;
use std::time::{Duration, SystemTime};

use crate::collision::AABB;

const GAME_SIZE_X: u32 = 768;
const GAME_SIZE_Y: u32 = 432;

fn move_anim(ticks: u32, fpm: u32) -> i32 {
    let x = 32 * ((ticks / 100) % fpm) as i32;
    println!("{:?}", x);
    x
}



fn main() -> Result<(), String> {
    let MOVEMENT_SPEED = 4;
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;
    let _ttf_context = sdl2::ttf::init().expect("Failed to initialize ttf");
    // the window is the representation of a window in your operating system,
    // however you can only manipulate properties of that window, like its size, whether it's
    // fullscreen, ... but you cannot change its content without using a Canvas or using the
    // `surface()` method.
    let window = video_subsystem
        .window("rust game", GAME_SIZE_X, GAME_SIZE_Y)
        .position_centered()
        .allow_highdpi()
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
    canvas.set_integer_scale(true)?;
    println!("Using SDL_Renderer \"{}\"", canvas.info().name);
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    let mut game = Game::new();

    // this struct manages textures. For lifetime reasons, the canvas cannot directly create
    // textures, you have to create a `TextureCreator` instead.
    let texture_creator: TextureCreator<_> = canvas.texture_creator();
    let texture = texture_creator.load_texture("assets/characters.png")?;
    let wall_tex = texture_creator.load_texture("assets/yellow.png")?;
    let _swing_tex = texture_creator.load_texture("assets/swoosh.png")?;

    let timer = sdl_context.timer()?;
    let mut player = entity::Player::new(Rect::new(0, 96, 32, 32), 1);
    let mut player2 = entity::Player::new(Rect::new(0, 6, 32, 32), 2);
    let wall = collision::Colider::new(
        Rect::new(0, 0, 1200, 1200),
        Rect::new(200, 50, 120*4, 120*4),
        3
    );
    let font = _ttf_context.load_font("assets/font/Roboto-Bold.ttf", 256)?;
    let mut font_surface = font
        .render("N/A")
        .blended(Color::RGBA(0, 255, 0, 255))
        .map_err(|e| e.to_string())?;
    
    let mut font_tex = texture_creator.create_texture_from_surface(font_surface).unwrap();
    let obama_tex = texture_creator.load_texture("assets/obama.jpg")?;
    // let fpsmanager = FPSManager::new();
    let mut fullscreen = false;
    let mut event_pump = sdl_context.event_pump()?;
    let mut frame_times = [0u128; 60];
    let mut frames = 0;
    let mut last_frame_time = SystemTime::now();
    let mut r = 255;
    let mut g = 0;
    let mut b = 0;
    'running: loop {
        // get the inputs here
        for event in event_pump.poll_iter() {
            if event == Event::Quit{..} | Event::KeyDown{keycode: Keycode::Escape, ..} {
                break 'running;
            };
            for player in game.players.iter_mut() {
                let tmp_x = player.dest.x;
                let tmp_y = player.dest.y;
                player.handle(&mut canvas, &event, &mut fullscreen);
                for collider in game.colliders.iter_mut() {
                    if AABB(player.dest, collider.dest) {
                        player.dest.x = tmp_x;
                        player.dest.y = tmp_y;
                    }
                }
            }
        }

        // update the game loop here
        if player.movement.should_play_animation() {
            player.sprite.x = move_anim(timer.ticks(), 4);
        }
        let player_speed = player.movement.get_speed();
        let ix = player_speed.0 * MOVEMENT_SPEED;
        let iy = player_speed.1 * MOVEMENT_SPEED;
        player.dest.x += ix;
        player.dest.y += iy;
        // if AABB(player.dest, wall.dest) {
        //     player.dest.x -= ix;
        //     player.dest.y -= iy;
        // }
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas.draw_rect(wall.dest).expect("drawing rect failed");
        canvas.draw_rect(player.dest).expect("drawing rect failed");

        wall.render(&mut canvas, &obama_tex);
        player.render(&mut canvas, &texture);
        player2.render(&mut canvas, &texture);
        canvas.copy(&font_tex, None, Some(Rect::new(200, 200, 200, 100))).expect("drawing text failed");
        canvas.present();
        let frame_time = SystemTime::now();
        frame_times[frames % 60] = frame_time
            .duration_since(last_frame_time)
            .unwrap()
            .as_nanos();
        frames += 1;
        last_frame_time = frame_time;
        let fps = 60f64 / frame_times.iter().sum::<u128>() as f64 * 1_000_000_000f64;
        if r as u8 > 255u8 {
            // r-=rand::rng.gen::<i32>;
            r-=1;
        } else {
            r+=1
        }
        if g as u8 > 255u8 {
            g-=1;
        } else {
            g+=1;
        }

        font_surface = font
        .render("obama")
        .blended(Color::RGBA(r as u8, g as u8, b as u8, 255))
        .map_err(|e| e.to_string())?;
        font_tex = texture_creator.create_texture_from_surface(font_surface).unwrap();
        // ((frame_times.into_iter().map(|v|frame_time - v).sum::<u128>()) * 1_000_000_000);
        // println!("{:?}", 60f64 / (frame_times.into_iter().map(|v|frame_time - v).sum::<u128>() as f64) * 1_000_000_000f64);
        // println!(
        //     "{:?}",
        //     60f64 / frame_times.iter().sum::<u128>() as f64 * 1_000_000_000f64
        // );
        std::thread::sleep(Duration::from_millis(0));
    }
    Ok(())
}
