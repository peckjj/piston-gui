extern crate byteorder;
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
// use std::collections::HashMap;
use std::fs;
use std::time::Instant;
// use std::hash::Hash;
use byteorder::{BigEndian, LittleEndian, ReadBytesExt, ByteOrder};
use std::io::Cursor;

const NUM_RECTS: usize = 3000; // Controls number of rects, similar to resolution
const RECT_PADDING: u64 = 5; // Controls padding between rectangles
const MAX_HEIGHT_RATIO: f64 = 0.90; // Controls how tall the waves get
const MAX_PERIODS: u64 = 4; // Controls how many waves there are
const SPEED_FACTOR: f64 = 12.0; // Controls how fast the waves move

const WAV_HEADER_SYNC: u32 = 0b01010010_01001001_01000110_01000110; // 'RIFF'
const WAV_FILE_TYPE: u32 = 0b01010111_01000001_01010110_01000101; // 'WAVE'
const WAV_FORMAT_CHUNK_START: u32 = 0b01100110_01101101_01110100_00100000; // 'fmt '
const WAV_DATA_CHUNK_START: u32 = 0b01100100_01100001_01110100_01100001; // 'data'

const DATA_PATH: &str = r#"C:\Users\pecki\Desktop\Arpeggio Feature.wav"#;

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
    let timer_start = Instant::now();

    let bytes = fs::read(DATA_PATH).expect("Couldn't read file"); 

    
    let header_start = seek_wav_header(&bytes);
    if header_start == bytes.len() {panic!("Couldn't find WAVE header.")};
    let bytes = &(bytes[header_start..]);
    let mut rdr = Cursor::new(bytes);

    // Order of reading matters here. Refer to ../notes/WAV_Format.pdf
    let file_size = read_u32::<LittleEndian>(&mut rdr);
    let is_wav = read_u32::<BigEndian>(&mut rdr) == WAV_FILE_TYPE;
    let is_format_chunk_valid = read_u32::<BigEndian>(&mut rdr) == WAV_FORMAT_CHUNK_START;
    let format_chunk_len = read_u32::<LittleEndian>(&mut rdr);
    let audio_format = read_u16::<LittleEndian>(&mut rdr);
    let num_channels = read_u16::<LittleEndian>(&mut rdr);
    let sample_rate = read_u32::<LittleEndian>(&mut rdr);
    let byte_rate = read_u32::<LittleEndian>(&mut rdr);
    let block_align = read_u16::<LittleEndian>(&mut rdr);
    let bits_per_sample = read_u16::<LittleEndian>(&mut rdr);
    let is_data_chunk_valid = read_u32::<BigEndian>(&mut rdr) == WAV_DATA_CHUNK_START;
    let data_size = read_u32::<LittleEndian>(&mut rdr);

    let mut i = 0;

    while i < 10000 {
        let channel_a = read_i24::<LittleEndian>(&mut rdr);
        let channel_b = read_i24::<LittleEndian>(&mut rdr);
        println!("{}, {}", i, channel_a);

        i = i+1;
    }

    println!();
    println!("Executed in {:?}", timer_start.elapsed());

    // let header = &(bytes[start..start + 4]);

    return;
}

fn read_u32<T>(rdr: &mut Cursor<&[u8]>) -> u32
where T: ByteOrder  {
        return rdr.read_u32::<T>().unwrap();
}

fn read_u16<T>(rdr: &mut Cursor<&[u8]>) -> u16
where T: ByteOrder {
    return rdr.read_u16::<T>().unwrap();
}

fn read_u24<T>(rdr: &mut Cursor<&[u8]>) -> u32
where T: ByteOrder {
    return rdr.read_u24::<T>().unwrap();
}

fn read_i24<T>(rdr: &mut Cursor<&[u8]>) -> i32
where T: ByteOrder {
    return rdr.read_i24::<T>().unwrap();
}

fn seek_wav_header(data: &[u8]) -> usize {
    let mut i = 0;

    let mut rdr = Cursor::new(&(data[i..i+4]));

    while i < data.len() - 4 {
        if rdr.read_u32::<BigEndian>().expect("seek_wav_header(): Couldn't read data") == WAV_HEADER_SYNC {
            return i + 4;
        }

        i += 1;
        rdr = Cursor::new(&(data[i..i + 4]));
    }

    return i;
}

fn seek_header(data: &[u8]) -> usize {
    let mut i = 0;

    while i < data.len() - 1 {
        let word: u16 = (((data[i] as u16) << 8) + data[i + 1] as u16) >> 4;

        if word == 0b0000111111111111 {
            return i;
        }
        i += 1;
    }
    return data.len();
}

// fn print_header(data: &[u8], bitrate_map: HashMap<u8, HashMap<u8, HashMap<u8, u16>>>, freq_map: HashMap<u8, HashMap<u8, u32>>, mode_map: HashMap<u8, &str>, emphasis_map: HashMap<u8, &str>) {
//     let first_byte = data[0];
//     let second_byte = data[1];
//     let version = (data[1] & 0x08 ) >> 3;
//     let layer = (second_byte & 0x06) >> 1;
//     let bitrate_code = data[2] as u8 >> 4;
//     let bitrate = *(bitrate_map.get(&version).unwrap().get(&layer).unwrap().get(&bitrate_code).unwrap());
//     let frequency_code = (data[2] & 0x0c) >> 2;
//     let frequency = *(freq_map.get(&version).unwrap().get(&frequency_code).unwrap());
//     let is_padded = data[2] & 2 != 0;
//     let private_bit = data[2] & 1;
//     let mode = (data[3] & 0xc0) >> 6;
//     let emphasis = (data[3] & 3);

//     let sync_word = ( (first_byte as u16) << 8) + ( (second_byte as u16) & 0x00f0 );

//     println!("Sync Word: {:b} {:b}", sync_word >> 8, sync_word >> 4 & 0x000f);
//     println!("Version: {}", if version != 0 {"MPEG-1"} else {"MPEG-2"});
//     println!("Layer: {}", if (second_byte & 0x06) >> 1 == 1 {"Layer 3"} else {"NOT Layer 3"});
//     println!("Error Protection: {}", if second_byte & 1 != 0 {"YES"} else {"NO"});
//     println!();
//     println!("Bit Rate: {}", bitrate);
//     println!("Frequency: {}", frequency);
//     println!("Padded: {}", if is_padded {"YES"} else {"NO"});
//     println!("Private bit: {}", private_bit);
//     println!("Mode: {}", mode_map.get(&mode).unwrap());

//     if mode == 1 {
//         println!("\tIntensity Stereo: {}", if (data[3] & 0x20) != 0 {"ON"} else {"OFF"});
//         println!("\tMS Stereo: {}", if (data[3] & 0x10) != 0 {"ON"} else {"OFF"});
//     }

//     println!("Copyrighted: {}", if data[3] & 0x08 != 0 {"YES"} else {"NO"});
//     println!("Is Original: {}", if data[3] & 0x04 != 0 {"YES"} else {"NO"});
//     println!("Emphasis: {}", emphasis_map.get(&emphasis).unwrap());

// }

fn main2() {
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
