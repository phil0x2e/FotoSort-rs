extern crate clap;
extern crate image;
extern crate image_window;
use clap::{crate_authors, crate_version, App, Arg};
use image_window::{FilterType, ImageWindow, Key, KeyRepeat, ScaleMode, WindowOptions};
use std::fs;
use std::fs::create_dir;
use std::path::Path;

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

fn copy_file(from: &str, to: &str) -> Result<(), std::io::Error> {
    if !Path::new(to).is_dir() {
        create_dir(to)?;
    }
    let file_name = Path::new(from)
        .file_name()
        .expect("File name ends in ..")
        .to_string_lossy();
    fs::copy(from, format!("{}/{}", to, file_name)).unwrap();
    println!("Copied {} to Folder {}", file_name, to);
    Ok(())
}

fn check_user_input(window: &mut ImageWindow, file_paths: &[String], pos: &mut usize) {
    if window.is_key_pressed(Key::Key1, KeyRepeat::No) {
        let dir_name = "1";
        if let Err(_e) = copy_file(&file_paths[*pos], dir_name) {
            println!("Error creating directory {}", dir_name);
        }
    }
    else if window.is_key_pressed(Key::Key2, KeyRepeat::No) {
        let dir_name = "2";
        if let Err(_e) = copy_file(&file_paths[*pos], dir_name) {
            println!("Error creating directory {}", dir_name);
        }
    }
    else if window.is_key_pressed(Key::Key3, KeyRepeat::No) {
        let dir_name = "3";
        if let Err(_e) = copy_file(&file_paths[*pos], dir_name) {
            println!("Error creating directory {}", dir_name);
        }
    }
    else if window.is_key_pressed(Key::Key4, KeyRepeat::No) {
        let dir_name = "4";
        if let Err(_e) = copy_file(&file_paths[*pos], dir_name) {
            println!("Error creating directory {}", dir_name);
        }
    }
    else if window.is_key_pressed(Key::Key5, KeyRepeat::No) {
        let dir_name = "5";
        if let Err(_e) = copy_file(&file_paths[*pos], dir_name) {
            println!("Error creating directory {}", dir_name);
        }
    }

    if window.is_key_pressed(Key::Left, KeyRepeat::No) {
        if *pos != 0 {
            *pos -= 1;
        } else {
            *pos = file_paths.len() - 1;
        }
    } else if window.is_key_pressed(Key::Right, KeyRepeat::No) {
        if *pos != file_paths.len() - 1 {
            *pos += 1;
        } else {
            *pos = 0;
        }
    }
}

fn main() {
    let mut file_paths = get_commandline_arguments();
    file_paths = file_paths
        .into_iter()
        .filter(|fp| Path::new(fp).exists())
        .collect();
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

    //let images = load_images(&file_paths);
    let mut pos: usize = 0;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        check_user_input(&mut window, &file_paths, &mut pos);

        window.set_image_from_path(&file_paths[pos]).unwrap();
        window.update();
    }
    println!("\nBye.");
}
