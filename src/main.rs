use colored::Colorize;
use macros_rs::{string, ternary};
use std::path::{Path, PathBuf};
use std::{error::Error, fs};

fn move_file(origin: &Path, new: &Path) -> Result<String, Box<dyn Error>> {
    fs::rename(origin, new)?;
    Ok(string!(new.file_name().unwrap().to_str().unwrap()))
}

fn home_dir() -> PathBuf {
    match dirs::home_dir() {
        Some(path) => path,
        None => panic!("could not get the home directory!"),
    }
}

fn find_files(path: &Path, contains: &str) -> Vec<PathBuf> {
    fs::read_dir(path)
        .expect("failed to read directory")
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.is_file() && path.to_string_lossy().contains(contains))
        .collect()
}

fn main() {
    let desktop = home_dir().join("Desktop");
    let screenshots = find_files(&desktop, "Screenshot");
    let recordings = find_files(&desktop, "Screen Recording");

    let message = |name: &str, files: &Vec<PathBuf>| format!("moved {} {} to the {name} folder.", files.len(), ternary!(files.len() > 1, "files", "file").cyan()).bright_blue();

    if screenshots.is_empty() && recordings.is_empty() {
        println!("{}", "no new files found to organize".yellow())
    }

    if !screenshots.is_empty() {
        let screenshots_dir = home_dir().join("Pictures/Screenshots");
        println!("{}", message("screenshots", &screenshots));

        for file in screenshots {
            let file_name = file.file_name().unwrap();
            let screenshot = screenshots_dir.join(file_name);
            match move_file(&file, &screenshot) {
                Ok(name) => println!("{} {}", "+".bright_green(), format!("{name}").white()),
                Err(err) => eprintln!("{}", format!("error moving file: {:?}", err).red()),
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
                Err(err) => eprintln!("{}", format!("error moving file: {:?}", err).red()),
            }
        }
    }
}
