use minifb::{Key, Window, WindowOptions};
use std::cell::RefCell;

mod chart;

use crate::chart::Chart;
use crate::chart::Note;

const WIDTH: usize = 100;
const HEIGHT: usize = 25;

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

struct CloneHero<'a> {
    buf: &'a RefCell<Vec<u32>>,
    chart: &'a Chart,
    width: usize,
    height: usize,
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
        }
    }

    fn update(&mut self, delta_time: f32) {
        let resolution: usize = 192;

        for i in 0..20 {
            let tick = i * resolution;

            let notes = self.chart.notes.iter().filter(|v| v.tick == tick);

            for note in notes {
                self.draw_note(note.color, i, fret_to_color(note.color));
            }
        }
    }

    fn draw_note(&mut self, fret: usize, line: usize, color: u32) {
        let mut buf = self.buf.borrow_mut();

        for y in 0..5 {
            let y_offset = (fret * 5 + y) * self.width;

            for x in 0..5 {
                let x_offset = line * 5 + x;

                let pixel_index = y_offset + x_offset;

                buf[pixel_index] = color;
            }
        }
    }
}

fn main() {
    let chart = Chart::from_path("notes.chart").unwrap();

    let buffer: RefCell<Vec<u32>> = RefCell::new(vec![0; WIDTH * HEIGHT]);

    let mut clone_hero = CloneHero::new(&buffer, WIDTH, HEIGHT, &chart);

    let mut window = Window::new(
        "Radio",
        WIDTH,
        HEIGHT,
        WindowOptions {
            scale: minifb::Scale::X8,
            ..WindowOptions::default()
        },
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let mut last_frame_time = std::time::Instant::now();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let now = std::time::Instant::now();
        let delta_time = now.duration_since(last_frame_time).as_secs_f32();

        clone_hero.update(delta_time);

        window
            .update_with_buffer(&*buffer.borrow(), WIDTH, HEIGHT)
            .unwrap();

        last_frame_time = now;
    }
}
