use clap::Parser;
use file_renamer::list_directories;

mod file_operations;

fn main() {
    let path = String::from("test");
    let dirs = list_directories(&path).unwrap();
    println!("directories {:?}", dirs);
}
