use minifb::Key;
use minifb::{Window, WindowOptions};
use screens::{AppScreen, CloneHero};

mod app;
mod chart;
mod constant;
mod screen;
mod screens;

use crate::app::App;
use crate::constant::BUF_SIZE;
use crate::screen::Screen;

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

    let buf: &mut [u32; BUF_SIZE] = &mut [0; BUF_SIZE];

    let app_screen = &mut AppScreen {};
    let clone_hero = &mut CloneHero {};

    let app = App::new(app_screen);

    'top: while window.is_open() {
        for k in window.get_keys_released() {
            match k {
                Key::Escape => break 'top,
                Key::A => app.set_screen(app_screen),
                Key::B => app.set_screen(clone_hero),
                _ => (),
            }
        }

        app.render(buf);

        window
            .update_with_buffer(buf, constant::WIN_WIDTH, constant::WIN_HEIGHT)
            .unwrap();

        buf.fill(0);
    }
}
