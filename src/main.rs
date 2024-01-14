use minifb::{Key, Window, WindowOptions};
use std::{borrow::BorrowMut, cell::RefCell};

mod chart;

use crate::chart::Chart;

const WIDTH: usize = 100;
const HEIGHT: usize = 25;

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
        // assuming a bpm of 120 * 1000 like in chart
        let bpm: usize = 120000;
        let resolution: usize = 192;

        let nb_notes: usize = WIDTH / 5;

        // draw player notes
        self.draw_note(0, 0, 0x00FF00);
        self.draw_note(1, 0, 0xFF0000);
        self.draw_note(2, 0, 0xFFFF00);
        self.draw_note(3, 0, 0x0000FF);
        self.draw_note(4, 0, 0xFF9F00);
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
