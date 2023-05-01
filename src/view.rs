use crate::draw::ImageDrawer;
use softbuffer::{Context, Surface};
use winit::window::{Window, WindowId};

pub struct ImageView {
    window: Window,
    surface: Surface,
    drawer: ImageDrawer,
}

impl ImageView {
    pub fn new(window: Window, drawer: ImageDrawer) -> Self {
        let context = unsafe { Context::new(&window) }.unwrap();
        let surface = unsafe { Surface::new(&context, &window) }.unwrap();
        ImageView {
            window,
            surface,
            drawer,
        }
    }

    pub fn window_id(&self) -> WindowId {
        self.window.id()
    }

    pub fn draw(&mut self) {
        self.drawer.draw(&mut self.surface);
    }
}
