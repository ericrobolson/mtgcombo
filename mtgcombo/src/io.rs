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
        let timeout = std::time::Duration::from_secs(30);
        let client = reqwest::blocking::Client::builder()
            .timeout(timeout)
            .build()
            .unwrap();

        let resp = client.get(url).send().unwrap();
        if !resp.status().is_success() {
            panic!("Failed to fetch {url}: {:#?}", resp);

        }
        let body = resp.bytes().unwrap();

        let is_zip = url.ends_with(".gz");
        let body = if is_zip {
            use std::io::prelude::*;
            use flate2::read::GzDecoder;
            let mut d = GzDecoder::new(body.as_ref());
            let mut s = String::new();
            d.read_to_string(&mut s).unwrap();
            s.into_bytes()
        } else {
            body.to_vec()
        };

        let mut file = File::create(path).unwrap();
        file.write_all(&body).unwrap();

        String::from_utf8(body.to_vec()).unwrap()
    }
}

fn url_to_path(url: &str) -> String {
    let mut path = String::from(CACHE_DIR);

    path.push_str(&url.replace("/", "").replace(":", "").replace(".", ""));

    path
}
