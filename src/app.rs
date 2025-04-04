use std::cell::RefCell;

use crate::screen::Screen;

pub struct App<'a> {
    current_screen: RefCell<&'a dyn Screen>,
}

impl<'a> App<'a> {
    pub fn new(screen: &'a dyn Screen) -> Self {
        Self {
            current_screen: RefCell::new(screen),
        }
    }

    pub fn set_screen(&self, screen: &'a dyn Screen) {
        self.current_screen.replace(screen);
    }
}

impl<'a> Screen for App<'a> {
    fn render(&self, buffer: &mut [u8]) {
        self.current_screen.borrow().render(buffer);
    }
}
