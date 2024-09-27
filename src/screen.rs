use crate::constant::BUF_SIZE;

pub trait Screen {
    fn render(&self, buffer: &mut [u32; BUF_SIZE]);
}
