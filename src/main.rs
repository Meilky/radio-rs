use std::fs::File;
use winit::event_loop::EventLoop;

use screens::DebugScreen;

mod app;
mod constant;
mod screen;
mod screens;

use crate::app::App;

fn main() {
    println!("{:?}", cpal::ALL_HOSTS);

    let stream_handle = rodio::DeviceSinkBuilder::open_default_sink().unwrap();
    let mixer = stream_handle.mixer();

    let file = File::open("").unwrap();

    mixer.add(rodio::Decoder::try_from(file).unwrap());

    let event_loop = EventLoop::new().unwrap();

    let mut app = App::new(Box::new(DebugScreen::new()));

    event_loop.run_app(&mut app).unwrap();
}
