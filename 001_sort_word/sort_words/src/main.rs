use std::{
    fs,
    path,
    io::{
        Read,
    }
};

fn main() {
    let dummy_path = path::Path::new("dummy.txt");

    println!("dummy_path {}", dummy_path.display());
    let mut f = fs::File::open(dummy_path).expect("dios mio dios mio, nosta");
    let mut s = String::new();
    f.read_to_string(&mut s).expect("i want to read the file");
    // split lines
    // CR LF   (windows)
    // CR (linux)
    let mut scol : Vec<&str> = s.lines().collect();
    scol.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
    println!("File found {:?}", scol);
}

