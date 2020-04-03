extern crate clap;
extern crate image;
extern crate image_window;
use clap::{crate_authors, crate_version, App, Arg};
use image_window::{FilterType, ImageWindow, Key, KeyRepeat, ScaleMode, WindowOptions};
use std::fs;
use std::fs::create_dir;
use std::path::Path;

static USAGE: &str = "Left/Right: Previous/Next Window\n\
    1..5: Copy (default) or move (when -m is set) current image to folder 1fs..5fs\n\
    C+1..5: Copy current image to folder fs[1..5]\n\
    M+1..5: Move current image to folder fs[1..5]\n\
    R / Shift+R: Rotate preview 90° clockwise / counter clockwise (rotation is not saved)\n\
    Del: Delete current image (confirm with Y)\n\
    H: Display this usage information\n\
    Esc: Quit";

struct Config {
    file_paths: Vec<String>,
    is_move: bool,
}

fn get_commandline_arguments() -> Config {
    let description = format!("A simple command line tool for reviewing images and sorting them into different folders.\n\nUsage:\n{}", USAGE);
    let matches = App::new("FotoSort-rs")
        .version(crate_version!())
        .author(crate_authors!())
        .about(description.as_str())
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

    Config {
        file_paths: files,
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

// is_move may be different from config.is_move!!
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
        if window.is_key_down(Key::LeftShift){
            window.rotate270();
        }else {
            window.rotate90();
        }
    }
    if window.is_key_pressed(Key::H, KeyRepeat::No) {
        println!("===========\nUsage:\n{}\n==========", USAGE);
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

fn window_loop(mut window: &mut ImageWindow, config: &mut Config) -> Result<(), &'static str> {
    let mut pos: usize = 0;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let mut is_move = config.is_move;
        if config.file_paths.len() > 0 {
            if window.is_key_down(Key::M) {
                is_move = true;
            }
            if window.is_key_down(Key::C) {
                is_move = false;
            }
            let refresh = check_user_input(&mut window, &mut config.file_paths, &mut pos, is_move);
            if refresh {
                if config.file_paths.len() == 0 {
                    println!("All images moved.");
                    break;
                }
                if pos >= config.file_paths.len() {
                    pos = config.file_paths.len() - 1;
                }
                println!("Image {}/{}", pos + 1, config.file_paths.len());
                window
                    .set_image_from_path_fit(&config.file_paths[pos])
                    .unwrap();
            }
            window.fit_to_screen();
        }
        window.update();
    }
    Ok(())
}

fn main() {
    let config = get_commandline_arguments();
    // Check if file exists and is an image, that can be opened
    let mut config = Config {
        file_paths: config
            .file_paths
            .into_iter()
            .filter(|fp| image::open(fp).is_ok())
            .collect(),
        ..config
    };

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

    println!("Usage:\n{}\n==========", USAGE);

    if config.file_paths.len() > 0 {
        window
            .set_image_from_path_fit(&config.file_paths[0])
            .unwrap();
        println!("Image {}/{}", 1, config.file_paths.len());
    }
    if let Err(e) = window_loop(&mut window, &mut config) {
        println!("Error: {}", e);
    } else {
        println!("\nBye.");
    }
}
