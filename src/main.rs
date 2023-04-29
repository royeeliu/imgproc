use crate::proc::gray;
use crate::window::Window;

use std::collections::HashMap;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

mod draw;
mod proc;
mod window;

fn main() {
    let lena_rgba = image::load_from_memory(include_bytes!("../res/lena.jpg")).unwrap();
    let painters = gray(lena_rgba);

    let event_loop = EventLoop::new();
    let mut windows = HashMap::new();

    let mut pos = (0, 0);
    for painter in painters {
        let window = WindowBuilder::new()
            .with_inner_size(winit::dpi::PhysicalSize::new(
                painter.width(),
                painter.height(),
            ))
            .build(&event_loop)
            .unwrap();
        window.set_outer_position(winit::dpi::PhysicalPosition::new(pos.0, pos.1));
        pos.0 += window.outer_size().width;

        let window = Window::new(window, painter);
        windows.insert(window.id(), window);
    }

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
