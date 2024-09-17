use anyhow::{Error, Result};
use std::collections::HashSet;

pub enum Command {
    Pwd,
    Cd   { dir: String },
    Echo { string: String },
    Exit { code: i32 },
    Type { arg: String },
    Exe  { file: String, args: Vec<String> },
}

impl Command {

    pub fn execute(&self, home: Option<&String>, paths: &HashSet<&str>) -> Result<()> {
        match self {
            Command::Cd { dir }         => Self::handle_cd(dir, home),
            Command::Echo { string }    => Self::handle_echo(string),
            Command::Exit { code }      => Self::handle_exit(*code),
            Command::Pwd                => Self::handle_pwd(),
            Command::Type { arg }       => Self::handle_type(arg, paths),
            Command::Exe { file, args } => Self::handle_exe(&file, args, paths),
        }
    }

    fn handle_cd(dir: &str, home: Option<&String>) -> Result<()> {
        let dir = match dir {
            "~" => home.ok_or_else(|| Error::msg("cd: HOME not set"))?,
            _   => dir,
        };
        std::env::set_current_dir(&dir)
            .map_err(|_| Error::msg(format!("cd: {}: No such file or directory", dir)))
    }

    fn handle_echo(string: &String) -> Result<()> {
        Ok(println!("{}", string))
    }

    fn handle_exit(code: i32) -> Result<()> {
        std::process::exit(code)
    }

    fn handle_pwd() -> Result<()> {
        let pwd = std::env::current_dir()?;
        Ok(println!("{}", pwd.display()))
    }

    fn handle_type(arg: &str, paths: &HashSet<&str>) -> Result<()> {
        let builtins = HashSet::from(["echo", "exit", "type", "pwd"]);
        match arg {
            _ if builtins.contains(arg) => Ok(println!("{} is a shell builtin", arg)),
            _ => {
                let file_path = Self::find_file(arg, paths);
                file_path.map_or_else(
                    || Err(Error::msg(format!("{}: not found", arg))),
                    |path| Ok(println!("{} is {}", arg, path)),
                )
            }
        }
    }

    fn handle_exe(file: &str, args: &Vec<String>, paths: &HashSet<&str>) -> Result<()> {
        let file_path = Self::find_file(file, paths);
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
            let file = format!("{}/{}", path, arg);
            std::fs::metadata(&file).ok().map(|_| file)
        })
    }
}

impl TryFrom<&str> for Command {
    type Error = anyhow::Error;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let mut it = input.trim().split_whitespace();
        match it.next() {
            Some("cd") => {
                let dir = it.next().unwrap_or("/").to_string();
                Ok(Command::Cd { dir })
            }
            Some("echo") => Ok(Command::Echo {
                string: it.collect::<Vec<_>>().join(" "),
            }),
            Some("exit") => {
                let code = it.next().unwrap_or("0").parse::<i32>()?;
                Ok(Command::Exit { code })
            }
            Some("pwd") => Ok(Command::Pwd),
            Some("type") => {
                let arg = it
                    .next()
                    .ok_or_else(|| Error::msg("type: missing argument"))?
                    .to_string();
                Ok(Command::Type { arg })
            }
            Some(file) => Ok(Command::Exe {
                file: file.to_string(),
                args: it.map(str::to_string).collect(),
            }),
            None => Err(Error::msg("Unexpected end of input")),
        }
    }
}
