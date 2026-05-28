use crate::repl::REPL;
use std::fs::{write, create_dir_all, read};
use std::os::unix::process::CommandExt;
use std::path::{Path};
use std::process::{Command};
pub enum CommandError {
    NotFound,
    Process(String)
}
pub enum ShellCommand {
    Exit,
    Echo,
    Type,
    Pwd,
    Cd,
    Complete
}
impl ShellCommand {
    pub fn from_str(check: &str) -> Result<Self, CommandError> {
        match check.trim() {
            "exit" => Ok(ShellCommand::Exit),
            "echo" => Ok(ShellCommand::Echo),
            "type" => Ok(ShellCommand::Type),
            "pwd" => Ok(ShellCommand::Pwd),
            "cd" => Ok(ShellCommand::Cd),
            "complete" => Ok(ShellCommand::Complete),
            _ => Err(CommandError::NotFound),
        }
    }
    pub fn handle_process( command: &str,mut args: Vec<String>) -> Result<std::process::Output, std::io::Error> {
        let original_command_input = args.remove(0);
        Command::new(command)
            .arg0(&original_command_input)
            .args(args.iter().map(|arg| return arg.trim()))
            .output()
    }
    pub fn handle_not_found(command: &str) -> String {
        let message = format!("{}: command not found", command.trim());
        message
    }
    pub fn redirect_output(output: &str, file_path: String, remaining_args: Vec<String>, append: bool) {
        let mut result = String::from(output);
        for arg in remaining_args {
            result.push_str(&arg);
        }
        let path = Path::new(&file_path);
        if let Some(parent_path) = path.parent() {
            if !parent_path.exists() {
                let _ = create_dir_all(parent_path);
            }
            if append {
                if let Ok(file_contents) = read(&path) && let Ok(mut new_string) = String::from_utf8(file_contents) {
                    if !new_string.is_empty() {
                        new_string.push_str("\n");
                    }
                    new_string.push_str(&result);
                    result = new_string;
                }
            }
            if let Err(err) = write(path, result) {
                REPL::print_string("Whoops, couldn't write to file");
                REPL::print_string(&err.to_string());
            }
        }
    }
}
