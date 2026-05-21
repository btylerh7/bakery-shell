use crate::shell::CommandError;
use crate::shell::ShellCommand;
use crate::repl::REPL;
use std::path::PathBuf;

pub fn handle_type(command: &str, paths: &Vec<PathBuf>) -> Result<String, CommandError> {
    match ShellCommand::from_str(command.trim()) {
        Ok(_) => Ok(format!("{} is a shell builtin", command)),
        Err(_) => {
            let in_path = REPL::check_in_path(&command, paths);
            match in_path {
                Some(exec_path) => Ok(format!("{} is {}", command, exec_path)),
                None => Err(CommandError::Process(format!("{}: not found", command))),
            }
        }
    }
}
