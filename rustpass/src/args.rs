use clap::Parser;

#[derive(Parser)]
#[command(name = "rustpass", about = "A password generator", version = "1.0")]
pub struct Args {
    /// The length of the password
    #[arg(short, long, default_value = "16")]
    pub lenght: usize,

    /// If the password should contain uppercase letters
    #[arg(short, long, default_value = "false")]
    pub uppercase: bool,

    /// If the password should contain numbers
    #[arg(short, long, default_value = "false")]
    pub numbers: bool,

    /// If the password should contain symbols
    #[arg(short, long, default_value = "false")]
    pub symbols: bool,
}