// ************************************************************************** \\
//                                                                            \\
//                                                                            \\
//   snake.rs                                                                 \\
//                                                                            \\
//   By: Thibault Cheneviere <thibault.cheneviere@telecomnancy.eu>            \\
//                                                                            \\
//   Created: 2022/11/28 13:04:16 by Thibault Cheneviere                      \\
//   Updated: 2022/11/29 17:58:01 by Thibault Cheneviere                      \\
//                                                                            \\
// ************************************************************************** \\

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;

use std::vec::Vec;

#[derive(Clone, PartialEq)]
pub enum Direction {
	UP,
	DOWN,
	LEFT,
	RIGHT,
}

pub struct Snake {
	d: Direction,
	parts: Vec<(u32, u32)>,
	width: u32,
}

impl Snake {
	pub fn new(_d: Direction, _width: u32) -> Snake {
		let mut snake: Snake = Snake{
			d: _d,
			parts: Vec::new(),
			width: _width,
		};

		snake.parts.push((0_u32, 0_u32));

		return snake;
	}

	pub fn render(&mut self, gl: &mut GlGraphics,  args: &RenderArgs) {
		use graphics::*;

		const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

		let squares: Vec<graphics::types::Rectangle> = self.parts
			.iter()
			.map(|p| (p.0 * self.width, p.1 * self.width))
			.map(|p| graphics::rectangle::square(p.0 as f64, p.1 as f64, self.width as f64))
			.collect();

		gl.draw(args.viewport(), |_c, _gl| {
			let transform = _c.transform;

			squares
				.into_iter()
				.for_each(|square| rectangle(WHITE, square, transform, _gl));
		});
	}

	pub fn update(&mut self, food: (u32, u32)) -> bool {
		let mut new_front: (u32, u32) = self.parts[0];

		match self.d {
			Direction::LEFT if new_front.0 == 0 => new_front.0 = 60,
			Direction::LEFT if new_front.0 != 0 => new_front.0 -= 1,
			Direction::RIGHT if new_front.0 == 60 => new_front.0 = 0,
			Direction::RIGHT if new_front.0 != 60 => new_front.0 += 1,
			Direction::UP if new_front.1 == 0 => new_front.1 = 60,
			Direction::UP if new_front.1 != 0 => new_front.1 -= 1,
			Direction::DOWN if new_front.1 == 60 => new_front.1 = 0,
			Direction::DOWN if new_front.1 != 60 => new_front.1 += 1,
			_ => (),
		}

		let b: bool = self.check_head_collision(food.0, food.1);

		if !b {
			self.parts.pop();
		}

		self.parts.insert(0, new_front);
		self.self_collision();

		b
	}

	pub fn change_dir(&mut self, new_d: Direction) {
		self.d = new_d;
	}

	pub fn get_dir(&mut self) -> Direction {
		self.d.clone()
	}

	pub fn check_new_dir(&mut self, dir: Direction) -> bool {
		let front: (u32, u32) = self.parts[0];

		if self.parts.len() < 2 {
			return true;
		}

		let check: (u32, u32)  = self.parts[1];

		let mut new_front = (front.0, front.1);

		match dir {
			Direction::LEFT => new_front.0 -= 1,
			Direction::RIGHT => new_front.0 += 1,
			Direction::UP => new_front.1 -= 1,
			Direction::DOWN => new_front.1 += 1,
		}

		(new_front.0 != check.0) || (new_front.1 != check.1)
	}

	pub fn check_head_collision(&mut self, x: u32, y: u32) -> bool {
		let head: (u32, u32) = self.parts[0];

		(head.0 == x) && (head.1 == y)
	}

	fn self_collision(&mut self) {
		let head: (u32, u32) = self.parts[0];
		let mut x: usize = 0;

		for i in &self.parts {
			if x != 0 && i.0 == head.0 && head.1 == i.1 {
				break;
			}
			x += 1;
		}

		self.parts.truncate(x);
	}
}
