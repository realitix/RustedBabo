use glium::Display;

pub trait ApplicationListener {
    fn create(&self);
    fn resize(&self, width: u32, height: u32);
    fn render(&self, display: &Display);
    fn dispose(&self);
}