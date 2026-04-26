use std::{cell::RefCell, sync::Arc};

use pixels::{Pixels, SurfaceTexture};
use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::{KeyEvent, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::{Key, NamedKey},
    window::{Window, WindowAttributes},
};

use crate::{constant, screen::Screen};

pub struct App {
    current_screen: RefCell<Box<dyn Screen>>,
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'static>>,
}

impl App {
    pub fn new(screen: Box<dyn Screen>) -> Self {
        Self {
            current_screen: RefCell::new(screen),
            window: None,
            pixels: None,
        }
    }

    pub fn set_screen(&self, screen: Box<dyn Screen>) {
        self.current_screen.replace(screen);
    }

    pub fn render(&self, buffer: &mut [u8]) {
        self.current_screen.borrow().render(buffer);
    }

    pub fn update(&self) {
        self.current_screen.borrow_mut().update();
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = {
            let size = LogicalSize::new(constant::WIN_WIDTH as f64, constant::WIN_HEIGHT as f64);

            Arc::new(
                event_loop
                    .create_window(
                        WindowAttributes::default()
                            .with_title("radio-rs")
                            .with_inner_size(size)
                            .with_min_inner_size(size),
                    )
                    .unwrap(),
            )
        };

        self.window = Some(window.clone());
        self.pixels = {
            let size = window.inner_size();
            let surface_texture = SurfaceTexture::new(size.width, size.height, window.clone());

            match Pixels::new(
                constant::WIN_WIDTH as u32,
                constant::WIN_HEIGHT as u32,
                surface_texture,
            ) {
                Ok(pixels) => {
                    // Kick off the redraw loop
                    window.request_redraw();

                    Some(pixels)
                }
                Err(err) => {
                    eprintln!("{}", err);
                    event_loop.exit();
                    None
                }
            }
        };
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        logical_key: Key::Named(NamedKey::Escape),
                        ..
                    },
                ..
            } => {
                event_loop.exit();
            }

            WindowEvent::Resized(size) => {
                if let Some(pixels) = &mut self.pixels {
                    if let Err(err) = pixels.resize_surface(size.width, size.height) {
                        eprintln!("{}", err);
                        event_loop.exit();
                    }
                }
            }

            WindowEvent::RedrawRequested => {
                let pixels = self.pixels.as_mut().unwrap().frame_mut();

                pixels.fill(0xFF);

                if let Err(err) = self.pixels.as_ref().unwrap().render() {
                    eprintln!("{}", err);
                    event_loop.exit();
                } else {
                    // Queue a redraw for the next frame
                    self.window.as_ref().unwrap().request_redraw();
                }
            }

            _ => {}
        }
    }
}
