use clap::Parser;

#[derive(Parser, Debuf)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to gree
    #[arg(short, long)]
    name: String,
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

