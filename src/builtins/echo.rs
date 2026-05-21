use crate::shell::CommandError;

pub fn handle_echo(input: &str) -> Result<String, CommandError> {
    Ok(input.to_string())
}
