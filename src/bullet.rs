use crate::graphics::Transformed;

use graphics::ellipse;
use graphics::math::Matrix2d;
use opengl_graphics::GlGraphics;
use piston::input::UpdateArgs;
use super::positioned::Positioned;

#[derive(Debug)]
pub struct Bullet {
    pub id: i64,
    pub transform: Matrix2d,
    pub lifetime: f64,
}

impl Bullet {
    pub fn update(&mut self, args: &UpdateArgs) {
        self.transform = self.transform.trans(0.0, -1.0);
        self.lifetime += args.dt;
    }

    pub fn render(&self, gl: &mut GlGraphics) {
        let rect = [0.0, 0.0, 10.0, 10.0];
        ellipse([1.0, 0.0, 0.0, 1.0], rect, self.transform, gl);
    }
}

impl Positioned for Bullet {
    fn position_point(&self) -> [f64; 2] {
        [5.0, -5.0]
    }

    fn transform_matrix(&self) -> Matrix2d {
        self.transform
    }
}


