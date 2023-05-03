use crate::proc::{binary, gray};
use crate::view::ImageView;

use clap::{arg, Command};
use image::DynamicImage;
use proc::histogram;
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
        .subcommand(
            Command::new("gray")
                .about("convert to grayscale image")
                .arg(arg!([PATH] ... "path of the image to process")),
        )
        .subcommand(
            Command::new("binary")
                .about("convert to binary image")
                .arg(arg!([PATH] ... "path of the image to process"))
                .arg(
                    arg!(--threshold <VALUE>)
                        .help("threshold value (0~255) for binarization.")
                        .require_equals(true),
                ),
        )
        .subcommand(
            Command::new("histogram")
                .about("show histogram of the image")
                .arg(arg!([PATH] ... "path of the image to process")),
        )
}

fn load_default_image() -> DynamicImage {
    image::load_from_memory(include_bytes!("../res/lena.jpg")).unwrap()
}

fn load_image(path: Option<&str>) -> DynamicImage {
    match path {
        Some(path) => {
            println!("Using image: {}", path);
            image::open(path).unwrap()
        }
        None => {
            println!("No image path provided, using default image.");
            load_default_image()
        }
    }
}

fn main() {
    let mut command = cli();
    let drawers;

    let matches = cli().get_matches_mut();
    match matches.subcommand() {
        Some(("gray", sub_matches)) => {
            let path = sub_matches.get_one::<String>("PATH").map(|s| s.as_str());
            drawers = gray(load_image(path));
        }
        Some(("binary", sub_matches)) => {
            let path = sub_matches.get_one::<String>("PATH").map(|s| s.as_str());
            let threshold = sub_matches
                .get_one::<String>("threshold")
                .map(|s| s.parse::<u8>().unwrap());
            drawers = binary(load_image(path), threshold);
        }
        Some(("histogram", sub_matches)) => {
            let path = sub_matches.get_one::<String>("PATH").map(|s| s.as_str());
            drawers = histogram(load_image(path));
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
