# rustpass 🔐
**Rustpass** is a secure, lightweight, and customizable password generator written in **Rust** 🦀. It allows you to create strong passwords directly from the command line.
<div>
   <img src="https://img.shields.io/badge/Language-rust-orange" alt="Language" align="center" />
   <img src="https://img.shields.io/github/last-commit/AlexysGromard/rustpass" alt="Last commit" align="center" />
</div>

## Installation
1. Clone the repository:
   ```bash
   git clone https://github.com/AlexysGromard/rustpass
   cd rustpass
   ```
Note: Make sure you have Rust installed. If not, you can install it from [rustup.rs](https://rustup.rs/).


## 🧑‍💻Usage
You can use **rustpass** to generate passwords with various options. Here are some examples:
   ```bash
   cargo run -- --length 16 --uppercase --numbers --symbols
   ```
Available options:
- `--length`: Specify the length of the password (default: 16).
- `--uppercase`: Include uppercase letters in the password.
- `--numbers`: Include numbers in the password.
- `--symbols`: Include symbols in the password.

