extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

use rand::Rng;

use std::collections::HashMap;

use glutin_window::GlutinWindow as Window;
use graphics::*;
use graphics::math::Matrix2d;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;

mod positioned;
use positioned::Positioned;

mod asteroid;
use asteroid::Asteroid;
use asteroid::AsteroidStats;

mod bullet;
use bullet::Bullet;

mod ship;
use ship::Ship;


#[derive(Debug)]
pub struct ButtonStates {
    left: bool,
    right: bool,
    up: bool,
    a: bool,
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

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    ship: Ship,
    bullets: HashMap<i64, Bullet>,
    next_bullet_id: i64,
    asteroids: HashMap<i64, Asteroid>,
    next_asteroid_id: i64,
    button_states: ButtonStates,
    has_collided:  bool,
    bullets_to_remove: Vec<i64>,
    asteroids_to_remove: Vec<i64>,
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

    pub fn cull_collided_objects(&mut self) {
        for bullet_id in &self.bullets_to_remove {
            if self.bullets.contains_key(&bullet_id) {
                self.bullets.remove(&bullet_id);
                self.ship.last_shot_counter = 0.0;
            }
        }

        let mut new_asteroids: Vec<AsteroidStats> = vec![];

        for asteroid_id in self.asteroids_to_remove.iter() {
            if self.asteroids.contains_key(&asteroid_id) {
                let killed_asteroid = &self.asteroids[&asteroid_id];
                let new_width = killed_asteroid.width / 2.0;
                let new_height = killed_asteroid.height / 2.0;

                if new_width > 24.0 && new_height > 24.0 {
                    new_asteroids.push(
                        AsteroidStats {
                            transform: killed_asteroid.transform.trans(-new_width, 0.0),
                            width: new_width,
                            height: new_height,
                        }
                    );

                    new_asteroids.push(
                        AsteroidStats {
                            transform: killed_asteroid.transform.trans(new_width, 0.0),
                            width: new_width,
                            height: new_height,
                        }
                    );

                }

                self.asteroids.remove(&asteroid_id);
            }
        }

        self.asteroids_to_remove = vec![];

        for asteroid in new_asteroids {
            self.add_asteroids(&asteroid);
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

    pub fn add_asteroids(&mut self, stats: &AsteroidStats) {
        let mut rng = rand::thread_rng();
        let rand_x_vel = rng.gen_range(0.2, 0.8);
        let rand_y_vel = rng.gen_range(0.2, 0.8);

        let new_ast = Asteroid {
            id: self.next_asteroid_id,
            transform: stats.transform,
            width: stats.width,
            height: stats.height,
            velocity: [rand_x_vel, rand_y_vel],
        };

        self.asteroids.insert(self.next_asteroid_id, new_ast);
        self.next_asteroid_id += 1;
    }

    fn render(&mut self, args: &RenderArgs) {

        clear([0.0, 0.0, 0.0, 1.0], &mut self.gl);

        let asteroids: Vec<&Asteroid> = self.asteroids.iter().map(|(_, ast)| ast).collect();
        let mut asteroid_bounds: Vec<[f64; 3]> = vec![];

        for a in &asteroids {
            a.render(&mut self.gl);
            let a_pos = a.get_position();
            let a_rad = a.width/2.0;
            let a_info = [a_pos[0], a_pos[1], a_rad];
            asteroid_bounds.push(a_info);
        }

        let bullets: Vec<&Bullet> = self.bullets.iter().map(|(_, bul)| bul).collect();
        for bullet in &bullets {
            bullet.render(&mut self.gl);
        }

        self.ship.render(&mut self.gl);

        self.gl.draw(args.viewport(), |_, _| {});

        for bullet in bullets.iter() {
            let bullet_pos = bullet.get_position();
            for (idx, bound) in asteroid_bounds.iter().enumerate() {
                let dx = bullet_pos[0] - bound[0];
                let dy = bullet_pos[1] - bound[1];
                let c_squared = dx * dx + dy * dy;
                let dist_to_center = c_squared.sqrt();
                if dist_to_center < bound[2] + 5.0 {  // 5.0 is half bullet width
                    println!("collision {:?}", bullet_pos);
                    self.bullets_to_remove.push(bullet.id);
                    self.asteroids_to_remove.push(asteroids[idx].id);
                }
            }
        }
    }

    fn update(&mut self, args: &UpdateArgs) {

        self.cull_expired_bullets();
        self.cull_collided_objects();

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


            // existing asteroids
            for (_, a) in &mut self.asteroids {
                a.update(args);
            }

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


    let mut rng = rand::thread_rng();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        ship: Ship::new(win_width, win_height),
        bullets: HashMap::new(),
        next_bullet_id: 0,
        asteroids: HashMap::new(),
        next_asteroid_id: 0,
        button_states: ButtonStates::new(),
        has_collided: false,
        bullets_to_remove: vec![],
        asteroids_to_remove: vec![],
    };

    let viewport = Viewport {
        rect: [0, 0, win_width, win_height],
        draw_size: [win_width as u32, win_height as u32],
        window_size: [win_width as f64, win_height as f64],
    };
    let c = Context::new_viewport(viewport);

    let starting_asteroid = AsteroidStats {
        transform: c.transform.trans(rng.gen_range(50.0, 412.0), rng.gen_range(50.0, 412.0)),
        width: 100.0,
        height: 100.0
    };

    let starting_asteroid_2 = AsteroidStats {
        transform: c.transform.trans(rng.gen_range(50.0, 412.0), rng.gen_range(50.0, 412.0)),
        width: 100.0,
        height: 100.0
    };

    app.add_asteroids(&starting_asteroid);
    app.add_asteroids(&starting_asteroid_2);

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {

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

        if let Some(args) = e.render_args() {
            app.render(&args);
        }
    }
}
