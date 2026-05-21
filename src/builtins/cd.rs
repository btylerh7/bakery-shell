use std::path::Path;
use crate::shell::CommandError;

pub fn handle_cd(directory: &str) -> Result<String, CommandError> {
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
        match std::env::set_current_dir(check_path) {
            // Ok(_) => Err(format!("Error changing directory to {:?}, {:?}", check_path, "Unknown error")),
            Ok(_) => Ok(String::new()),
            Err(error) => Err(CommandError::Process(format!("Error changing directory to {:?}, {:?}", check_path, error)))
        }
    } else {
        let message = format!("cd: {}: No such file or directory \r\n", new_dir);
        Err(CommandError::Process(message))
    }
}
