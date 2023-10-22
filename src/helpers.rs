use macros_rs::string;
use std::path::{Path, PathBuf};
use std::{error::Error, fs};

pub fn move_file(origin: &Path, new: &Path) -> Result<String, Box<dyn Error>> {
    fs::rename(origin, new)?;
    Ok(string!(new.file_name().unwrap().to_str().unwrap()))
}

pub fn home_dir() -> PathBuf {
    match dirs::home_dir() {
        Some(path) => path,
        None => panic!("could not get the home directory!"),
    }
}

pub fn find_files(path: &Path, contains: &str) -> Vec<PathBuf> {
    fs::read_dir(path)
        .expect("failed to read directory")
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.is_file() && path.to_string_lossy().contains(contains))
        .collect()
}
