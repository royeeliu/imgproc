use crate::draw::ImagePainter;
use softbuffer::{Context, Surface};
use winit;

pub struct Window {
    inner: winit::window::Window,
    surface: Surface,
    painter: ImagePainter,
}

impl Window {
    pub fn new(window: winit::window::Window, painter: ImagePainter) -> Self {
        let context = unsafe { Context::new(&window) }.unwrap();
        let surface = unsafe { Surface::new(&context, &window) }.unwrap();
        Window {
            inner: window,
            surface,
            painter,
        }
    }

    pub fn id(&self) -> winit::window::WindowId {
        self.inner.id()
    }

    pub fn draw(&mut self) {
        self.painter.draw(&mut self.surface);
    }
}
