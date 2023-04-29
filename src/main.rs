use crate::proc::gray;
use crate::window::Window;

use clap::Command;
use std::collections::HashMap;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

mod draw;
mod proc;
mod window;

fn cli() -> Command {
    Command::new("imgproc")
        .about("A image processing tool")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(Command::new("gray").about("convert to grayscale image"))
}

fn main() {
    let mut command = cli();
    let raw_image = image::load_from_memory(include_bytes!("../res/lena.jpg")).unwrap();
    let painters;

    let matches = cli().get_matches_mut();
    match matches.subcommand() {
        Some(("gray", _sub_matches)) => {
            painters = gray(raw_image);
        }
        _ => {
            command.print_help().unwrap();
            return;
        }
    }

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
