mod screen;

use crate::gdx;

use glium::Display;

use gdx::app::ApplicationListener;
use gdx::app::Screen;
use gdx::app::GliumApplication;
use gdx::app::AppContext;
use screen::GameScreen;



pub struct BaboApplication {
    screen: Box<Screen>,
}

impl BaboApplication {
    pub fn new() -> BaboApplication {
        let screen = Box::new(GameScreen::new());
        BaboApplication { screen }
    }

    pub fn set_screen(&mut self, context: &AppContext, screen: Box<Screen>) {
        self.screen.hide(context);
        self.screen = screen;
        self.screen.show(context);
        //TODO: Size
        self.screen.resize(context, 0, 0);
    }
}

impl ApplicationListener for BaboApplication {
    fn create(&self, context: &AppContext) {

    }

    fn resize(&self, context: &AppContext, width: u32, height: u32) {
        self.screen.resize(context, width, height);
    }

    fn render(&self, context: &AppContext) {
        let delta = 1.;
        self.screen.render(context, delta);
    }

    fn dispose(&self, context: &AppContext) {
        self.screen.hide(context);
        self.screen.dispose(context);
    }
}