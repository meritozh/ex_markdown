use ex_markdown::markdown;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let path = Path::new("./test.md");
    let mut file = File::open(&path).unwrap();
    let mut document = String::new();

    file.read_to_string(&mut document).unwrap();
    let parser = markdown(&document);
    println!("{:#?}", parser.nodes);
}
