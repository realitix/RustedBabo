use crate::gdx;

use glium::Display;
use glium::Surface;
use gdx::app::ApplicationListener;

pub struct BaboApplication {}

impl BaboApplication {
    pub fn new() -> BaboApplication {
        BaboApplication {}
    }
}

impl ApplicationListener for BaboApplication {
    fn create(&self) {

    }

    fn resize(&self, width: u32, height: u32) {

    }

    fn render(&self, display: &Display) {
        let mut target = display.draw();
        target.clear_color(0.0, 0., 1., 1.);
        target.finish().unwrap();
    }

    fn dispose(&self) {

    }
}