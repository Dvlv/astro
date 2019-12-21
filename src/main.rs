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

pub struct Ship {
    transform: Matrix2d,
    rot_speed: f64,
    move_speed: f64,

    height: f64,
    width: f64,

    // This should probably belong to App but only the ship cares about
    // the inputs, so meh.
    button_states: ButtonStates,
}

impl Ship {
    pub fn new() -> Ship {
        // magic number from printing c.transform.
        let s_transform = [[0.00390777647518562, 0.0, -1.0], [0.0, -0.004492362982929021, 1.0]];
        Ship {
            transform : s_transform.trans(256.0, 256.0),
            rot_speed : 2.0,
            move_speed : 60.0,
            height: 20.0,
            width: 16.0,
            button_states : ButtonStates::new(),
        }
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        if self.button_states.left && !self.button_states.right {
            self.rotate_left(args);
        }

        else if self.button_states.right && !self.button_states.left {
            self.rotate_right(args);
        }

        if self.button_states.up {
            self.move_fwd(args);
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
}

impl ButtonStates {
    fn new() -> ButtonStates {
        ButtonStates {
            left : false,
            right : false,
            up : false,
            a : false,
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

        let ship_transform = self.ship.transform;
        let ship_triangle = self.ship.get_coords();

        self.gl.draw(args.viewport(), |_c, gl| {
            clear(BLACK, gl);

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

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("spinning-square", [512, 512])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        ship: Ship::new(),
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

        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}
