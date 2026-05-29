use std::path::PathBuf;
use std::collections::HashMap;
use crate::shell::{CommandError, ShellCommand, ShellHelper};

pub mod cd;
pub mod exit;
pub mod echo;
pub mod typecmd; // type is a reserved word
pub mod pwd;
pub mod complete;

pub fn run_builtin(command: ShellCommand, args: Vec<String>, paths: &Vec<PathBuf>, completions: &mut ShellHelper) -> Result<String, CommandError> {
    match command {
        ShellCommand::Exit => exit::handle_exit(),
        ShellCommand::Cd => cd::handle_cd(&args[1].trim()),
        ShellCommand::Echo => echo::handle_echo(&args[1..].join(" ")),
        ShellCommand::Type => typecmd::handle_type(&args[1], paths),
        ShellCommand::Pwd => pwd::handle_pwd(),
        ShellCommand::Complete => complete::handle_complete(&args, completions),
    }

}
