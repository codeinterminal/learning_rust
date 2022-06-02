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
#[derive(Debug)]
struct Timespan {
    start_utc_timestamp : u64,
    duration_in_secs    : u64,

    task_name           : String,
    task_description    : String,
    tags                : Vec<String>,
}


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

    let mut tss : Vec<Timespan> = Vec::new();
    let mut xx : Option<Timespan>;
    loop {
        xx = read_timespan(&mut it);
        if xx.is_none() {
            break;
        }
        let ts = xx.unwrap();
        tss.push(ts);
    }
    print_summary(tss);
}


fn read_timespan<'a, T: Iterator< Item=&'a str>>(line_it : &mut T) -> Option<Timespan> {

    let mut tlines: Vec<String> = Vec::new();
    // min 4 lines (timestamp, duration, tags, name, description)
    for _ in 0..4 {
        let ln = line_it.next();
        match ln {
            None => {
                return None
            },
            Some(content) => {
                if content.trim.length() == 0 {
                    return None
                }
                tlines.push(content);
            }
        }
    }

    // TODO: parse to u64 the two first lines

    //
    let desc : String = String::new();
    let descL = line_it.next();
    while !descL.is_none() && Some(descL).trim().length() > 0 {
        desc.Append(Some() )
    }
}

fn print_summary(timespans: Vec<Timespan>) {
    let mut total : u64 = 0;
    for ts in timespans {
        total += ts.duration_in_secs;
        print_task(&ts);
    }

    println!("---------------------------------------");
    println!("total secs: {total}");
}

fn print_task(ts: &Timespan) {
    println!("==[ {} ] @{} ==", ts.task_name, ts.start_utc_timestamp);
    println!("{}", ts.task_description);
    println!("------- (secs: {})", ts.duration_in_secs);
    for tag in &ts.tags {
        print!("#{tag} ");
    }
    println!("");
}
