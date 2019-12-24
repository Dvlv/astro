use graphics::*;
use graphics::math::Matrix2d;
use piston::input::UpdateArgs;
use opengl_graphics::GlGraphics;
use super::positioned::Positioned;


#[derive(Debug)]
pub struct Asteroid {
    pub id: i64,
    pub transform: Matrix2d,
}

impl Asteroid {
    pub fn update(&mut self, _args: &UpdateArgs) {
        self.transform = self.transform.trans(0.0, -5.0);
    }

    pub fn render(&self, gl: &mut GlGraphics) {
        let rect = [10.0, 10.0, 100.0, 100.0];
        ellipse([0.0, 1.0, 0.0, 1.0], rect, self.transform, gl);
    }

}

impl Positioned for Asteroid {
    fn position_point(&self) -> [f64; 2] {
        [10.0, 10.0]
    }

    fn transform_matrix(&self) -> Matrix2d {
        self.transform
    }
}

