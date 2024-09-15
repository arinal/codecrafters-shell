use std::{
    collections::HashSet,
    io::{self, Write},
};

use anyhow::{Error, Result};

fn main() -> io::Result<()> {
    let stdin = io::stdin();

    loop {
        print!("$ ");
        io::stdout().flush()?;
        let mut input = String::new();
        stdin.read_line(&mut input)?;
        let mut it = input.trim().split_whitespace();

        match it.next() {
            Some("echo") => handle_echo(it),
            Some("exit") => handle_exit(it),
            Some("type") => handle_type(it),
            Some(unknown) => Err(Error::msg(format!("{}: command not found", unknown))),
            None => Err(Error::msg("unexpected end of command input")),
        }
        .unwrap_or_else(|e| eprintln!("{}", e));
    }
}
fn handle_echo(it: std::str::SplitWhitespace) -> Result<()> {
    let rest = it.collect::<Vec<&str>>().join(" ").trim().to_string();
    Ok(println!("{}", rest))
}

fn handle_exit(mut it: std::str::SplitWhitespace) -> Result<()> {
    let code = it.next().unwrap_or("0").parse::<i32>()?;
    std::process::exit(code);
}

fn handle_type(mut it: std::str::SplitWhitespace) -> Result<()> {
    let arg = it
        .next()
        .ok_or_else(|| Error::msg("type: missing argument"))?;
    match arg {
        arg if HashSet::from(["echo", "exit", "type"]).contains(arg) => {
            println!("{} is a shell builtin", arg);
            Ok(())
        }
        _ => Err(Error::msg(format!("{}: not found", arg))),
    }
}
