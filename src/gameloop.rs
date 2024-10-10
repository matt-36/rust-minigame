use std::{cell::RefCell, rc::Rc, time::SystemTime};

use sdl2::{event::Event, keyboard::Keycode, render::{TextureCreator, WindowCanvas}, ttf::Sdl2TtfContext, video::{FullscreenType, WindowContext}, Sdl};

use crate::game::Game;
#[cfg(target_family = "wasm")]
use crate::emscripten;

// ctx: Rc<RefCell<Sdl>>, rect: Rc<RefCell<Rect>>, canvas: Rc<RefCell<WindowCanvas>>
pub fn game_loop(
    sdl_context: Rc<RefCell<Sdl>>, 
    canvas: Rc<RefCell<WindowCanvas>>, 
    game: Rc<RefCell<Game<'_>>>, 
    _ttf_context: Rc<RefCell<Sdl2TtfContext>>
    // texture_creator: TextureCreator<WindowContext>

) -> impl FnMut() + '_ {
    move || {
        let timer = sdl_context.borrow_mut().timer().unwrap();

        let mut fullscreen = false;
        let mut event_pump = sdl_context.borrow_mut().event_pump().unwrap();
        // get the inputs here
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => unsafe { emscripten::emscripten_cancel_main_loop() },
                Event::KeyDown {
                    keycode: Some(Keycode::F11),
                    ..
                } => {
                    // canvas.window_mut().set_size(1920, 1080).expect("fuck");
                    canvas.borrow_mut()
                        .window_mut()
                        .set_fullscreen(if !fullscreen {
                            FullscreenType::Desktop
                        } else {
                            FullscreenType::Off
                        })
                        .expect("failed changing to or from fullscreen");
                    let size = canvas.borrow_mut().window().size();
                    canvas.borrow_mut()
                        .set_scale(
                            (size.0 as f32) / (crate::GAME_SIZE_X as f32),
                            (size.1 as f32) / (crate::GAME_SIZE_Y as f32),
                        )
                        .expect("failed resizing the canvas");
                    fullscreen ^= true;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::F7),
                    ..
                } => game.borrow_mut().togglehitboxes(),
                Event::KeyDown { .. } | Event::KeyUp { .. } => game.borrow_mut().handle(&event),
                _ => {} // direction works but not movement speed
            } // nop no move
        }

        game.borrow_mut().tick(timer.ticks());
        // this is where it belongs because it should happen every frame
        // and to make it independent of time we should have it be
        // multiplied by the duration since last update
        // though we can have a custom function that takes game
        // or could have it be a function on game that is like... a tick advance
        // yeah
        // before i would check collision before rendering
        //  ok lets do that
        canvas.borrow_mut().clear();
        game.borrow_mut().update(&mut canvas.borrow_mut(), &_ttf_context.borrow_mut())
            .expect("game failed to update/render"); // it happens here cos the function doesnt render players yet
                                                     //still nothing :(
        canvas.borrow_mut().present(); // its cos we never define the x and y of the player
    }
}