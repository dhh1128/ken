//! A crate for parsing Ken in Rust
#![crate_name = "ken"]
#![deny(missing_docs)]
#![deny(warnings)]

extern crate regex;

#[macro_use]
extern crate pipeline;

#[macro_use]
extern crate lazy_static;

use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

mod html;
mod generator;
mod parser;

pub use parser::{Block, ListItem, Span};

/// Converts a Ken string to HTML
pub fn to_html(text: &str) -> String {
    let result = parser::parse(text);
    html::to_html(&result)
}

/// Converts a Ken string to a tokenset of Ken items
pub fn tokenize(text: &str) -> Vec<Block> {
    parser::parse(text)
}

/// Convert tokenset of Ken items back to String
pub fn generate(x: Vec<Block>) -> String {
    generator::generate(x)
}

/// Opens a file and converts its contents to HTML
pub fn file_to_html(path: &Path) -> io::Result<String> {
    let mut file = File::open(path)?;

    let mut text = String::new();
    file.read_to_string(&mut text)?;

    let result = parser::parse(&text);
    println!("Found {} blocks", result.len());
    Ok(html::to_html(&result))
}
