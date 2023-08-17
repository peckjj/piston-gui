extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use graphics::*;

const NUM_RECTS: usize = 10;
const RECT_PADDING: u8 = 5;
const MAX_HEIGHT_RATIO: f64 = 0.85;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    rotation: f64,  // Rotation for the square.
}

impl App {
    fn render(&mut self, args: &RenderArgs) {

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let window_width = args.window_size[0];

        let rect_width = ( window_width - (RECT_PADDING * (NUM_RECTS as u8 + 1)) as f64 ) / NUM_RECTS as f64;

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);

            // Draw a box rotating around the middle of the screen.
            let mut i = 0;

            while i < NUM_RECTS {
                let bottom_x = (i as f64 * rect_width) + (RECT_PADDING as f64 * (i + 1) as f64);
                let bottom_y = args.window_size[1] - 1.0;

                let top_x = (bottom_x + rect_width);
                let top_y: f64 = ((1.0 - MAX_HEIGHT_RATIO) * args.window_size[1]) * rad2ratio(calc_radians(i, self.rotation, 4));

                let square = rectangle::rectangle_by_corners(bottom_x, bottom_y, top_x, top_y);
                rectangle(RED, square, c.transform, gl);
                i += 1;
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt;
    }
}

fn create_rects() -> [types::Rectangle; NUM_RECTS] {
    let mut ret: [types::Rectangle; NUM_RECTS] = Default::default();

    let mut i = 0;

    while i < ret.len() {
        // ret[i] = rectangle::rectangle_by_corners(x0, y0, x1, y1);
        i += 1;
    }

    return ret;
}

fn calc_radians(position: usize, start_rotation: f64, max_rotations: u8) -> f64 {
    let mut radians = 0.0;

    let total_radians: f64 = 2.0 * std::f64::consts::PI * (max_rotations as f64);
    let rotation_interval = total_radians / (NUM_RECTS as f64);

    radians = (position as f64 * rotation_interval) + start_rotation;

    return radians;
}

fn rad2ratio(radians: f64) -> f64 {
    return (radians % (2.0 * std::f64::consts::PI)) / std::f64::consts::PI;
}

fn main2() {
    let rects = create_rects();

    for rect in rects {
        println!("rect: {},{},{},{}", rect[0], rect[1], rect[2], rect[3]);
    }

    println!("Total rects: {}", rects.len());
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create a Glutin window.
    let mut window: Window = WindowSettings::new("spinning-square", [800, 600])
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
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}