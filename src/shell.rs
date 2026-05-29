use rustyline::completion::{FilenameCompleter, Pair};

use crate::parser::Parser;
use crate::repl::REPL;
use std::fs::{write, create_dir_all, read};
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::{Command};
use std::collections::HashMap;
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
}
pub struct ShellHelper {
    pub file_names: FilenameCompleter,
    pub completions: HashMap<String, String>
}
impl ShellHelper {
    pub fn new() -> Self {
        ShellHelper {
            file_names: FilenameCompleter::new(),
            completions: HashMap::new()
        }
    }
    pub fn run_completer_script(command: &str, completions: &HashMap<String, String>) -> Option<Vec<Pair>> {
        println!("Command was {}\n Completions were {:?}", command, completions);
        // TODO: This is not returning results, clean up if statements to be less confusing
        if let Some(file_path) = completions.get(command) {
            if let Ok(result) = ShellHelper::handle_process(&file_path, vec![command.to_string()]) {
                if let Ok(out) = String::from_utf8(result.stdout) {
                    let completion_opts:Vec<Pair> = out.lines().map(|line| {
                        let path_string = line.to_string();
                        let path = std::path::Path::new(&path_string);
                        if path.exists() && REPL::is_executable(&path.to_path_buf()) {
                            let mut replace_str = line.to_string();
                            replace_str.push_str(" ");
                            return Pair{display: line.to_string(), replacement: replace_str}
                        } else {
                            return Pair{display: String::new(), replacement: String::new()}
                        }
                    }).collect();
                    return Some(completion_opts)
                }
            }
        }
        None
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
