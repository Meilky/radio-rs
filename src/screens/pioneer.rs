use std::fs::File;
use std::io::prelude::*;

use crate::screen::Screen;

pub struct PionnerScreen {
    frame_count: u32,
    frames: Vec<u8>,
}

impl PionnerScreen {
    pub fn new() -> Self {
        let mut frames: Vec<u8> = vec![];

        let mut f = File::open("assets/bad_apple.rgb").unwrap();

        f.read_to_end(&mut frames).unwrap();

        Self {
            frames,
            frame_count: 0,
        }
    }
}

// real resolution: 192*48
// (5 times win resolution)

impl Screen for PionnerScreen {
    fn render(&self, buffer: &mut [u8]) {
        let mut x: usize = (self.frame_count * 192 * 48) as usize;

        let mut row_count: u8 = 0;

        for (_i, row) in buffer.chunks_exact_mut(960 * 4).enumerate() {
            let mut column_count: u8 = 0;

            for (_j, pixel) in row.chunks_exact_mut(4).enumerate() {
                if row_count == 0 {
                    pixel.copy_from_slice(&[0x00, 0x00, 0x00, 0xff]);
                    continue;
                } else if row_count == 4 {
                    pixel.copy_from_slice(&[0x00, 0x00, 0x00, 0xff]);
                    continue;
                }

                if column_count == 0 {
                    pixel.copy_from_slice(&[0x00, 0x00, 0x00, 0xff]);
                } else if column_count == 4 {
                    pixel.copy_from_slice(&[0x00, 0x00, 0x00, 0xff]);
                    column_count = 0;

                    continue;
                } else {
                    let b_pixel = self.frames.get(x * 3..(x * 3) + 3).unwrap();

                    let total: u32 = b_pixel[0] as u32 + b_pixel[1] as u32 + b_pixel[2] as u32;

                    if total >= 700 {
                        pixel.copy_from_slice(&[0x00, 0xe0, 0xff, 0xff]);
                    } else {
                        pixel.copy_from_slice(&[0x00, 0x00, 0x00, 0xff]);
                    }
                }

                if row_count == 3 && column_count == 3 {
                    x += 1;
                }

                column_count += 1;
            }

            if row_count == 4 {
                row_count = 0;
                continue;
            }

            row_count += 1;
        }
    }

    fn update(&mut self) {
        self.frame_count += 1;

        if self.frame_count == (self.frames.len() / 192 / 48 / 3) as u32 {
            self.frame_count = 0;
        }
    }
}
