use crate::screen::Screen;

pub struct PionnerScreen;

// real resolution: 192*48
// (5 times win resolution)

impl Screen for PionnerScreen {
    fn render(&self, buffer: &mut [u8]) {
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
                    pixel.copy_from_slice(&[0x00, 0xe0, 0xff, 0xff]);
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
}
