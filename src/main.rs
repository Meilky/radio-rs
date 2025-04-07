use pixels::{Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::keyboard::KeyCode;
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

use screens::{AppScreen, CloneHero, GranTourismoScreen, PionnerScreen};

mod app;
mod chart;
mod constant;
mod screen;
mod screens;

use crate::app::App;
use crate::screen::Screen;

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(constant::WIN_WIDTH as f64, constant::WIN_HEIGHT as f64);
        WindowBuilder::new()
            .with_title("radio-rs")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(
            constant::WIN_WIDTH as u32,
            constant::WIN_HEIGHT as u32,
            surface_texture,
        )
        .unwrap()
    };

    let mut app = App::new(Box::new(GranTourismoScreen::new()));

    let _ = event_loop.run(|event, elwt| {
        // Draw the current frame
        if let Event::WindowEvent {
            event: WindowEvent::RedrawRequested,
            ..
        } = event
        {
            app.render(pixels.frame_mut());

            if let Err(err) = pixels.render() {
                eprintln!("{}", err);
                elwt.exit();
                return;
            }
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(KeyCode::Escape) || input.close_requested() {
                elwt.exit();
                return;
            }

            if input.key_pressed(KeyCode::KeyA) {
                app.set_screen(Box::new(AppScreen {}));
            }

            if input.key_pressed(KeyCode::KeyS) {
                app.set_screen(Box::new(CloneHero {}));
            }

            if input.key_pressed(KeyCode::KeyP) {
                app.set_screen(Box::new(PionnerScreen::new()));
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                if let Err(err) = pixels.resize_surface(size.width, size.height) {
                    eprintln!("{}", err);
                    elwt.exit();
                    return;
                }
            }

            app.update();
            window.request_redraw();
        }
    });
}
