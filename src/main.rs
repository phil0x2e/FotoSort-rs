extern crate clap;
extern crate image;
extern crate image_window;
use clap::{crate_authors, crate_version, App, Arg};
use image::DynamicImage;
use image_window::{FilterType, ImageWindow, Key, KeyRepeat, ScaleMode, WindowOptions};

fn get_commandline_arguments() -> Vec<String> {
    let description = "TODO";
    let matches = App::new("FotoSort-rs")
        .version(crate_version!())
        .author(crate_authors!())
        .about(description)
        .arg(
            Arg::with_name("FILES")
                .help("Image Files")
                .required(true)
                .min_values(1)
                .index(1),
        )
        .get_matches();
    let files: Vec<String> = matches
        .values_of("FILES")
        .unwrap()
        .into_iter()
        .map(|s| String::from(s))
        .collect();
    files
}

fn load_images(file_paths: &[String]) -> Vec<DynamicImage> {
    let mut images = Vec::with_capacity(file_paths.len());
    for fp in file_paths {
        if let Ok(img) = image::open(fp) {
            images.push(img);
        }
    }
    images
}

fn main() {
    let file_paths = get_commandline_arguments();
    let mut window = ImageWindow::new(
        "image_window example",
        800,
        600,
        WindowOptions {
            resize: true,
            scale_mode: ScaleMode::AspectRatioStretch,
            ..WindowOptions::default()
        },
        None,
    )
    .unwrap();

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let images = load_images(&file_paths);
    let mut pos: usize = 0;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        if window.is_key_pressed(Key::Left, KeyRepeat::No) {
            if pos != 0 {
                pos -= 1;
            } else {
                pos = images.len() - 1;
            }
        } else if window.is_key_pressed(Key::Right, KeyRepeat::No) {
            if pos != images.len() - 1 {
                pos += 1;
            } else {
                pos = 0;
            }
        }

        window.set_from_image(&images[pos]);
        window.update();
    }
}
