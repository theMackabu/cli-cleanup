use crate::helpers::{find_files, home_dir, move_file};

use colored::Colorize;
use macros_rs::ternary;
use std::path::PathBuf;

fn main() {
    let desktop = home_dir().join("Desktop");
    let screenshots = find_files(&desktop, "Screenshot");
    let recordings = find_files(&desktop, "Screen Recording");
    let message = |name: &str, files: &Vec<PathBuf>| {
        format!("moved {} {} to the {name} folder.", files.len(), ternary!(files.len() > 1, "files", "file").cyan().bold())
            .bright_blue()
            .bold()
    };

    if screenshots.is_empty() && recordings.is_empty() {
        println!("{}", "no new files found to organize".yellow().bold())
    }

    if !screenshots.is_empty() {
        let screenshots_dir = home_dir().join("Pictures/Screenshots");
        println!("{}", message("screenshots", &screenshots));

        for file in screenshots {
            let file_name = file.file_name().unwrap();
            let screenshot = screenshots_dir.join(file_name);
            match move_file(&file, &screenshot) {
                Ok(name) => println!("{} {}", "+".bright_green(), format!("{name}").white()),
                Err(err) => eprintln!("{}", format!("error moving file: {:?}", err).bright_red().bold()),
            }
        }
    }

    if !recordings.is_empty() {
        let recordings_dir = home_dir().join("Movies/Screen Recordings");
        println!("{}", message("recordings", &recordings));

        for file in recordings {
            let file_name = file.file_name().unwrap();
            let recording = recordings_dir.join(file_name);
            match move_file(&file, &recording) {
                Ok(name) => println!("{} {}", "+".bright_green(), format!("{name}").white()),
                Err(err) => eprintln!("{}", format!("error moving file: {:?}", err).bright_red().bold()),
            }
        }
    }
}

mod helpers;
