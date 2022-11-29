// ************************************************************************** \\
//                                                                            \\
//                                                                            \\
//   main.rs                                                                  \\
//                                                                            \\
//   By: Thibault Cheneviere <thibault.cheneviere@telecomnancy.eu>            \\
//                                                                            \\
//   Created: 2022/11/28 12:50:10 by Thibault Cheneviere                      \\
//   Updated: 2022/11/29 12:03:37 by Thibault Cheneviere                      \\
//                                                                            \\
// ************************************************************************** \\

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::OpenGL;
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;
use piston::input::{RenderEvent, UpdateEvent};

mod snake;
mod game;
mod food;

fn main() {
	let open_gl = OpenGL::V3_2;

	let mut window: Window = WindowSettings::new("Snake game", [600, 600])
		.graphics_api(open_gl)
		.exit_on_esc(true)
		.build()
		.unwrap();

	let mut game = game::Game::new(open_gl, 10);

	let mut events = Events::new(EventSettings::new()).ups(10);

    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            game.render(&args);
        }

        if let Some(args) = e.update_args() {
            game.update(&args);
        }

		if let Some(k) = e.button_args() {
			if k.state == ButtonState::Press {
				game.pressed(&k.button);
			}
		}
    }
}
