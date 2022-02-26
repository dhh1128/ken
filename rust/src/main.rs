use std::{env, fs};
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

extern crate colored;
#[macro_use]
extern crate lazy_static;
extern crate ken;
extern crate regex;
extern crate walkdir;

use regex::Regex;
use colored::*;
use walkdir::{WalkDir, DirEntry};


fn trace(msg: &str) {
    println!("{}", msg);
}

fn complain(e: &dyn std::error::Error) {
    println!("  {}: {}", "error".red(), e);
}

fn transform_file(ken_path: &str) -> bool {
    let mut html_path = PathBuf::from(&ken_path);
    println!("{} {} -> {:?}", "Transforming".green(), ken_path, html_path.to_str());
    html_path.set_extension("html");
    match File::create(&html_path) {
        Ok(mut html_file) => {
            trace("Opened output file.");
            match ken::file_to_html(&Path::new(ken_path)) {
                Ok(html) => {
                    trace(format!("{} bytes of HTML.", html.len()).as_str());
                    match html_file.write_all(html.as_bytes()) {
                        Ok(_) => return true,
                        Err(e) => complain(&e)
                    }
                },
                Err(e) => complain(&e)
            }
        },
        Err(e) => complain(&e)
    }
    false
}

fn is_ken(entry: &DirEntry) -> bool {
    lazy_static! {
        static ref EXT: Regex = Regex::new(".*[.](?i)ken$").unwrap();
    }
    let mut answer = false;
    if let Some(fname) = entry.file_name().to_str() {
        if EXT.is_match(&fname) {
            if let Ok(metadata) = entry.metadata() {
                answer = metadata.is_file();
            }
        }
    }
    answer
}

fn transform_dir(folder: &str) {
    for entry in WalkDir::new(folder).into_iter().filter_map(|e| e.ok()) {
        if is_ken(&entry) {
            if let Some(fname) = entry.file_name().to_str() {
                transform_file(fname);
            }
        }
    }
}

fn main() {
    let mut args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        args.push(".".to_string())
    }
    for arg in &args[1..] {
        if let Ok(metadata) = fs::metadata(&arg) {
            if metadata.is_file() {
                transform_file(&arg);
            } else if metadata.is_dir() {
                transform_dir(&arg);
            }
        }
    }
}
