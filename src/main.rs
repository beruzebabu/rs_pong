mod model;

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use std::collections::HashMap;

use glutin_window::GlutinWindow as Window;
use graphics::color::WHITE;
use graphics::ellipse::circle;
use model::Pallet;
use opengl_graphics::{GlyphCache, TextureSettings};
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;
use rand::{Rng, thread_rng};
use crate::model::{Ball, Direction};

pub struct App<'a> {
    gl: GlGraphics, // OpenGL drawing backend.
    pallet: Pallet,
    ball: Ball,
    resolution: [f64; 2],
    scale_factor: f64,
    started: bool,
    round: u64,
    glyphs: GlyphCache<'a>,
}

impl App<'_> {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        let (x, y) = (self.pallet.x, self.pallet.y);
        let mut pallet_rectangle = rectangle::centered_square(x, y, self.pallet.size);
        pallet_rectangle[2] = self.pallet.size / 2 as f64;

        let ball_circle = circle(self.ball.x, self.ball.y, self.ball.size);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);

            //draw pallet and ball
            let transform = c
                .transform;

            rectangle(WHITE, pallet_rectangle, transform, gl);
            circle_arc(WHITE, self.ball.size, 0.0, 360.0, ball_circle, transform, gl);

            let transform = c.transform.trans(self.resolution[0] - 96.0, self.resolution[1] - 36.0);

            match text(WHITE, 24, &self.round.to_string(), &mut self.glyphs, transform, gl) {
                Ok(_) => {},
                Err(e) => println!("{}", e),
            }

        });
    }

    fn update(&mut self, args: &UpdateArgs, keyboard_values: &HashMap<Key, f64>) {
        // Start game on spacebar press
        if keyboard_values.contains_key(&Key::Space) && keyboard_values[&Key::Space] > 0.0 && self.started == false {
            self.started = true;
        }
        // Update pallet movement
        if keyboard_values.contains_key(&Key::S) && keyboard_values[&Key::S] > 0.0 && (self.pallet.y + self.pallet.size) < self.resolution[1] {
            self.pallet.y += self.pallet.speed * self.resolution[1] * args.dt;
        }
        if keyboard_values.contains_key(&Key::W) && keyboard_values[&Key::W] > 0.0 && (self.pallet.y - self.pallet.size) > 0.0 {
            self.pallet.y -= self.pallet.speed * self.resolution[1] * args.dt;
        }

        if self.started {
            if self.ball.bottom_bound() >= self.pallet.top_bound() && self.ball.top_bound() <= self.pallet.bottom_bound() && (self.pallet.x - self.pallet.size / 4.0) >= self.ball.x {
                self.ball.target = [self.resolution[0], self.resolution[1] / 2.0];
            }

            //Bounce ball back off of opposite wall
            if self.ball.right_bound() >= self.resolution[0] {
                let random_y = f64::round(thread_rng().gen_range(0.0..self.resolution[1]));
                self.ball.target = [0.0, random_y];
                self.round = self.round + 1;
            }

            //Reset if ball passes the pallet and hits the player side wall
            if self.ball.left_bound() <= 0.0 {
                self.reset();
            }

            match self.ball.direction() {
                Direction::Left => {
                    self.ball.x = self.ball.x - (self.ball.speed * self.resolution[0]) * args.dt;
                    let sign = f64::signum(self.ball.y - self.ball.target[1]);
                    let distance = f64::abs(self.ball.y - self.ball.target[1]);
                    self.ball.y = self.ball.y - (sign * (self.ball.speed * distance)) * args.dt;
                }
                Direction::Right => {
                    self.ball.x = self.ball.x + (self.ball.speed * self.resolution[0]) * args.dt;
                }
            }
        }
    }

    fn reset(&mut self) {
        self.round = 0;
        self.started = false;
        self.ball.x = self.resolution[0] / 2.0;
        self.ball.y = self.resolution[1] / 2.0;
        self.ball.target = [0.0, self.resolution[1] / 2.0];
        self.pallet.y =  self.resolution[1] / 2.0;
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

    let mut path = std::env::current_dir().unwrap();
    path.push("fonts");
    path.push("ARCADE_N.TTF");
    let mut glyphs = GlyphCache::new(".\\fonts\\ARCADE_N.TTF", (), TextureSettings::new()).unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        glyphs: glyphs,
        pallet: Pallet {
            x: inner_width as f64 / 8.0,
            y: inner_height as f64 / 2.0,
            size: 40.0,
            color: WHITE,
            randomness: 0.0,
            divisions: 5,
            speed: 1.0,
        },
        ball: Ball {
            x: inner_width as f64 / 2.0,
            y: inner_height as f64 / 2.0,
            size: 4.0,
            color: WHITE,
            speed: 0.5,
            target: [0.0, inner_height as f64 / 2.0],
        },
        resolution: [inner_width as f64, inner_height as f64],
        scale_factor: window.ctx.window().scale_factor(),
        started: false,
        round: 0,
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

        if let Some(args) = e.resize_args() {
            println!("{:?}", args.window_size);
            app.resolution[0] = args.window_size[0] * app.scale_factor;
            app.resolution[1] = args.window_size[1] * app.scale_factor;

            app.reset();

            /* if (app.pallet.y + app.pallet.size) > app.resolution[1] {
                app.pallet.y = app.resolution[1] - app.pallet.size;
            }

            if app.ball.target[0] > 0.0 && app.ball.target[0] < app.resolution[0] {
                app.ball.target[0] = app.resolution[0];
            } */
        }
    }
}