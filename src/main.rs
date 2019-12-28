extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

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
        let mut new_ast = Asteroid {
            id: self.next_asteroid_id,
            transform: t.trans(50.0, 50.0),
            width: 100.0,
            height: 100.0,
            velocity: [0.5, 0.3],
        };

        self.asteroids.insert(self.next_asteroid_id, new_ast);
        self.next_asteroid_id += 1;
    }

    fn render(&mut self, args: &RenderArgs) {

        clear([0.0, 0.0, 0.0, 1.0], &mut self.gl);

        let asteroid_positions: Vec<&Asteroid> = self.asteroids.iter().map(|(_, ast)| ast).collect();
        let mut asteroid_bounds: Vec<[f64; 4]> = vec![];

        for a in asteroid_positions {
            a.render(&mut self.gl);
            let a_pos = a.get_position();
            let bounds = [
                a_pos[0] - a.width/2.0,
                a_pos[1] - a.height/2.0,
                a_pos[0] + a.width/2.0,
                a_pos[1] + a.height/2.0
            ];
            asteroid_bounds.push(bounds);
        }

        let bullet_positions: Vec<&Bullet> = self.bullets.iter().map(|(_, bul)| bul).collect();
        for bullet in &bullet_positions {
            bullet.render(&mut self.gl);
        }

        self.ship.render(&mut self.gl);

        self.gl.draw(args.viewport(), |_, _| {});
        
        if !self.has_collided {

        for bullet in & bullet_positions {
            let bullet_pos = bullet.get_position();
            for bound in &asteroid_bounds {
                if bullet_pos[0] > bound[0] && bullet_pos[0] < bound[2] {
                    if bullet_pos[1] > bound[1] && bullet_pos[1] < bound[3] {
                        println!("colliding! {:?}, b {:?}", bound, bullet_pos);
                        self.has_collided = true;
                    }
                }
            }
        }
        }

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


        if !self.has_collided {
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
        has_collided: false,
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
