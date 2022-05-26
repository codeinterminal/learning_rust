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
