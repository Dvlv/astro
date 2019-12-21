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
use graphics::{Context, Viewport};

use std::collections::HashMap;
use crate::graphics::Transformed;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    ship: Ship,
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
        self.transform = self.transform.trans(0.0, -1.0);
        self.lifetime += args.dt;
    }
}

pub struct Ship {
    transform: Matrix2d,
    rot_speed: f64,
    move_speed: f64,

    height: f64,
    width: f64,

    next_bullet_id: i64,
    bullets: HashMap<i64, Bullet>,

    shoot_cooldown: f64,
    last_shot_counter: f64,

    // This should probably belong to App but only the ship cares about
    // the inputs, so meh.
    button_states: ButtonStates,
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
            move_speed: 60.0,
            height: 20.0,
            width: 16.0,
            shoot_cooldown: 3.0,
            last_shot_counter: 0.0,
            next_bullet_id: 0,
            bullets: HashMap::new(),
            button_states: ButtonStates::new(),
        }
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

        self.cull_expired_bullets();

        // existing bullets
        for (_, bullet) in &mut self.bullets {
            bullet.update(args);
        }

        // user input
        if self.button_states.left && !self.button_states.right {
            self.rotate_left(args);
        }

        else if self.button_states.right && !self.button_states.left {
            self.rotate_right(args);
        }

        if self.button_states.up {
            self.move_fwd(args);
        }

        if self.button_states.a && self.last_shot_counter == 0.0 {
            self.shoot();
        }
    }

    pub fn get_coords(&self) -> [[f64;2]; 3] {
        [[0.0, 0.0 - self.height/2.0],
        [0.0 - self.width/2.0, 0.0+self.height/2.0],
        [0.0 + self.width/2.0, 0.0+self.height/2.0]]
    }

    pub fn rotate_left(&mut self, args: &UpdateArgs) {
        self.transform = self.transform.rot_rad(-self.rot_speed * args.dt);
    }

    pub fn rotate_right(&mut self, args: &UpdateArgs) {
        self.transform = self.transform.rot_rad(self.rot_speed * args.dt);
    }

    pub fn move_fwd(&mut self, args: &UpdateArgs) {
        self.transform = self.transform.trans(0.0, -self.move_speed * args.dt);
    }

    pub fn shoot(&mut self) {
        let new_bullet = Bullet {id: self.next_bullet_id, transform: self.transform, lifetime: 0.0};
        self.bullets.insert(new_bullet.id, new_bullet);
        self.next_bullet_id += 1;

        self.last_shot_counter = self.shoot_cooldown;
    }

    pub fn cull_expired_bullets(&mut self) {
        let mut bullets_to_remove: Vec<i64> = vec![];

        for (_, bullet) in &mut self.bullets {
            if bullet.lifetime >= 3.0 {
                bullets_to_remove.push(bullet.id);
            }
        }

        for bullet_id in bullets_to_remove {
            self.bullets.remove(&bullet_id);
        }

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
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let ship_transform = self.ship.transform;
        let ship_triangle = self.ship.get_coords();

        let bullet_positions: Vec<&Bullet> = self.ship.bullets.iter().map(|(_, bul)| bul).collect();

        self.gl.draw(args.viewport(), |_c, gl| {
            clear(BLACK, gl);

            if bullet_positions.len() > 0 {
                for bullet in bullet_positions {
                    let rect = [10.0, 10.0, 10.0, 10.0];
                    ellipse(RED, rect, bullet.transform, gl);
                }
            }

            polygon(BLUE, &ship_triangle, ship_transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.ship.update(args);
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
    };


    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }
        if let Some(Button::Keyboard(key)) = e.press_args() {
            if key == Key::Left {
                app.ship.button_states.left = true;
            } else if key == Key::Right {
                app.ship.button_states.right = true;
            }

            if key == Key::Up {
                app.ship.button_states.up = true;
            }

            if key == Key::A {
                app.ship.button_states.a = true;
            }

            println!("Pressed keyboard key '{:?}'", key);
        }

        if let Some(Button::Keyboard(key)) = e.release_args() {
            if key == Key::Left {
                app.ship.button_states.left = false;
            } else if key == Key::Right {
                app.ship.button_states.right = false;
            }

            if key == Key::Up {
                app.ship.button_states.up = false;
            }

            if key == Key::A {
                app.ship.button_states.a = false;
            }

        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}