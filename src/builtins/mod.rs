use crate::shell::{CommandError, ShellBuiltin, ShellHelper};
use std::path::PathBuf;

pub mod cd;
pub mod complete;
pub mod echo;
pub mod exit;
pub mod jobs;
pub mod pwd;
pub mod typecmd; // type is a reserved word

pub fn run_builtin(
    command: ShellBuiltin,
    args: Vec<String>,
    paths: &Vec<PathBuf>,
    completions: &mut ShellHelper,
) -> Result<String, CommandError> {
    match command {
        ShellBuiltin::Exit => exit::handle_exit(),
        ShellBuiltin::Cd => cd::handle_cd(&args[1].trim()),
        ShellBuiltin::Echo => echo::handle_echo(&args[1..].join(" ")),
        ShellBuiltin::Type => typecmd::handle_type(&args[1], paths),
        ShellBuiltin::Pwd => pwd::handle_pwd(),
        ShellBuiltin::Complete => complete::handle_complete(&args, completions),
        ShellBuiltin::Jobs => jobs::handle_jobs(&mut completions.running_jobs, true),
    }
}
