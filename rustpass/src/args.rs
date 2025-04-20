use std::env;

pub fn read_args() {
    // Read the arguments
    let args: Vec<String> = env::args().skip(1).collect();

    for (i, argument) in args.iter().enumerate() {
        println!("Argument {i} is {argument}")
    }
}