extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use std::collections::HashMap;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    rotation: f64,  // Rotation for the square.
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);
        let rotation = self.rotation;
        let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);

            let transform = c
                .transform
                .trans(x, y)
                .rot_rad(rotation)
                .trans(-25.0, -25.0);

            // Draw a box rotating around the middle of the screen.
            rectangle(WHITE, square, transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs, keyboard_values: &HashMap<Key, f64>) {
        // Rotate 2 radians per second.
        if keyboard_values.contains_key(&Key::W) && keyboard_values[&Key::W] > 0.0 {
            self.rotation += 2.0 * args.dt;
        }
        if keyboard_values.contains_key(&Key::S) && keyboard_values[&Key::S] > 0.0 {
            self.rotation -= 2.0 * args.dt;
        }

    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;
    let mut keyboard_values: HashMap<Key, f64> = HashMap::new();

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("spinning-square", [200, 200])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        rotation: 0.0,
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.press_args() {
            match args {
                Button::Keyboard(key) => {
                    keyboard_values.insert(key, 1.0);
                },
                Button::Mouse(_) => {},
                Button::Controller(_) => {},
                Button::Hat(_) => {},
            }
        };
        if let Some(args) = e.release_args() {
            match args {
                Button::Keyboard(key) => {
                    keyboard_values.insert(key, 0.0);
                },
                Button::Mouse(_) => {},
                Button::Controller(_) => {},
                Button::Hat(_) => {},
            }
        };
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args, &keyboard_values);
        }
    }
}