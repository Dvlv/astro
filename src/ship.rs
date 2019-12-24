use std::cmp;
use graphics::*;
use piston::input::UpdateArgs;
use graphics::math::Matrix2d;
use opengl_graphics::GlGraphics;
use super::positioned::Positioned;


pub struct Ship {
    pub transform: Matrix2d,
    pub rot_speed: f64,
    pub move_speed: f64,
    pub rotation: f64,

    pub height: f64,
    pub width: f64,

    pub shoot_cooldown: f64,
    pub last_shot_counter: f64,
}

impl Ship {
    pub fn new(win_width: i32, win_height: i32) -> Ship {
        let viewport = Viewport {
            rect: [0, 0, win_width, win_height],
            draw_size: [win_width as u32, win_height as u32],
            window_size: [win_width as f64, win_height as f64],
        };
        let c = Context::new_viewport(viewport);

        let s_transform = c.transform;

        Ship {
            transform: s_transform.trans(win_width as f64 / 2.0, win_height as f64 / 2.0),
            rot_speed: 2.0,
            move_speed: 120.0,
            rotation: 0.0,
            height: 20.0,
            width: 16.0,
            shoot_cooldown: 3.0,
            last_shot_counter: 0.0,
        }
    }

    pub fn render(&self, gl: &mut GlGraphics) {
        polygon([0.0, 0.0, 1.0, 1.0], &self.get_coords(), self.transform, gl);
    }

    pub fn update(&mut self, args: &UpdateArgs) {

        // lifetimes
        if self.last_shot_counter > 0.0 {
            self.last_shot_counter -= args.dt;

            // is there a better way to clamp this?
            if self.last_shot_counter < 0.0 {
                self.last_shot_counter = 0.0;
            }
        }

        let pos = self.get_position();

        if pos[1] > 512.0 || pos[1] < 0.0 {
            let percents = self.get_percent_of_screen_moved();
            self.emerge_other_side(&percents, 1.0, 1.0);
        } else if pos[0] > 512.0 || pos[0] < 0.0 {
            let percents = self.get_percent_of_screen_moved();
            self.emerge_other_side(&percents, -1.0, -1.0)
        }

    }
    pub fn emerge_other_side(&mut self, percents: &[f64; 2], x_mod: f64, y_mod: f64) {
        let viewport = Viewport {
            rect: [0, 0, 512, 512],
            draw_size: [512 as u32, 512 as u32],
            window_size: [512.0, 512.0],
        };

        let t = Context::new_viewport(viewport).transform;
        let mut y_pos = y_mod * 256.0 * percents[1];
        let mut x_pos = x_mod * 256.0 * percents[0];

        let max_screen_move = 256 - cmp::max((self.height/2.0) as i64, (self.width/2.0) as i64);
        let max_s_move_up = max_screen_move * -1;

        // clamp doesnt work =/
        if y_pos < max_s_move_up as f64 {
            y_pos = max_s_move_up as f64;
        } else if y_pos > max_screen_move as f64 {
            y_pos = max_screen_move as f64;
        }

        if x_pos < max_s_move_up as f64 {
            x_pos = max_s_move_up as f64;
        } else if x_pos > max_screen_move as f64 {
            x_pos = max_screen_move as f64;
        }

        self.transform = t.trans(256.0, 256.0).trans(x_pos, y_pos).rot_rad(self.rotation);

    }

    pub fn get_coords(&self) -> [[f64;2]; 3] {
        [[0.0, 0.0 - self.height/2.0],
        [0.0 - self.width/2.0, 0.0+self.height/2.0],
        [0.0 + self.width/2.0, 0.0+self.height/2.0]]
    }

    pub fn get_percent_of_screen_moved(&self) -> [f64; 2] {
        let t = self.transform;
        let mut cm = vec![];
        let top_p = self.get_coords()[0];
        cm.push(top_p[0]);
        cm.push(top_p[1] * -1.0);
        cm.push(1.0);

        let div = [
            t[0][0] * cm[0] + t[0][1] * cm[1] + t[0][2],
            t[1][0] * cm[0] + t[1][1] * cm[1] + t[1][2],
        ];

        div

    }

    pub fn rotate_left(&mut self, args: &UpdateArgs) {
        self.rotation -= self.rot_speed * args.dt;
        self.transform = self.transform.rot_rad(-self.rot_speed * args.dt);
    }

    pub fn rotate_right(&mut self, args: &UpdateArgs) {
        self.rotation += self.rot_speed * args.dt;
        self.transform = self.transform.rot_rad(self.rot_speed * args.dt);
    }

    pub fn move_fwd(&mut self, args: &UpdateArgs) {
        self.transform = self.transform.trans(0.0, -self.move_speed * args.dt);
    }
}

impl Positioned for Ship {
    fn position_point(&self) -> [f64; 2] {
        [0.0, 0.0]
    }

    fn transform_matrix(&self) -> Matrix2d {
        self.transform
    }
}


