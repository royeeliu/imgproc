use crate::view::ImageView;

use clap::{arg, Command};
use image::DynamicImage;
use proc::*;
use std::cmp::max;
use std::collections::HashMap;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{WindowBuilder, WindowButtons};

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
            Command::new("grayscale")
                .about("convert to grayscale image")
                .arg(arg!([PATH] ... "path of the image to process"))
                .arg(
                    arg!(--color_space <COLOR_SPACE>)
                        .help("HSV, HSI, HSL, YUV or RGB")
                        .require_equals(true),
                ),
        )
        .subcommand(
            Command::new("binarize")
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
        .subcommand(
            Command::new("equalize")
                .about("equalize histogram")
                .arg(arg!([PATH] ... "path of the image to process"))
                .arg(
                    arg!(--grayscale)
                        .help("equalize histogram of grayscale image")
                        .action(clap::ArgAction::SetTrue),
                )
                .arg(
                    arg!(--color_space <COLOR_SPACE>)
                        .help("HSV, HSI, HSL, YUV or RGB")
                        .require_equals(true),
                ),
        )
        .subcommand(
            Command::new("invert")
                .about("invert image")
                .arg(arg!([PATH] ... "path of the image to process")),
        )
        .subcommand(
            Command::new("complement")
                .about("show image with complementary colors")
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
        Some(("grayscale", sub_matches)) => {
            let path = sub_matches.get_one::<String>("PATH").map(|s| s.as_str());
            let color_space = sub_matches.get_one::<String>("color_space");
            drawers = grayscale(load_image(path), color_space);
        }
        Some(("binarize", sub_matches)) => {
            let path = sub_matches.get_one::<String>("PATH").map(|s| s.as_str());
            let threshold = sub_matches
                .get_one::<String>("threshold")
                .map(|s| s.parse::<u8>().unwrap());
            drawers = binarize(load_image(path), threshold);
        }
        Some(("histogram", sub_matches)) => {
            let path = sub_matches.get_one::<String>("PATH").map(|s| s.as_str());
            drawers = histogram(load_image(path));
        }
        Some(("equalize", sub_matches)) => {
            let path = sub_matches.get_one::<String>("PATH").map(|s| s.as_str());
            let grayscale = sub_matches.get_flag("grayscale");
            let color_space = sub_matches.get_one::<String>("color_space");
            drawers = equalize(load_image(path), grayscale, color_space);
        }
        Some(("invert", sub_matches)) => {
            let path = sub_matches.get_one::<String>("PATH").map(|s| s.as_str());
            drawers = invert(load_image(path));
        }
        Some(("complement", sub_matches)) => {
            let path = sub_matches.get_one::<String>("PATH").map(|s| s.as_str());
            drawers = complement(load_image(path));
        }
        _ => {
            command.print_help().unwrap();
            return;
        }
    }

    let event_loop = EventLoop::new();
    let mut views = HashMap::new();

    let mut pos = winit::dpi::PhysicalPosition::new(0, 0);
    let mut y_offset = 0;
    for drawer in drawers {
        let window = WindowBuilder::new()
            .with_inner_size(winit::dpi::PhysicalSize::new(
                drawer.width(),
                drawer.height(),
            ))
            .with_resizable(false)
            .with_enabled_buttons(WindowButtons::CLOSE)
            .build(&event_loop)
            .unwrap();
        if let Some(mon) = window.current_monitor() {
            // 如果不是第一个窗口并且按当前位置排列会超出显示器边界，则将该窗口排到下一行
            if pos.x > 0 && (pos.x + window.outer_size().width) > mon.size().width {
                pos = winit::dpi::PhysicalPosition::new(0, y_offset);
            }
        }
        window.set_outer_position(pos);
        pos.x += window.outer_size().width;
        y_offset = max(y_offset, pos.y + window.outer_size().height);

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
