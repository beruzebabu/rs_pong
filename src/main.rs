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
    start_game_key: Key,
    up_key: Key,
    down_key: Key,
    speed_down_key: Key,
    speed_up_key: Key,
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
            // clear the screen.
            clear(BLACK, gl);

            // draw pallet and ball
            let transform = c
                .transform;

            rectangle(self.pallet.color, pallet_rectangle, transform, gl);
            circle_arc(self.ball.color, self.ball.size, 0.0, 360.0, ball_circle, transform, gl);

            // draw score text
            let score_text = format!("SCORE:{}", self.round);
            let transform = c.transform.trans(self.resolution[0] - ((score_text.len() + 3) as f64 * 24.0), self.resolution[1] - 24.0);

            match text(WHITE, 24, &score_text, &mut self.glyphs, transform, gl) {
                Ok(_) => {},
                Err(e) => println!("{}", e),
            }

            // draw speed text
            let speed_text = format!("SPEED:{:.2}", self.pallet.speed);
            let transform = c.transform.trans(0.0 + 24.0, self.resolution[1] - 24.0);

            match text(WHITE, 8, &speed_text, &mut self.glyphs, transform, gl) {
                Ok(_) => {},
                Err(e) => println!("{}", e),
            }

            // draw start game text if not started
            if !self.started {
                let start_text = format!("PRESS {:?} TO START!", self.start_game_key);
                let transform = c.transform.trans(self.resolution[0] / 2.0 - ((start_text.len() + 6) as f64 * 12.0) / 2.0, self.resolution[1] / 3.0);
    
                match text(WHITE, 12, &start_text, &mut self.glyphs, transform, gl) {
                    Ok(_) => {},
                    Err(e) => println!("{}", e),
                }
            }

        });
    }

    fn update(&mut self, args: &UpdateArgs, keyboard_values: &HashMap<Key, f64>) {
        // Start game on spacebar press
        if keyboard_values.contains_key(&self.start_game_key) && keyboard_values[&self.start_game_key] > 0.0 && self.started == false {
            self.started = true;
        }
        // Update pallet movement
        if keyboard_values.contains_key(&self.down_key) && keyboard_values[&self.down_key] > 0.0 && (self.pallet.y + self.pallet.size) < self.resolution[1] {
            self.pallet.y += self.pallet.speed * self.resolution[1] * args.dt;
        }
        if keyboard_values.contains_key(&self.up_key) && keyboard_values[&self.up_key] > 0.0 && (self.pallet.y - self.pallet.size) > 0.0 {
            self.pallet.y -= self.pallet.speed * self.resolution[1] * args.dt;
        }

        if keyboard_values.contains_key(&self.speed_down_key) && keyboard_values[&self.speed_down_key] > 0.0 && self.pallet.speed > 0.01 {
            self.pallet.speed -= 0.01;
        }

        if keyboard_values.contains_key(&self.speed_up_key) && keyboard_values[&self.speed_up_key] > 0.0 {
            self.pallet.speed += 0.01;
        }

        if self.started {
            if self.ball.bottom_bound() >= self.pallet.top_bound() && self.ball.top_bound() <= self.pallet.bottom_bound() && (self.pallet.x - self.pallet.size / 4.0) >= self.ball.x {
                let random_y = f64::round(thread_rng().gen_range(0.0..self.resolution[1]));
                self.ball.target = [self.resolution[0], random_y];
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
                    self.ball.x = self.ball.x - ((self.ball.speed + (1.0 + self.round as f64) / 100.0) * self.resolution[0]) * args.dt;
                    let sign = f64::signum(self.ball.y - self.ball.target[1]);
                    let distance = f64::abs(self.ball.y - self.ball.target[1]);
                    self.ball.y = self.ball.y - (sign * ((self.ball.speed + (1.0 + self.round as f64) / 100.0) * distance)) * args.dt;
                }
                Direction::Right => {
                    self.ball.x = self.ball.x + ((self.ball.speed + (1.0 + self.round as f64) / 100.0) * self.resolution[0]) * args.dt;
                    let sign = f64::signum(self.ball.y - self.ball.target[1]);
                    let distance = f64::abs(self.ball.y - self.ball.target[1]);
                    self.ball.y = self.ball.y - (sign * ((self.ball.speed + (1.0 + self.round as f64) / 100.0) * distance)) * args.dt;
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

    // Create a Glutin window.
    let mut window: Window = WindowSettings::new("rs-squash", [600, 600])
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
        start_game_key: Key::Space,
        up_key: Key::Up,
        down_key: Key::Down,
        speed_down_key: Key::Left,
        speed_up_key: Key::Right,
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