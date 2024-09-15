#[allow(unused_imports)]
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let stdin = io::stdin();

    loop {
        print!("$ ");
        io::stdout().flush()?;
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let mut it = input.trim().split_whitespace();

        match it.next() {
            Some("exit") => break Ok(()),
            Some("echo") => {
                let rest = it.collect::<Vec<&str>>().join(" ").trim().to_string();
                println!("{}", rest);
            }
            Some(unknown) => println!("{}: command not found", unknown),
            None => (),
        }
    }
}
