use clap::Parser;
use std::fs;
use std::path;
use std::io::Read;

#[derive(Parser, Debug)]
#[clap(author, version)]
struct Args {
    #[clap(short, long)]
    dbfile: String,
}

/*
    File format:
        - each field is written in a line
        - new fields will be added at the end
        - to split entries there will be an empty line

*/
/*
#[derive(Debug)]
struct Timespan {
    start_utc_timestamp u64;
    duration_in_secs    u64;

    task_name           String;
    task_description    String;
    tags                Vec<String>;
}
*/


fn main() {
    println!("Hello, world!");

    let args = Args::parse();
    println!("Input file: {}", args.dbfile);

    let dbfile = args.dbfile.to_string();
    let fpath = path::Path::new(&dbfile);

    // try to open tasks db file:
    let mut f = fs::File::open(fpath)
        .expect("cannot open tasks db file");

    let mut content = String::new();
    f.read_to_string(&mut content)
        .expect("cannot read tasks content");

    let lines : Vec<&str> = content.lines().collect();

    let mut it = lines.into_iter();

    let mut xx : Option<String>;
    loop {
        xx = read_timespan(&mut it);
        if xx.is_none() {
            break;
        }
        println!("++++ {xx:?}");
    }
}


fn read_timespan<'a, T: Iterator< Item=&'a str>>(line_it : &mut T) -> Option<String> {
    let opt = line_it.next();
    match opt {
        Some(foo) => Some("bar ".to_owned() + foo),
        None => None,
    }
}
