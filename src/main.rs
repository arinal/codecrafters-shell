#[allow(unused_imports)]
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let stdin = io::stdin();

    loop {
        print!("$ ");
        io::stdout().flush()?;
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        input = input.trim().to_string();

        match input.as_str() {
            "exit 0" => break Ok(()),
            _ => println!("{}: command not found", input),
        }
    }
}
