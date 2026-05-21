use crate::shell::CommandError;

pub fn handle_exit() -> Result<String, CommandError>{
    std::process::exit(0)
}
