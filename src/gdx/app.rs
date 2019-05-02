use glium::{glutin, Surface, Display}; 

pub trait ApplicationListener {
    fn create(&self, context: &AppContext);
    fn resize(&self, context: &AppContext, width: u32, height: u32);
    fn render(&self, context: &AppContext);
    fn dispose(&self, context: &AppContext);
}


pub trait Screen {
    fn show(&self, context: &AppContext);
    fn render(&self, context: &AppContext, delta: f32);
    fn resize(&self, context: &AppContext, width: u32, height: u32);
    fn hide(&self, context: &AppContext);
    fn dispose(&self, context: &AppContext);
}

pub struct AppContext {
    pub display: Display,
    pub events_loop: glutin::EventsLoop
}

pub struct GliumApplicationConfiguration {

}

pub struct GliumApplication<T: ApplicationListener> {
    listener: T,
    context: AppContext,
    config: GliumApplicationConfiguration
}

impl<T: ApplicationListener> GliumApplication<T> {
    pub fn new(listener: T, config: GliumApplicationConfiguration) -> GliumApplication<T> {
        let mut events_loop = glutin::EventsLoop::new();
        let wb = glutin::WindowBuilder::new();
        let cb = glutin::ContextBuilder::new();
        let display = glium::Display::new(wb, cb, &events_loop).unwrap();

        let context = AppContext {
            display, events_loop
        };

        listener.create(&context);
        GliumApplication {
            listener, context, config
        }
    }

    pub fn run(&mut self) {
        let mut closed = false;
        while !closed {
            self.listener.render(&self.context);

            &(self.context.events_loop).poll_events(|event| {
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
