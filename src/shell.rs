use crate::repl::REPL;
use std::fs::{write, create_dir_all};
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::{Command};
pub enum CommandError {
    NotFound,
}
pub enum ShellCommand {
    Exit,
    Echo,
    Type,
    Pwd,
    Cd,
}
impl ShellCommand {
    // fn to_str(&self) -> &str {
    //     match self {
    //         ShellCommand::Exit => "exit",
    //         ShellCommand::Echo => "echo",
    //         ShellCommand::Type => "type",
    //     }
    // }
    pub fn from_str(check: &str) -> Result<Self, CommandError> {
        match check.trim() {
            "exit" => Ok(ShellCommand::Exit),
            "echo" => Ok(ShellCommand::Echo),
            "type" => Ok(ShellCommand::Type),
            "pwd" => Ok(ShellCommand::Pwd),
            "cd" => Ok(ShellCommand::Cd),
            _ => Err(CommandError::NotFound),
        }
    }
    pub fn handle_echo(input: &str) -> String {
        input.to_string()
    }
    pub fn handle_exit() -> String {
        std::process::exit(0);
    }
    pub fn handle_pwd() -> String {
        let current_directory = std::env::current_dir().unwrap();
        current_directory.into_os_string().to_str().unwrap().to_string()
    }
    pub fn handle_type(command: &str, paths: &Vec<PathBuf>) -> String {
        let result = match ShellCommand::from_str(command.trim()) {
            Ok(_) => format!("{} is a shell builtin", command),
            Err(_) => {
                let in_path = REPL::check_in_path(&command, paths);
                match in_path {
                    Some(exec_path) => format!("{} is {}", command, exec_path),
                    None => format!("{}: not found", command),
                }
            }
        };
        result
    }
    pub fn handle_process( command: &str, args: Vec<String>,) -> Result<std::process::Output, std::io::Error> {
        Command::new(command)
            .args(args.iter().map(|arg| return arg.trim()))
            .output()
    }
    pub fn handle_not_found(command: &str) -> String {
        let message = format!("{}: command not found", command.trim());
        message
    }
    pub fn handle_cd(directory: &str) -> String {
        let mut new_dir = directory.to_owned();
        if new_dir.starts_with("~") {
            if let Some(home_dir) = std::env::var_os("HOME")
                && let Ok(home_dir_string) = home_dir.to_os_string().into_string()
            {
                new_dir = new_dir.replace("~", &home_dir_string);
            }
        }
        let check_path = Path::new(&new_dir);
        if check_path.exists() {
            if let Err(error) = std::env::set_current_dir(check_path) {
                format!("Error changing directory to {:?}, {:?}", check_path, error).to_string()
            } else {
                format!("Error changing directory to {:?}, {:?}", check_path, "Unknown error").to_string()
            }
        } else {
            let message = format!("cd: {}: No such file or directory \r\n", new_dir);
            message
        }
    }
    pub fn redirect_std_out(output: &str, file_path: String, remaining_args: Vec<String>) {
        let mut result = String::from(output);
        for arg in remaining_args {
            result.push_str(&arg);
        }
        let path = Path::new(&file_path);
        if let Some(parent_path) = path.parent() {
            if !parent_path.exists() {
                let _ = create_dir_all(parent_path);
            }
            if let Err(err) = write(path, result) {
                REPL::print_string("Whoops, couldn't write to file");
                REPL::print_string(&err.to_string());
            }
        }
    }
}
