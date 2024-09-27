use crate::constant::BUF_SIZE;
use crate::screen::Screen;

pub struct TestScreen;

impl Screen for TestScreen {
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
