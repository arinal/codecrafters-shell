use std::{
    collections::HashSet,
    io::{self, Write},
    str::SplitWhitespace,
};

use anyhow::{Error, Result};

fn main() -> io::Result<()> {
    let binding = std::env::var("PATH").expect("PATH not set");
    let paths = binding.split(':').collect::<HashSet<_>>();
    let home = std::env::var("HOME").ok();

    loop {
        let input = prompt()?;
        let mut it = input.trim().split_whitespace();
        match it.next() {
            Some("cd") => handle_cd(it, home.as_ref()),
            Some("echo") => handle_echo(it),
            Some("exit") => handle_exit(it),
            Some("pwd") => handle_pwd(),
            Some("type") => handle_type(it, &paths),
            Some(file) => handle_exe(file, it, &paths),
            None => Ok(()),
        }
        .unwrap_or_else(|e| eprintln!("{}", e));
    }
}

fn handle_cd(mut args: SplitWhitespace, home: Option<&String>) -> Result<()> {
    let dir = args.next().unwrap_or("/");
    let dir = match dir {
        "~" => home.ok_or_else(|| Error::msg("cd: HOME not set"))?,
        _ => dir,
    };
    std::env::set_current_dir(&dir)
        .map_err(|_| Error::msg(format!("cd: {}: No such file or directory", dir)))
}

fn handle_echo(args: SplitWhitespace) -> Result<()> {
    let rest = args.collect::<Vec<&str>>().join(" ").trim().to_string();
    Ok(println!("{}", rest))
}

fn handle_exit(mut args: SplitWhitespace) -> Result<()> {
    let code = args.next().unwrap_or("0").parse::<i32>()?;
    std::process::exit(code)
}

fn handle_pwd() -> Result<()> {
    let pwd = std::env::current_dir()?;
    Ok(println!("{}", pwd.display()))
}

fn handle_type(mut args: SplitWhitespace, paths: &HashSet<&str>) -> Result<()> {
    let arg = args
        .next()
        .ok_or_else(|| Error::msg("type: missing argument"))?;

    let builtins = HashSet::from(["echo", "exit", "type", "pwd"]);

    match arg {
        _ if builtins.contains(arg) => Ok(println!("{} is a shell builtin", arg)),
        _ => {
            let file_path = find_file(arg, paths);
            file_path.map_or_else(
                || Err(Error::msg(format!("{}: not found", arg))),
                |path| Ok(println!("{} is {}", arg, path)),
            )
        }
    }
}

fn handle_exe(file: &str, args: SplitWhitespace, paths: &HashSet<&str>) -> Result<()> {
    let file_path = find_file(file, paths);
    file_path.map_or_else(
        || Err(Error::msg(format!("{}: not found", file))),
        |path| {
            let mut cmd = std::process::Command::new(path);
            cmd.args(args);
            cmd.status().map(|_| ()).map_err(Error::from)
        },
    )
}

fn find_file(arg: &str, paths: &HashSet<&str>) -> Option<String> {
    paths.iter().find_map(|path| {
        let cmd = format!("{}/{}", path, arg);
        std::fs::metadata(&cmd).ok().map(|_| cmd)
    })
}

fn prompt() -> io::Result<String> {
    let stdin = io::stdin();
    print!("$ ");
    io::stdout().flush()?;
    let mut input = String::new();
    stdin.read_line(&mut input)?;
    Ok(input)
}
