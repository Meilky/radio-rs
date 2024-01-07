use minifb::{Key, Window, WindowOptions};
use regex::Regex;
use std::{cell::RefCell, fs};

const WIDTH: usize = 100;
const HEIGHT: usize = 25;

struct Note {
    color: usize,
    tick: usize,
}

struct Bpm {
    bpm: usize,
    tick: usize,
}

struct Chart {
    notes: Vec<Note>,
    bpms: Vec<Bpm>,
}

impl Chart {
    fn from_path(path: &str) -> Chart {
        let file = fs::read_to_string(path).unwrap();

        let title_regex = Regex::new(r"\[(\w*)\]").unwrap();
        let note_regex = Regex::new(r"\s\s(\d*)\s=\s(\w)\s(\d)\s(\d+)").unwrap();

        let splited_file = file.split("\n");

        let mut is_notes: bool = false;

        let mut notes: Vec<Note> = vec![];

        for line in splited_file.into_iter() {
            let title_cap = title_regex.captures(line);

            if let Some(title) = title_cap {
                let (_, [name]) = title.extract();

                if name == "ExpertSingle" {
                    is_notes = true;
                } else {
                    is_notes = false;
                }

                continue;
            }

            if !is_notes {
                continue;
            }

            if line.starts_with("{") || line.starts_with("}") {
                continue;
            }

            let note_cap = note_regex.captures(line);

            if let Some(note) = note_cap {
                let (_, [tick, kind, color, _length]) = note.extract();

                if kind != "N" {
                    continue;
                }

                let color_parsed: usize = match color.parse::<usize>() {
                    Ok(parsed) => parsed,
                    Err(_) => {
                        println!("Failed to parse the color as usize");
                        continue;
                    }
                };

                if color_parsed > 5 {
                    continue;
                }

                let tick_parsed: usize = match tick.parse::<usize>() {
                    Ok(parsed) => parsed,
                    Err(_) => {
                        println!("Failed to parse the size as usize");
                        continue;
                    }
                };

                let note: Note = Note {
                    color: color_parsed,
                    tick: tick_parsed,
                };

                notes.push(note);
            }
        }

        Chart {
            notes,
            bpms: vec![],
        }
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
        let mut buf = self.buf.borrow_mut();

        for y in 0..self.height {
            for x in 0..6 {
                let pixel_index = y * self.width + x;

                let mut color = 0x000000;

                if y < 5 {
                    color = 0x00FF00;
                } else if y < 10 {
                    color = 0xFF0000;
                } else if y < 15 {
                    color = 0xFFFF00;
                } else if y < 20 {
                    color = 0x0000FF;
                } else if y < 25 {
                    color = 0xFF9F00;
                }

                buf[pixel_index as usize] = color;
            }
        }
    }
}

fn main() {
    let chart = Chart::from_path("notes.chart");

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
            .update_with_buffer(&buffer.borrow(), WIDTH, HEIGHT)
            .unwrap();

        last_frame_time = now;
    }
}
