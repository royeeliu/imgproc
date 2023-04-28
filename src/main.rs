use image::GenericImageView;
use softbuffer::Surface;
use std::collections::HashMap;
use std::num::NonZeroU32;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

pub trait Draw {
    fn draw(&self, surface: &mut Surface);
}

pub struct Window {
    window: winit::window::Window,
    surface: Surface,
    drawer: Box<dyn Draw>,
}

impl Window {
    pub fn new(window: winit::window::Window, drawer: Box<dyn Draw>) -> Self {
        let context = unsafe { softbuffer::Context::new(&window) }.unwrap();
        let surface = unsafe { softbuffer::Surface::new(&context, &window) }.unwrap();
        Window {
            window,
            surface,
            drawer,
        }
    }

    pub fn id(&self) -> winit::window::WindowId {
        self.window.id()
    }

    pub fn draw(&mut self) {
        self.drawer.draw(&mut self.surface);
    }
}

struct DynamicImageDrawer(image::DynamicImage);
struct ImageBufferLumaU8(image::ImageBuffer<image::Luma<u8>, Vec<u8>>);

impl Draw for DynamicImageDrawer {
    fn draw(&self, surface: &mut Surface) {
        let width = self.0.width();
        let height = self.0.height();
        surface
            .resize(
                NonZeroU32::new(width).unwrap(),
                NonZeroU32::new(height).unwrap(),
            )
            .unwrap();

        let mut buffer = surface.buffer_mut().unwrap();
        for (x, y, pixel) in self.0.pixels() {
            let red = pixel.0[0] as u32;
            let green = pixel.0[1] as u32;
            let blue = pixel.0[2] as u32;
            let color = blue | (green << 8) | (red << 16);
            let index = y as usize * width as usize + x as usize;
            buffer[index] = color;
        }

        buffer.present().unwrap();
    }
}

impl Draw for ImageBufferLumaU8 {
    fn draw(&self, surface: &mut Surface) {
        let width = self.0.width();
        let height = self.0.height();
        surface
            .resize(
                NonZeroU32::new(width).unwrap(),
                NonZeroU32::new(height).unwrap(),
            )
            .unwrap();

        let mut buffer = surface.buffer_mut().unwrap();
        for y in 0..height {
            for x in 0..width {
                let luma = self.0.get_pixel(x, y).0[0] as u32;
                let color = luma | (luma << 8) | (luma << 16);
                let index = y as usize * width as usize + x as usize;
                buffer[index] = color;
            }
        }

        buffer.present().unwrap();
    }
}

fn main() {
    let lena = image::load_from_memory(include_bytes!("../res/lena.jpg")).unwrap();
    let lena_gray = lena.to_luma8();

    let event_loop = EventLoop::new();
    let mut windows = HashMap::new();

    let window = WindowBuilder::new()
        .with_inner_size(winit::dpi::PhysicalSize::new(lena.width(), lena.height()))
        .build(&event_loop)
        .unwrap();
    window.set_outer_position(winit::dpi::PhysicalPosition::new(0, 0));

    let drawer = Box::new(DynamicImageDrawer(lena));
    let window = Window::new(window, drawer);
    windows.insert(window.id(), window);

    let window = WindowBuilder::new()
        .with_inner_size(winit::dpi::PhysicalSize::new(
            lena_gray.width(),
            lena_gray.height(),
        ))
        .build(&event_loop)
        .unwrap();

    let drawer = Box::new(ImageBufferLumaU8(lena_gray));
    let window = Window::new(window, drawer);
    windows.insert(window.id(), window);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::RedrawRequested(window_id) => {
                let window = windows.get_mut(&window_id).unwrap();
                window.draw();
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } => {
                windows.remove(&window_id);
                if windows.is_empty() {
                    *control_flow = ControlFlow::Exit;
                }
            }
            _ => {}
        }
    });
}
