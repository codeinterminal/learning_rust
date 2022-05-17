use std::fs;
use std::path;
use std::io::Read;

fn main() {
    let dummy_path = path::Path::new("dummy.txt");
    let mut f = fs::File::open(dummy_path).expect("cannot find file");
    let mut s = String::new();
    f.read_to_string(&mut s).expect("i want to read the file");
    println!("File found {}", s);
}
