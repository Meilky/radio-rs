use crate::constant::BUF_SIZE;

pub trait App {
    fn render<'a>(&self, buffer: &'a mut [u32; BUF_SIZE]) -> &'a mut [u32; BUF_SIZE];
}

pub struct MainMenu;

impl App for MainMenu {
    fn render<'a>(&self, buffer: &'a mut [u32; BUF_SIZE]) -> &'a mut [u32; BUF_SIZE] {
        let mid = BUF_SIZE / 2;

        for i in 0..BUF_SIZE {
            if i < mid {
                buffer[i] = 0xFFFFFF;
            } else {
                buffer[i] = 0x000000;
            }
        }

        return buffer;
    }
}

pub struct CloneHero;

impl App for CloneHero {
    fn render<'a>(&self, buffer: &'a mut [u32; BUF_SIZE]) -> &'a mut [u32; BUF_SIZE] {
        for i in 0..BUF_SIZE {
            buffer[i] = 0x111111;
        }

        return buffer;
    }
}
