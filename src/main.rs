use constant::BUF_SIZE;
use menu::{App, MainMenu};
use minifb::{Key, Window, WindowOptions};
use std::cell::RefCell;

mod chart;
mod constant;
mod menu;

use crate::chart::{Bpm, Chart, Note};

const WIDTH: usize = 106;
const HEIGHT: usize = 38;

fn fret_to_color(fret: usize) -> u32 {
    match fret {
        0 => 0x00FF00,
        1 => 0xFF0000,
        2 => 0xFFFF00,
        3 => 0x0000FF,
        4 => 0xFF9F00,
        _ => 0x000000,
    }
}

fn get_time_pass(tick_start: usize, tick_end: usize, bpm: usize) -> f32 {
    (((tick_end - tick_start) / 192) * 60) as f32 / (bpm / 1000) as f32
}

fn draw_note(buf: &mut Vec<u32>, fret: usize, line: usize, color: u32, width: usize) {
    for y in 0..6 {
        let fret_gap = fret * 2 * width;
        let y_offset = (fret * 6 + y) * width + fret_gap;

        for x in 0..6 {
            // remove corner
            if (y == 0 || y == 5) && (x == 0 || x == 5) {
                continue;
            }

            let x_offset = line + x;

            let pixel_index = y_offset + x_offset;

            // mid gray
            if (y == 2 || y == 3) && (x == 2 || x == 3) {
                buf[pixel_index] = 0x808080;
                continue;
            };

            buf[pixel_index] = color;
        }
    }
}

struct CloneHero<'a> {
    buf: &'a RefCell<Vec<u32>>,
    chart: &'a Chart,
    width: usize,
    height: usize,
    total_time: f32,
}

impl<'a> CloneHero<'a> {
    fn new(
        buf: &'a RefCell<Vec<u32>>,
        width: usize,
        height: usize,
        chart: &'a Chart,
    ) -> CloneHero<'a> {
        CloneHero {
            buf,
            width,
            height,
            chart,
            total_time: 0.0,
        }
    }

    fn update(&mut self, delta_time: f32) {
        let s_per_line: f32 = 0.02;

        let mut buf = self.buf.borrow_mut();

        buf.fill(0x000000);

        for i in 0..(self.width - 6) {
            let notes = self.get_notes(self.total_time + (i as f32 * s_per_line), s_per_line);

            for note in notes {
                draw_note(
                    &mut buf,
                    note.color,
                    i,
                    fret_to_color(note.color),
                    self.width,
                );
            }
        }

        self.total_time = self.total_time + delta_time;
    }

    fn get_notes(&self, start: f32, s_per_line: f32) -> Vec<&Note> {
        let mut notes: Vec<&Note> = vec![];

        for note in self.chart.notes.iter() {
            let t = self.get_tick_time(note.tick);

            if t >= start && t <= start + s_per_line {
                notes.push(note);
            }

            if t > start + s_per_line {
                break;
            }
        }

        notes
    }

    fn get_tick_time(&self, tick: usize) -> f32 {
        let mut time_offset: f32 = 0.0;

        let mut last_bpm: Option<&Bpm> = Option::None;

        for bpm in self.chart.bpms.iter() {
            if bpm.tick > tick {
                break;
            }

            match last_bpm {
                None => last_bpm = Some(bpm),
                Some(l_bpm) => {
                    time_offset += get_time_pass(l_bpm.tick, bpm.tick, l_bpm.bpm);

                    last_bpm = Some(bpm);
                }
            };
        }

        match last_bpm {
            None => (),
            Some(l_bpm) => {
                time_offset += get_time_pass(l_bpm.tick, tick, l_bpm.bpm);
            }
        };

        time_offset
    }
}

fn main() {
    let chart = Chart::from_path("notes.chart").unwrap();

    let buffer: RefCell<Vec<u32>> = RefCell::new(vec![0; WIDTH * HEIGHT]);

    let mut clone_hero = CloneHero::new(&buffer, WIDTH, HEIGHT, &chart);

    let mut window = Window::new(
        "Radio",
        constant::WIN_WIDTH,
        constant::WIN_HEIGHT,
        WindowOptions {
            scale: minifb::Scale::X2,
            ..WindowOptions::default()
        },
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let mut cur_app: &dyn App = &MainMenu {};

    let buf: &mut [u32; BUF_SIZE] = &mut [0; BUF_SIZE];

    'top: while window.is_open() {
        for k in window.get_keys_released() {
            match k {
                Key::Escape => break 'top,
                Key::A => cur_app = &menu::MainMenu {},
                Key::B => cur_app = &menu::CloneHero {},
                _ => (),
            }
        }

        cur_app.render(buf);

        window
            .update_with_buffer(buf, constant::WIN_WIDTH, constant::WIN_HEIGHT)
            .unwrap();

        buf.fill(0);
    }
}
