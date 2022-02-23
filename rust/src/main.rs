extern crate ken;

use std::env;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let path = Path::new(&args[1]);
    // let display = path.display();

    println!("{}", ken::file_to_html(&path).unwrap());
}
