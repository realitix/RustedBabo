use glium::{glutin, Surface}; 

use crate::gdx::app::ApplicationListener;


pub struct GliumApplicationConfiguration {

}

pub struct GliumApplication<T: ApplicationListener> {
    application_listener: T
}

impl<T: ApplicationListener> GliumApplication<T> {
    pub fn new(listener: T, config: GliumApplicationConfiguration) -> GliumApplication<T> {
        GliumApplication {
            application_listener: listener
        }
    }

    pub fn run(&self) {
        let mut events_loop = glutin::EventsLoop::new();
        let wb = glutin::WindowBuilder::new();
        let cb = glutin::ContextBuilder::new();
        let display = glium::Display::new(wb, cb, &events_loop).unwrap();

        let mut closed = false;
        while !closed {
            let mut target = display.draw();
            target.clear_color(0.0, 0., 1., 1.);
            target.finish().unwrap();

            events_loop.poll_events(|event| {
                match event {
                    glutin::Event::WindowEvent { event, ..} => match event {
                        glutin::WindowEvent::CloseRequested => closed = true,
                        _ => (),
                    },
                    _ => (),
                }
            });
        }
    }
}