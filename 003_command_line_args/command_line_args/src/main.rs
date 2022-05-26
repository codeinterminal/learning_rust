use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version)]
struct Args {
    #[clap(short, long)]
    infile: String,
}

fn main() {
    let args = Args::parse();
    println!("Input file: {}", args.infile);
}
