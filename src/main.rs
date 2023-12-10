use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short = 'k', long = "kount", default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();

    println!("name: {:?}, count: {:?}", args.name, args.count)
}
