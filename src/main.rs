use std::{
    collections::HashSet,
    io::{self, Write},
    str::SplitWhitespace,
};

use anyhow::{Error, Result};

fn main() -> io::Result<()> {
    let binding = std::env::var("PATH").expect("PATH not set");
    let paths = binding.split(':').collect::<HashSet<_>>();

    loop {
        let input = prompt()?;
        let mut it = input.trim().split_whitespace();
        match it.next() {
            Some("echo") => handle_echo(it),
            Some("exit") => handle_exit(it),
            Some("type") => handle_type(it, &paths),
            Some(unknown) => Err(Error::msg(format!("{}: command not found", unknown))),
            None => Err(Error::msg("unexpected end of command input")),
        }
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

fn handle_echo(tokens: SplitWhitespace) -> Result<()> {
    let rest = tokens.collect::<Vec<&str>>().join(" ").trim().to_string();
    Ok(println!("{}", rest))
}

fn handle_exit(mut tokens: SplitWhitespace) -> Result<()> {
    let code = tokens.next().unwrap_or("0").parse::<i32>()?;
    std::process::exit(code)
}

fn handle_type(mut tokens: SplitWhitespace, paths: &HashSet<&str>) -> Result<()> {
    let arg = tokens
        .next()
        .ok_or_else(|| Error::msg("type: missing argument"))?;

    let builtins = HashSet::from(["echo", "exit", "type"]);

    match arg {
        _ if builtins.contains(arg) => Ok(println!("{} is a shell builtin", arg)),
        _ => {
            let path = paths.iter().find_map(|path| {
                let cmd = format!("{}/{}", path, arg);
                std::fs::metadata(&cmd).ok().map(|_| cmd)
            });
            path.map_or_else(
                || Err(Error::msg(format!("{}: not found", arg))),
                |path| Ok(println!("{} is {}", arg, path)),
            )
        }
    }
}
