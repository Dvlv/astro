use crate::graphics::Transformed;

use graphics::ellipse;
use graphics::math::Matrix2d;
use opengl_graphics::GlGraphics;
use piston::input::UpdateArgs;
use super::positioned::Positioned;


#[derive(Debug)]
pub struct Asteroid {
    pub id: i64,
    pub transform: Matrix2d,
    pub width: f64,
    pub height: f64,
    pub velocity: [f64; 2],
}

impl Asteroid {
    pub fn update(&mut self, _args: &UpdateArgs) {
        //self.transform = self.transform.trans(self.velocity[0], self.velocity[1]);

        let current_pos = self.get_position();
        let x_max = 512.0;
        let x_min = self.width/2.0;
        let y_max = 512.0 - self.height/2.0;
        let y_min = 0.0;

        if current_pos[0] > x_max || current_pos[0] < x_min {
            self.velocity[0] *= -1.0;
        }

        if current_pos[1] > y_max || current_pos[1] < y_min {
            self.velocity[1] *= -1.0;
        }
    }

    pub fn render(&self, gl: &mut GlGraphics) {
        let rect = [
            0.0, 0.0,
            self.width, self.height,
        ];
        ellipse([0.0, 1.0, 0.0, 1.0], rect, self.transform, gl);
    }

}

impl Positioned for Asteroid {
    fn position_point(&self) -> [f64; 2] {
        [self.width/2.0, -self.height/2.0]
    }

    fn transform_matrix(&self) -> Matrix2d {
        self.transform
    }
}

