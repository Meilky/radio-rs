pub trait Screen {
    fn render(&self, buffer: &mut [u8]);

    fn update(&mut self);
}
