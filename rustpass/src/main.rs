mod args;
mod generator;

use args::Args;
use clap::Parser;

fn main() {
    let args: Args = Args::parse();
    let password = generator::generate_password(&args);
    println!("{}", password);
}
