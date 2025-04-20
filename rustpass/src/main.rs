mod args;
mod generator;

use args::Args;
use clap::Parser;

fn main() {
    let args: Args = Args::parse();

    // Print the arguments
    println!("Length: {}", args.lenght);
    println!("Uppercase: {}", args.uppercase);
    println!("Numbers: {}", args.numbers);
    println!("Symbols: {}", args.symbols);
}
