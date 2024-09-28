use crate::constant::BUF_SIZE;
use crate::screen::Screen;

pub struct AppScreen;

impl Screen for AppScreen {
    fn render(&self, buffer: &mut [u32; BUF_SIZE]) {
        let mid = BUF_SIZE / 2;

        for i in 0..BUF_SIZE {
            if i < mid {
                buffer[i] = 0xFFFFFF;
            } else {
                buffer[i] = 0x000000;
            }
        }
    }
}
