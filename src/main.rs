extern crate xi_rope;

use std::env;
use std::io::Read;
use std::fs::File;

use xi_rope::rope::{Rope};
use xi_rope::tree::{Cursor};
use xi_rope::find::{find, CaseMatching};

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let search_string = &args[2];

    println!("Search for `{}` in `{}`", search_string, path);

    let mut file = File::open(path).expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let rope = Rope::from(&contents);
    let mut cursor = Cursor::new(&rope, 0);

    match find(&mut cursor, CaseMatching::CaseInsensitive, &search_string) {
        Some(start) => println!("Found string at {}", start),
        None => println!("Nothing found"),
    }
}
