use constant::BUF_SIZE;
use minifb::{Key, Window, WindowOptions};

mod chart;
mod constant;
mod screen;
mod screens;

use crate::screen::Screen;
use crate::screens::{CloneHero, TestScreen};

fn main() {
    let mut window = Window::new(
        "radio-rs",
        constant::WIN_WIDTH,
        constant::WIN_HEIGHT,
        WindowOptions {
            scale: minifb::Scale::X2,
            ..WindowOptions::default()
        },
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.set_target_fps(60);

    let mut cur_app: &dyn Screen = &screens::TestScreen {};

    let buf: &mut [u32; BUF_SIZE] = &mut [0; BUF_SIZE];

    'top: while window.is_open() {
        for k in window.get_keys_released() {
            match k {
                Key::Escape => break 'top,
                Key::A => cur_app = &TestScreen {},
                Key::B => cur_app = &CloneHero {},
                _ => (),
            }
        }

        cur_app.render(buf);

        window
            .update_with_buffer(buf, constant::WIN_WIDTH, constant::WIN_HEIGHT)
            .unwrap();

        buf.fill(0);
    }
}
