use crate::args::Args;
use rand::Rng;

pub fn generate_password(args: &Args) -> String {
    let mut base = String::from("abcdefghijklmnopqrstuvwxyz");

    if args.uppercase {
        base.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    }

    if args.numbers {
        base.push_str("0123456789");
    }

    if args.symbols {
        base.push_str("!@#$%^&*()-_=+[]{}|;:,.<>?/");
    }

    // Get an RNG:
    let mut rng = rand::rng();
    // Generate a random password:
    let password: String = (0..args.lenght)
        .map(|_| {
            let idx = rng.random_range(0..base.len());
            base.chars().nth(idx).unwrap()
        })
        .collect();

    return password;
}