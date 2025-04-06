use std::cell::RefCell;

use crate::screen::Screen;

pub struct App {
    current_screen: RefCell<Box<dyn Screen>>,
}

impl App {
    pub fn new(screen: Box<dyn Screen>) -> Self {
        Self {
            current_screen: RefCell::new(screen),
        }
    }

    pub fn set_screen(&self, screen: Box<dyn Screen>) {
        self.current_screen.replace(screen);
    }
}

impl Screen for App {
    fn render(&self, buffer: &mut [u8]) {
        self.current_screen.borrow().render(buffer);
    }

    fn update(&mut self) {
        (*self.current_screen.borrow_mut()).update();
    }
}
