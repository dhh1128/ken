use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

extern crate clap;
extern crate colored;
#[macro_use]
extern crate lazy_static;
extern crate ken;
extern crate regex;
extern crate walkdir;

use regex::Regex;
use clap::{arg, Arg, Command};
use colored::*;
use walkdir::{WalkDir, DirEntry};


fn trace(msg: &str) {
    println!("{}", msg);
}

fn complain(e: &dyn std::error::Error) {
    println!("  {}: {}", "error".red(), e);
}

fn transform_file(ken_path: &PathBuf) -> bool {
    let mut html_path = PathBuf::from(&ken_path);
    println!("{} {} -> {:?}", "Transforming".green(), ken_path.to_str().expect("bad utf8 path"), html_path.to_str());
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

fn transform_dir(folder: &PathBuf) {
    for entry in WalkDir::new(folder).into_iter().filter_map(|e| e.ok()) {
        if is_ken(&entry) {
            if let Some(fname) = entry.file_name().to_str() {
                transform_file(&PathBuf::from(fname));
            }
        }
    }
}

// Use of a mod or pub mod is not actually necessary.
pub mod built_info {
    // The file has been placed there by the build script.
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

fn main() {

    let cmdline = Command::new(built_info::PKG_NAME)
        .about(built_info::PKG_DESCRIPTION)
        .version(built_info::PKG_VERSION)
        .author(built_info::PKG_AUTHORS)
        .subcommand(Command::new("standalone")
            .about("Generates standalone web pages")
            .arg(Arg::new("template").short('t'))
            .arg_required_else_help(true)
            .arg(arg!(<PATH> ... "files or folders to convert").allow_invalid_utf8(true)));

    let cmd = cmdline.get_matches();
    match cmd.subcommand() {
        Some(("standalone", args)) => {

            let mut paths = args
                .values_of_os("PATH")
                .unwrap_or_default()
                .map(PathBuf::from)
                .collect::<Vec<_>>();

            if paths.len() == 1 {
                paths.push(PathBuf::from("."))
            }

            for path in &paths {
                if let Ok(metadata) = fs::metadata(&path) {
                    if metadata.is_file() {
                        transform_file(&path);
                    } else if metadata.is_dir() {
                        transform_dir(&path);
                    }
                }
            }
        }
        _ => unreachable!(),
    }
}
