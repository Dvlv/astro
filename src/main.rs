extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::{WindowSettings};

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    rotation: f64,  // Rotation for the square.
    movement: f64,
    transform: [[f64; 3]; 2],
    rot_speed: f64,
    move_speed: f64,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
        
        let ship_height: f64 = 20.0;
        let ship_base: f64 = 15.0;
        let mid: f64 = 256.0;
        let orig = 0.0;

        let rotation = self.rotation;
        let movement = self.movement;
        let ship_transform = self.transform.trans(mid, mid);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);

            // Draw a box rotating around the middle of the screen.

			polygon(BLUE, &[
                    [orig, orig - ship_height/2.0],
					[orig - ship_base/2.0, orig+ship_height/2.0],
					[orig + ship_base/2.0, orig+ship_height/2.0]
				], ship_transform, gl);
        });
    }

    fn rot_right(&mut self) {
        self.rotation += self.rot_speed;
    }

    fn rot_left(&mut self) {
        self.rotation -= self.rot_speed;
    }

    fn move_up(&mut self) {
        self.movement += self.move_speed;
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
        rotation: 0.0,
        movement: 0.0,
        transform:[[0.00390777647518562, 0.0, -1.0], [0.0, -0.004492362982929021, 1.0]], 
        rot_speed: 0.5,
        move_speed: 0.8,
    };

    let mut events = Events::new(EventSettings::new().lazy(true));
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }
		 if let Some(Button::Keyboard(key)) = e.press_args() {
            if key == Key::Left {
                app.rot_left();
            } else if key == Key::Right {
                app.rot_right();
            }

            if key == Key::Up {
                app.move_up();
            }

        };
    }
}
