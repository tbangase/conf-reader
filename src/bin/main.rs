use clap::Parser;
use conf_reader::{conf_lines_from_path, json_from_conf, schema_from_path};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    conf_file_path: String,
    #[clap(short, long)]
    schema_file_path: Option<String>,
}

fn main() {
    let args = Args::parse();

    let json_value = json_from_conf(
        conf_lines_from_path(args.conf_file_path),
        args.schema_file_path
            .map(schema_from_path)
            .transpose()
            .unwrap(),
    )
    .unwrap();

    println!("{}", json_value);
}
