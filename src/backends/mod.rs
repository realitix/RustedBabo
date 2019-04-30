use glium::{glutin, Surface}; 

use crate::gdx::app::ApplicationListener;


pub struct GliumApplicationConfiguration {

}

pub struct GliumApplication<T: ApplicationListener> {
    listener: T,
    config: GliumApplicationConfiguration
}

impl<T: ApplicationListener> GliumApplication<T> {
    pub fn new(listener: T, config: GliumApplicationConfiguration) -> GliumApplication<T> {
        listener.create();
        GliumApplication {
            listener: listener,
            config: config
        }
    }

    pub fn run(&self) {
        let mut events_loop = glutin::EventsLoop::new();
        let wb = glutin::WindowBuilder::new();
        let cb = glutin::ContextBuilder::new();
        let display = glium::Display::new(wb, cb, &events_loop).unwrap();

        let mut closed = false;
        while !closed {
            self.listener.render(&display);

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