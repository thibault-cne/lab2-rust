// ************************************************************************** \\
//                                                                            \\
//                                                                            \\
//   game.rs                                                                  \\
//                                                                            \\
//   By: Thibault Cheneviere <thibault.cheneviere@telecomnancy.eu>            \\
//                                                                            \\
//   Created: 2022/11/28 14:36:02 by Thibault Cheneviere                      \\
//   Updated: 2022/11/29 17:52:32 by Thibault Cheneviere                      \\
//                                                                            \\
// ************************************************************************** \\

use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::*;

use crate::snake::{Snake, Direction};
use crate::food::Food;

pub struct Game {
	gl: GlGraphics,
	snake: Snake,
	food: Food,
}

impl Game {
	pub fn new(open_gl: OpenGL, width: u32) -> Game {
		return Game{gl: GlGraphics::new(open_gl), snake: Snake::new(Direction::RIGHT, width), food: Food::new(1, 1, width)};
	}

	pub fn render(&mut self, args: &RenderArgs) {
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

		self.gl.draw(args.viewport(), |_c, _gl| {
			graphics::clear(BLACK, _gl);
		});

		self.snake.render(&mut self.gl, args);
		self.food.render(&mut self.gl, args);
	}

	pub fn update(&mut self, _args: &UpdateArgs) {
		if self.snake.update(self.food.get_coordinate()) {
			self.food.update();
		}
	}

	pub fn pressed(&mut self, btn: &Button) {
		let old_d: Direction = self.snake.get_dir();

		let new_d: Direction = match btn {
			&Button::Keyboard(Key::Up) => Direction::UP,
			&Button::Keyboard(Key::Down) => Direction::DOWN,
			&Button::Keyboard(Key::Left) => Direction::LEFT,
			&Button::Keyboard(Key::Right) => Direction::RIGHT,
			_ => old_d,
		};

		if self.snake.check_new_dir(new_d.clone()) {
			self.snake.change_dir(new_d.clone());
		}
	}
}
