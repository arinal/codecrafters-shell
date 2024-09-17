mod command;

use command::Command;
use std::{
    collections::HashSet,
    io::{self, Write},
};

fn main() -> io::Result<()> {
    let binding = std::env::var("PATH").expect("PATH not set");
    let paths = binding.split(':').collect::<HashSet<_>>();
    let home = std::env::var("HOME").ok();

    loop {
        prompt()?
            .as_str()
            .try_into()
            .and_then(|cmd: Command| cmd.execute(home.as_ref(), &paths))
            .unwrap_or_else(|e| eprintln!("{}", e));
    }
}

fn prompt() -> io::Result<String> {
    let stdin = io::stdin();
    print!("$ ");
    io::stdout().flush()?;
    let mut input = String::new();
    stdin.read_line(&mut input)?;
    Ok(input)
}
