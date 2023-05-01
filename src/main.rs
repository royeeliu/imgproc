use crate::proc::{binary, gray};
use crate::view::ImageView;

use clap::{arg, Command};
use std::collections::HashMap;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

mod alg;
mod draw;
mod proc;
mod view;

fn cli() -> Command {
    Command::new("imgproc")
        .about("A image processing tool")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(Command::new("gray").about("convert to grayscale image"))
        .subcommand(
            Command::new("binary").about("convert to binary image").arg(
                arg!(--threshold <VALUE>)
                    .help("threshold value (0~255) for binarization.")
                    .require_equals(true),
            ),
        )
}

fn main() {
    let mut command = cli();
    let image = image::load_from_memory(include_bytes!("../res/lena.jpg")).unwrap();
    let drawers;

    let matches = cli().get_matches_mut();
    match matches.subcommand() {
        Some(("gray", _sub_matches)) => {
            drawers = gray(image);
        }
        Some(("binary", sub_matches)) => {
            let threshold = sub_matches
                .get_one::<String>("threshold")
                .map(|s| s.parse::<u8>().unwrap());
            drawers = binary(image, threshold);
        }
        _ => {
            command.print_help().unwrap();
            return;
        }
    }

    let event_loop = EventLoop::new();
    let mut views = HashMap::new();

    let mut pos = (0, 0);
    for drawer in drawers {
        let window = WindowBuilder::new()
            .with_inner_size(winit::dpi::PhysicalSize::new(
                drawer.width(),
                drawer.height(),
            ))
            .build(&event_loop)
            .unwrap();
        window.set_outer_position(winit::dpi::PhysicalPosition::new(pos.0, pos.1));
        pos.0 += window.outer_size().width;

        let view = ImageView::new(window, drawer);
        views.insert(view.window_id(), view);
    }

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::RedrawRequested(window_id) => {
                let view = views.get_mut(&window_id).unwrap();
                view.draw();
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } => {
                views.remove(&window_id);
                if views.is_empty() {
                    *control_flow = ControlFlow::Exit;
                }
            }
            _ => {}
        }
    });
}
