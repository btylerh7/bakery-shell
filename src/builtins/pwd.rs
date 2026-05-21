use crate::shell::CommandError;

pub fn handle_pwd() -> Result<String, CommandError> {
    let current_directory = std::env::current_dir().unwrap();
    Ok(current_directory.into_os_string().to_str().unwrap().to_string())
}
