pub trait ApplicationListener {
    fn create();
    fn resize(width: u32, height: u32);
    fn render();
    fn pause();
    fn resume();
    fn dispose();
}