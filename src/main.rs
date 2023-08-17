extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use graphics::*;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

const NUM_RECTS: usize = 20; // Controls number of rects, similar to resolution
const RECT_PADDING: u64 = 5; // Controls padding between rectangles
const MAX_HEIGHT_RATIO: f64 = 0.90; // Controls how tall the waves get
const MAX_PERIODS: u64 = 2; // Controls how many waves there are
const SPEED_FACTOR: f64 = 3.0; // Controls how fast the waves move

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    rotation: f64,  // Rotation for the square.
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let window_width = args.window_size[0];
        // Calculate width of rect as (TotalWidthOfWindow - TotalWidthOfPadding) / NUM_RECTS
        let rect_width =
            (window_width - (RECT_PADDING * (NUM_RECTS as u64 + 1)) as f64) / NUM_RECTS as f64;

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);

            // Draw all rectangles
            let mut i = 0;

            while i < NUM_RECTS {
                // Calculate bottom corner:
                //  x = i * Width_Of_Rectangle + Total_Width_Of_Padding_To_The_Left
                //  y = Constant
                let bottom_x = (i as f64 * rect_width) + (RECT_PADDING as f64 * (i + 1) as f64);
                let bottom_y = args.window_size[1] - 1.0;

                // Calculate top corner:
                //  x = bottom_x + Width_Of_Rectangle
                //  y  = Bottom_Of_Screen - (MAX_HEIGHT_RATIO * <% of rotation out of 360 degrees> * Bottom_Of_Screen)
                let top_x = bottom_x + rect_width;
                let top_y: f64 = args.window_size[1]
                    - (MAX_HEIGHT_RATIO
                        * args.window_size[1]
                        * rad2ratio(calc_radians(i, self.rotation, MAX_PERIODS)));

                let rect = rectangle::rectangle_by_corners(bottom_x, bottom_y, top_x, top_y);
                rectangle(RED, rect, c.transform, gl);
                i += 1;
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += SPEED_FACTOR * args.dt;
    }
}

fn calc_radians(position: usize, start_rotation: f64, max_rotations: u64) -> f64 {
    let radians;

    // Calculate maximum radians as max_rotations * 2*PI
    let total_radians: f64 = 2.0 * std::f64::consts::PI * (max_rotations as f64);

    // Calculate rotation interval between rectangles as total_radians / NUM_RECTS
    let rotation_interval = total_radians / (NUM_RECTS as f64);

    // Calculate radians as position * rotation_interval + start_rotation
    radians = (position as f64 * rotation_interval) + start_rotation;

    return radians;
}

fn rad2ratio(radians: f64) -> f64 {
    // Convert radians to value between -1 and 1. Map result to range between 0 and 1.
    let ratio = map_range((-1.0, 1.0), (0.0, 1.0), radians.cos());
    return ratio;
}

// Converts a float in <from_range> to a float in <to_range>
fn map_range(from_range: (f64, f64), to_range: (f64, f64), s: f64) -> f64 {
    to_range.0 + (s - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
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
