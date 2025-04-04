use crate::screen::Screen;

pub struct CloneHero;

impl Screen for CloneHero {
    fn render(&self, buffer: &mut [u8]) {
        for (_i, pixel) in buffer.chunks_exact_mut(4).enumerate() {
            pixel.copy_from_slice(&[0x00, 0xff, 0x00, 0xff]);
        }
    }
}
