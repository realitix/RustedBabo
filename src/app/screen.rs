use glium::Surface;
use glium::Display;
use crate::gdx::app::{Screen, AppContext};


pub struct GameScreen {
}

impl GameScreen {
    pub fn new() -> GameScreen {
        GameScreen {}
    }
}

impl Screen for GameScreen {
    fn show(&self, context: &AppContext) {

    }

    fn render(&self, context: &AppContext, delta: f32) {
        let mut target = context.display.draw();
        target.clear_color(0.0, 0., 1., 1.);
        target.finish().unwrap();
    }

    fn resize(&self, context: &AppContext, width: u32, height: u32) {

    }

    fn hide(&self, context: &AppContext) {

    }

    fn dispose(&self, context: &AppContext) {
        
    }
}