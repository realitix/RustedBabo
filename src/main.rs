#[macro_use]
extern crate glium;

mod gdx;
mod app;

use gdx::app::GliumApplicationConfiguration;
use gdx::app::GliumApplication;

use app::BaboApplication;


fn main() {
    let config = GliumApplicationConfiguration{};
    let mut app = GliumApplication::new(BaboApplication::new(), config);
    app.run()
}
