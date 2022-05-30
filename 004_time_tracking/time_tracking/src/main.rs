use clap::Parser;
use std::fs;
use std::path;



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

    let fpath = path::Path::new(&args.dbfile.to_string());
    // try to open tasks db file:
    let mut f = fs::File::open(fpath)
        .expect("cannot open tasks db file");

    let mut content = String::new();
    f.read_to_string(&mut content)
        .expect("cannot read tasks content");

    let mut lines : Vec<&str> = content.lines().collect();

    let mut it = lines.into_iter();
    let ts = read_timespan(&mut it);

}


fn read_timespan(line_it: &mut dyn Iterator<Item=&str>) -> Option<String> {
    let opt = line_it.next();
    let mut foo = String::new();
    if Some(foo) {
        return Some("bar".to_string());
    } else {
        return None;
    }
}
