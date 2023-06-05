use std::{fs::File, io::Write};

const CACHE_DIR: &str = ".cache/";

pub fn init_dirs() {
    let dirs_to_make = [CACHE_DIR];
    for dir in dirs_to_make.iter() {
        if !std::path::PathBuf::from(dir).exists() {
            std::fs::create_dir_all(dir).unwrap();
        }
    }
}

pub fn fetch_url(url: &str) -> String {
    let path = url_to_path(url);
    // Attempt to load from the file system.
    // If it doesn't exist, then download it.
    // Only save successful ones.
    if std::path::PathBuf::from(&path).exists() {
        println!("Reading from file system {url}");
        std::fs::read_to_string(path).unwrap()
    } else {
        println!("Fetching {url}");
        let resp = reqwest::blocking::get(url).unwrap();
        let body = resp.text().unwrap();
        let mut file = File::create(path).unwrap();
        file.write_all(body.as_bytes()).unwrap();
        body
    }
}

fn url_to_path(url: &str) -> String {
    let mut path = String::from(CACHE_DIR);
    path.push_str(&url.replace("/", "").replace(":", "").replace(".", ""));
    path
}
