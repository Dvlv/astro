use graphics::math::{Matrix2d};

pub trait Positioned {
    fn position_point(&self) -> [f64; 2];
    fn transform_matrix(&self) -> Matrix2d;

    fn get_position(&self) -> [f64; 2]{
        let t = self.transform_matrix();
        let mut cm = vec![];
        cm.push(self.position_point()[0]);
        cm.push(self.position_point()[1] * -1.0);
        cm.push(1.0);

        let div = [
            t[0][0] * cm[0] + t[0][1] * cm[1] + t[0][2],
            t[1][0] * cm[0] + t[1][1] * cm[1] + t[1][2],
        ];

        [(512.0 - (512.0 - (div[0] * 512.0)) / 2.0), (512.0 - (512.0 - (div[1] * 512.0)) / 2.0)]
    }
}
