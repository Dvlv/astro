extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::{WindowSettings};
use graphics::math::{Matrix2d};
use graphics::*;

use std::collections::HashMap;
use std::cmp;
use crate::graphics::Transformed;

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

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    ship: Ship,
    bullets: HashMap<i64, Bullet>,
    next_bullet_id: i64,
    asteroids: HashMap<i64, Asteroid>,
    next_asteroid_id: i64,
    button_states: ButtonStates,
}

#[derive(Debug)]
pub struct Asteroid {
    id: i64,
    transform: Matrix2d,
}

impl Asteroid {
    pub fn update(&mut self, args: &UpdateArgs) {
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

#[derive(Debug)]
pub struct ButtonStates {
    left: bool,
    right: bool,
    up: bool,
    a: bool,
}

#[derive(Debug)]
pub struct Bullet {
    id: i64,
    transform: Matrix2d,
    lifetime: f64,
}

impl Bullet {
    pub fn update(&mut self, args: &UpdateArgs) {
        self.transform = self.transform.trans(0.0, -5.0);
        self.lifetime += args.dt;
    }

    pub fn render(&self, gl: &mut GlGraphics) {
        let rect = [0.0, 0.0, 10.0, 10.0];
        ellipse([1.0, 0.0, 0.0, 1.0], rect, self.transform, gl);
    }
}

impl Positioned for Bullet {
    fn position_point(&self) -> [f64; 2] {
        [0.0, 0.0]
    }

    fn transform_matrix(&self) -> Matrix2d {
        self.transform
    }
}

pub struct Ship {
    transform: Matrix2d,
    rot_speed: f64,
    move_speed: f64,
    rotation: f64,

    height: f64,
    width: f64,

    shoot_cooldown: f64,
    last_shot_counter: f64,
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

impl ButtonStates {
    fn new() -> ButtonStates {
        ButtonStates {
            left: false,
            right: false,
            up: false,
            a: false,
        }
    }

    pub fn reset(&mut self) {
        self.left = false;
        self.right = false;
        self.up = false;
        self.a = false
    }
}

impl App {
    pub fn cull_expired_bullets(&mut self) {
        let mut bullets_to_remove: Vec<i64> = vec![];

        for (_, bullet) in &mut self.bullets {
            let bullet_pos = bullet.get_position();
            if bullet_pos[0] > 512.0 || bullet_pos[0] < 0.0 || bullet_pos[1] > 512.0 || bullet_pos[1] < 0.0 {
                bullets_to_remove.push(bullet.id);
                self.ship.last_shot_counter = 0.0;
            } else if bullet.lifetime >= 1.2 {
                bullets_to_remove.push(bullet.id);
                self.ship.last_shot_counter = 0.0;
            }
        }

        for bullet_id in bullets_to_remove {
            self.bullets.remove(&bullet_id);
        }

    }

    pub fn shoot(&mut self) {
        let new_bullet = Bullet {
            id: self.next_bullet_id,
            transform: self.ship.transform.trans(-self.ship.width/2.0, -self.ship.height/2.0),
            lifetime: 0.0
        };

        self.bullets.insert(new_bullet.id, new_bullet);
        self.next_bullet_id += 1;

        self.ship.last_shot_counter = self.ship.shoot_cooldown;
    }

    pub fn add_asteroids(&mut self, t: Matrix2d) {
        let new_ast = Asteroid {
            id: self.next_asteroid_id,
            transform: t,
        };

        self.asteroids.insert(self.next_asteroid_id, new_ast);
        self.next_asteroid_id += 1;
    }

    fn render(&mut self, args: &RenderArgs) {

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        clear(BLACK, &mut self.gl);

        self.ship.render(&mut self.gl);

        let bullet_positions: Vec<&Bullet> = self.bullets.iter().map(|(_, bul)| bul).collect();
        for bullet in bullet_positions {
            bullet.render(&mut self.gl);
        }

        let asteroid_positions: Vec<&Asteroid> = self.asteroids.iter().map(|(_, ast)| ast).collect();
        for aster in asteroid_positions {
            aster.render(&mut self.gl);
        }

        self.gl.draw(args.viewport(), |_c, gl| {});
    }

    fn update(&mut self, args: &UpdateArgs) {

        self.cull_expired_bullets();

        // user input
        if self.button_states.left && !self.button_states.right {
            self.ship.rotate_left(args);
        }

        else if self.button_states.right && !self.button_states.left {
            self.ship.rotate_right(args);
        }

        if self.button_states.up {
            self.ship.move_fwd(args);
        }

        if self.button_states.a && self.ship.last_shot_counter <= 0.0 {
            self.shoot();
        }

        self.ship.update(args);

        // existing bullets
        for (_, bullet) in &mut self.bullets {
            bullet.update(args);
        }

    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    let win_width: i32 = 512;
    let win_height: i32 = 512;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("Asteroids", [win_width as u32, win_height as u32])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        ship: Ship::new(win_width, win_height),
        bullets: HashMap::new(),
        next_bullet_id: 0,
        asteroids: HashMap::new(),
        next_asteroid_id: 0,
        button_states: ButtonStates::new(),
    };

    let viewport = Viewport {
        rect: [0, 0, win_width, win_height],
        draw_size: [win_width as u32, win_height as u32],
        window_size: [win_width as f64, win_height as f64],
    };
    let c = Context::new_viewport(viewport);

    app.add_asteroids(c.transform);

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }
        if let Some(Button::Keyboard(key)) = e.press_args() {
            if key == Key::Left {
                app.button_states.left = true;
            } else if key == Key::Right {
                app.button_states.right = true;
            }

            if key == Key::Up {
                app.button_states.up = true;
            }

            if key == Key::A {
                app.button_states.a = true;
            }

        }

        if let Some(Button::Keyboard(key)) = e.release_args() {
            if key == Key::Left {
                app.button_states.left = false;
            } else if key == Key::Right {
                app.button_states.right = false;
            }

            if key == Key::Up {
                app.button_states.up = false;
            }

            if key == Key::A {
                app.button_states.a = false;
            }

        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}
