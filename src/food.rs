// ************************************************************************** \\
//                                                                            \\
//                                                                            \\
//   food.rs                                                                  \\
//                                                                            \\
//   By: Thibault Cheneviere <thibault.cheneviere@telecomnancy.eu>            \\
//                                                                            \\
//   Created: 2022/11/28 15:32:12 by Thibault Cheneviere                      \\
//   Updated: 2022/11/29 17:50:08 by Thibault Cheneviere                      \\
//                                                                            \\
// ************************************************************************** \\

use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;
use rand::Rng;
use rand::rngs::ThreadRng;

pub struct Food {
	x: u32,
	y: u32,
	width: u32,
	rng: ThreadRng,
}

impl Food {
	pub fn new(x: u32, y: u32, width: u32) -> Food {
		Food{x: x, y: y, width: width, rng: rand::thread_rng()}
	}

	pub fn get_coordinate(&mut self) -> (u32, u32) {
		(self.x, self.y)
	}

	pub fn update(&mut self) {
		self.x = self.rng.gen_range(0..60);
		self.y = self.rng.gen_range(0..60);
	}

	pub fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let x = self.x * self.width;
        let y = self.y * self.width;

        let square = graphics::rectangle::square(x as f64, y as f64, self.width as f64);

        gl.draw(args.viewport(), |_c, _gl| {
            let transform = _c.transform;

            graphics::rectangle(RED, square, transform, _gl)
        });
	}
}
