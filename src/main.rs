#[macro_use]
extern crate glium;

mod gdx;
mod backends;
mod app;


fn main() {
    let config = backends::GliumApplicationConfiguration{};
    let app = backends::GliumApplication::new(app::BaboApplication::new(), config);
    app.run()
}
