use crate::constant::BUF_SIZE;
use crate::screen::Screen;

pub struct CloneHero;

impl Screen for CloneHero {
    fn render(&self, buffer: &mut [u32; BUF_SIZE]) {
        for i in 0..BUF_SIZE {
            buffer[i] = 0x111111;
        }
    }
}
