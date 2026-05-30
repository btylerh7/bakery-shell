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
    pub fn run_completer_script(args: &Vec<String>, completions: &HashMap<String, String>) -> Option<Vec<Pair>> {

        let command = &args[0];
        let length = args.len();
        let file_path = completions.get(command)?;
        // Arg1: Command, Arg2: Word being completed, Arg3: Previous arg after Command, if it exists
        let mut completion_args = vec![
            command.clone().to_string(),
            String::new(),
            String::new()
        ];
        if length > 1 {
            completion_args[1] = args.last().unwrap_or(&String::new()).to_string();
        }
        if length > 2 {
            completion_args[2] = args[length - 1].clone();
        }
        let process_result = ShellHelper::handle_process(&file_path, completion_args).ok()?;
        let out = String::from_utf8(process_result.stdout).ok()?;
        let completion_opts:Vec<Pair> = out.lines()
            .filter(|line| !line.is_empty())
            .map(|line| {
                let mut replace_string = line.to_string();
                replace_string.push(' ');
                Pair { display: line.to_string(), replacement: replace_string}
            })
            .collect();
        Some(completion_opts)
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
