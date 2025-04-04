pub trait Screen {
    fn render(&self, buffer: &mut [u8]);
}
