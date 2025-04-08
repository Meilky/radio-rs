use crate::screen::Screen;

pub struct DebugScreen {
    frame_count: u32,
    frames: Vec<u8>,
}

impl DebugScreen {
    pub fn new() -> Self {
        let mut frames: Vec<u8> = vec![];

        let mut next: u8 = 0;
        for _i in 0..48 {
            for _j in 0..192 {
                match next {
                    0 => {
                        frames.push(0xff);
                        frames.push(0x00);
                        frames.push(0x00);
                    }
                    1 => {
                        frames.push(0x00);
                        frames.push(0xff);
                        frames.push(0x00);
                    }
                    2 => {
                        frames.push(0x00);
                        frames.push(0x00);
                        frames.push(0xff);
                    }
                    _ => {}
                };

                next += 1;

                if next == 3 {
                    next = 0;
                }
            }

            next = 0;
        }

        Self {
            frames,
            frame_count: 0,
        }
    }
}

// real resolution: 192*48
// (5 times win resolution)

impl Screen for DebugScreen {
    fn render(&self, buffer: &mut [u8]) {
        let offset = (self.frame_count * 192 * 48) as usize;
        let mut x: usize = 0;
        let mut y: usize = 0;
        let mut row_count: u8 = 0;
        let mut column_count: u8 = 0;

        for (_i, row) in buffer.chunks_exact_mut(960 * 4).enumerate() {
            if row_count == 0 {
                let mut k = 0;

                row.fill_with(|| {
                    let mut v: u8 = 0x00;

                    if k == 4 {
                        v = 0xff;
                        k = 0;
                    } else {
                        k += 1;
                    }

                    v
                });
                row_count += 1;

                continue;
            } else if row_count == 4 {
                let mut k = 0;

                row.fill_with(|| {
                    let mut v: u8 = 0x00;

                    if k == 4 {
                        v = 0xff;
                        k = 0;
                    } else {
                        k += 1;
                    }

                    v
                });

                row_count = 0;
                y += 1;
                continue;
            }

            for (_j, pixel) in row.chunks_exact_mut(4).enumerate() {
                if column_count == 0 {
                    pixel.copy_from_slice(&[0x00, 0x00, 0x00, 0xff]);
                    column_count += 1;
                    continue;
                } else if column_count == 4 {
                    pixel.copy_from_slice(&[0x00, 0x00, 0x00, 0xff]);
                    column_count = 0;
                    x += 1;
                    continue;
                }

                column_count += 1;

                let index: usize = offset + (y * 48 * 3) + (x * 3);

                let b_pixel = self.frames.get(index..index + 3).unwrap();

                pixel.copy_from_slice(&[b_pixel[0], b_pixel[1], b_pixel[2], 0xff]);
            }

            row_count += 1;

            x = 0;
        }
    }

    fn update(&mut self) {
        self.frame_count += 1;

        if self.frame_count == (self.frames.len() / 192 / 48 / 3) as u32 {
            self.frame_count = 0;
        }
    }
}
