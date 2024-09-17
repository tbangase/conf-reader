use clap::Parser;
use conf_reader::{json_from_conf, lines_from_path};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    file_path: String,
}

fn main() {
    let args = Args::parse();

    println!("{}", json_from_conf(lines_from_path(args.file_path)));
}
