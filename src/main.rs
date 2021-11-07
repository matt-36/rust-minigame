extern crate sdl2;
extern crate serde;
extern crate serde_json;

mod animation;
mod entity;
mod events;
mod game;
mod collision;

use std::path::Path;

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

fn something(ticks: u32, fpm: u32) -> i32 {
    32 * ((ticks / 100) % fpm) as i32
}

fn main() -> Result<(), String> {
    let mut MOVEMENT_SPEED = 4;
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;
    // the window is the representation of a window in your operating system,
    // however you can only manipulate properties of that window, like its size, whether it's
    // fullscreen, ... but you cannot change its content without using a Canvas or using the
    // `surface()` method.
    let mut window = video_subsystem
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
    canvas.set_integer_scale(true);
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
    let wall_tex = texture_creator.load_texture("assets/yellow.png")?;
    let timer = sdl_context.timer()?;
    let mut player = entity::Player::new(Rect::new(0, 0, 32, 32));
    let mut wall = collision::Colider::new(
        Rect::new(0, 0, 12, 80),
        Rect::new(500, 70, 12*4, 80*4)
    );

    // let fpsmanager = FPSManager::new();
    let mut fullscreen = false;
    let mut event_pump = sdl_context.event_pump()?;
    let mut frame_times = [0u128; 60];
    let mut frames = 0;
    let mut last_frame_time = SystemTime::now();
    'running: loop {
        // get the inputs here
        for event in event_pump.poll_iter() {
            if let Event::KeyDown { repeat: false, .. } | Event::KeyUp { .. } = event {
                println!("recieved {:?}", event);
            };

            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::F11),
                    ..
                } => {
                    // canvas.window_mut().set_size(1920, 1080).expect("fuck");
                    let window = canvas.window_mut();
                    window
                        .set_fullscreen(if fullscreen {
                            FullscreenType::Desktop
                        } else {
                            FullscreenType::Off
                        })
                        .expect("fuck");
                    let size = window.size();
                    canvas
                        .set_scale(
                            (size.0 as f32) / (GAME_SIZE_X as f32),
                            (size.1 as f32) / (GAME_SIZE_Y as f32),
                        )
                        .expect("fuck");
                    fullscreen = !fullscreen;
                }

                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    player.movement.set_by_key(keycode, true);
                }
                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => {
                    player.movement.set_by_key(keycode, false);
                }
                _ => {}
            }
        }

        // update the game loop here
        if player.movement.should_play_animation() {
            player.sprite.x = something(timer.ticks(), 4);
        }
        let player_speed = player.movement.get_speed();
        let ix = player_speed.0 * MOVEMENT_SPEED;
        let iy = player_speed.1 * MOVEMENT_SPEED;
        player.dest.x += ix;
        player.dest.y += iy;
        if AABB(player.dest, wall.dest) {
            player.dest.x -= ix;
            player.dest.y -= iy;
        }
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas.draw_rect(wall.dest).expect("drawing rect failed");
        canvas.draw_rect(player.dest).expect("drawing rect failed");
        wall.render(&mut canvas, &wall_tex);
        player.render(&mut canvas, &texture);
        canvas.present();
        let frame_time = SystemTime::now();
        frame_times[frames % 60] = frame_time
            .duration_since(last_frame_time)
            .unwrap()
            .as_nanos();
        frames += 1;
        last_frame_time = frame_time;
        // ((frame_times.into_iter().map(|v|frame_time - v).sum::<u128>()) * 1_000_000_000);
        // println!("{:?}", 60f64 / (frame_times.into_iter().map(|v|frame_time - v).sum::<u128>() as f64) * 1_000_000_000f64);
        println!(
            "{:?}",
            60f64 / frame_times.iter().sum::<u128>() as f64 * 1_000_000_000f64
        );
        std::thread::sleep(Duration::from_millis(0));
    }
    Ok(())
}
