mod model;

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use std::collections::HashMap;

use glutin_window::GlutinWindow as Window;
use graphics::color::BLACK;
use graphics::color::WHITE;
use model::Pallet;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::Size;
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    rotation: f64,  // Rotation for the square.
    pallet: Pallet,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        let rotation = self.rotation;
        let (x, y) = (self.pallet.x, self.pallet.y);
        let pallet_rectangle = rectangle::centered_square(x, y, self.pallet.size);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);

            //draw pallet
            let transform = c
                .transform;

            // Draw a box rotating around the middle of the screen.
            rectangle(WHITE, pallet_rectangle, transform, gl);
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
    let mut window: Window = WindowSettings::new("rs-squash", [400, 400])
        .samples(2)
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let inner_width: u32 = window.ctx.window().inner_size().to_logical(window.ctx.window().scale_factor()).width;
    let inner_height: u32 =  window.ctx.window().inner_size().to_logical(window.ctx.window().scale_factor()).height;

    println!("{}", window.ctx.window().inner_size().width);
    println!("{}", inner_height);
    println!("{}", window.ctx.window().scale_factor());

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        rotation: 0.0,
        pallet: Pallet {
            x: inner_width as f64 / 8.0,
            y: inner_height as f64 / 2.0,
            size: 40.0,
            color: WHITE,
            randomness: 0.0,
            divisions: 5,
        },
    };

    println!("{}", app.pallet.y);

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