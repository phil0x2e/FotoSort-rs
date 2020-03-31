extern crate clap;
extern crate image;
extern crate image_window;
use clap::{crate_authors, crate_version, App, Arg};
use image_window::{FilterType, ImageWindow, Key, KeyRepeat, ScaleMode, WindowOptions};
use std::fs;
use std::fs::create_dir;
use std::path::Path;

struct Arguments {
    paths: Vec<String>,
    is_move: bool,
}

fn get_commandline_arguments() -> Arguments {
    let description = "A Tool for sorting images into different folders.\n\n\
                       Just pass the images as arguments and copy the currently displayed image into the folder fs1-fs5, using the keys 1-5.\n\
                       To rotate the preview by 90° press R, the image file is not rotated.\n\
                       You can delte a file by pressing Del and then Y\n\
                       To move a file you can hold down M while entering a number for a folder or if you want to move all files just start with -m.\n\
                       To Copy a file in -m mode hold down C while entering a number";
    let matches = App::new("FotoSort-rs")
        .version(crate_version!())
        .author(crate_authors!())
        .about(description)
        .arg(
            Arg::with_name("move")
                .long("move")
                .short("m")
                .help("If set files are moved by default instead of copied"),
        )
        .arg(
            Arg::with_name("FILES")
                .help("Paths to images")
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

    Arguments {
        paths: files,
        is_move: matches.is_present("move"),
    }
}

fn copy_or_move_file(from: &str, to: &str, is_move: bool) -> Result<(), std::io::Error> {
    if !Path::new(to).is_dir() {
        create_dir(to)?;
    }
    let file_name = Path::new(from)
        .file_name()
        .expect("File name ends in ..")
        .to_string_lossy();
    if is_move {
        fs::rename(from, format!("{}/{}", to, file_name)).unwrap();
        println!("Moved {} to Folder {}", file_name, to);
    } else {
        fs::copy(from, format!("{}/{}", to, file_name)).unwrap();
        println!("Copied {} to Folder {}", file_name, to);
    }
    Ok(())
}

fn check_user_input(
    window: &mut ImageWindow,
    file_paths: &mut Vec<String>,
    pos: &mut usize,
    is_move: bool,
) -> bool {
    let mut refresh = false;
    if window.is_key_pressed(Key::Key1, KeyRepeat::No) {
        let dir_name = "fs1";
        if let Err(_e) = copy_or_move_file(&file_paths[*pos], dir_name, is_move) {
            println!("Error creating directory {}", dir_name);
        }
        if is_move {
            file_paths.remove(*pos);
            refresh = true;
        }
    } else if window.is_key_pressed(Key::Key2, KeyRepeat::No) {
        let dir_name = "fs2";
        if let Err(_e) = copy_or_move_file(&file_paths[*pos], dir_name, is_move) {
            println!("Error creating directory {}", dir_name);
            file_paths.remove(*pos);
        }
        if is_move {
            file_paths.remove(*pos);
            refresh = true;
        }
    } else if window.is_key_pressed(Key::Key3, KeyRepeat::No) {
        let dir_name = "fs3";
        if let Err(_e) = copy_or_move_file(&file_paths[*pos], dir_name, is_move) {
            println!("Error creating directory {}", dir_name);
            file_paths.remove(*pos);
        }
        if is_move {
            file_paths.remove(*pos);
            refresh = true;
        }
    } else if window.is_key_pressed(Key::Key4, KeyRepeat::No) {
        let dir_name = "fs4";
        if let Err(_e) = copy_or_move_file(&file_paths[*pos], dir_name, is_move) {
            println!("Error creating directory {}", dir_name);
            file_paths.remove(*pos);
        }
        if is_move {
            file_paths.remove(*pos);
            refresh = true;
        }
    } else if window.is_key_pressed(Key::Key5, KeyRepeat::No) {
        let dir_name = "fs5";
        if let Err(_e) = copy_or_move_file(&file_paths[*pos], dir_name, is_move) {
            println!("Error creating directory {}", dir_name);
            file_paths.remove(*pos);
        }
        if is_move {
            file_paths.remove(*pos);
            refresh = true;
        }
    }

    if window.is_key_pressed(Key::R, KeyRepeat::No) {
        window.rotate90();
    }

    if window.is_key_pressed(Key::Delete, KeyRepeat::No) {
        println!(
            "Are you sure you want to delete {}? Yes: Y; No: N",
            &file_paths[*pos]
        );
        while window.is_open() {
            if window.is_key_pressed(Key::Y, KeyRepeat::No) {
                println!("Deleting {}", &file_paths[*pos]);
                fs::remove_file(&file_paths[*pos]).ok();
                file_paths.remove(*pos);
                refresh = true;
                break;
            }
            if window.is_key_pressed(Key::N, KeyRepeat::No) {
                println!("Aborted deletion");
                break;
            }
            window.fit_to_screen();
            window.update();
        }
    }

    if window.is_key_pressed(Key::Left, KeyRepeat::No) {
        refresh = true;
        if *pos != 0 {
            *pos -= 1;
        } else {
            *pos = file_paths.len() - 1;
        }
    } else if window.is_key_pressed(Key::Right, KeyRepeat::No) {
        refresh = true;
        if *pos != file_paths.len() - 1 {
            *pos += 1;
        } else {
            *pos = 0;
        }
    }
    refresh
}

fn main() {
    let arguments = get_commandline_arguments();
    let mut file_paths = arguments.paths;
    // Check if file exists and is an image, that can be opened
    file_paths = file_paths
        .into_iter()
        .filter(|fp| image::open(fp).is_ok())
        .collect();

    let mut window = ImageWindow::new(
        "FotoSort-rs",
        800,
        600,
        WindowOptions {
            resize: true,
            scale_mode: ScaleMode::Center,
            ..WindowOptions::default()
        },
        Some(FilterType::CatmullRom),
    )
    .unwrap();

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    //let images = load_images(&file_paths);
    let mut pos: usize = 0;
    if file_paths.len() > 0 {
        window.set_image_from_path_fit(&file_paths[pos]).unwrap();
        println!("Image {}/{}", 1, file_paths.len());
    }
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let mut is_move = arguments.is_move;
        if file_paths.len() > 0 {
            if window.is_key_down(Key::M) {
                is_move = true;
            }
            if window.is_key_down(Key::C) {
                is_move = false;
            }
            let refresh = check_user_input(&mut window, &mut file_paths, &mut pos, is_move);
            if refresh {
                if file_paths.len() == 0 {
                    println!("All images moved.");
                    break;
                }
                if pos >= file_paths.len() {
                    pos = file_paths.len() - 1;
                }
                println!("Image {}/{}", pos + 1, file_paths.len());
                window.set_image_from_path_fit(&file_paths[pos]).unwrap();
            }
            window.fit_to_screen();
        }
        window.update();
    }
    println!("\nBye.");
}
