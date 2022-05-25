use std::{
    fs,
    path,
    io::{
        Read,
    }
};

fn main() {
    let dummy_path = path::Path::new("dummy.txt");

    let mut f = fs::File::open(dummy_path).expect("");
    let mut all_lines : Vec<String> = Vec::new();
    let mut buffer : Vec<u8> = Vec::new();
    let mut byte : [u8; 1] = [0];

    let mut read_n = f.read(&mut byte).expect("cannot read from file");
    while read_n != 0 {
        let c = byte[0];
        if c == 0x0A { // CR
            all_lines.push(String::from_utf8(buffer).expect("invalid string"));
            buffer = Vec::new();
        } else {
            buffer.push(byte[0]);
        }
        read_n = f.read(&mut byte).expect("cannot read from file");
    }
    println!("readed: {:?}", all_lines);
}


/*
fn all_in_mem() {
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
*/
