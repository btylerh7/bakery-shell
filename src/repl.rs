use crate::shell::ShellCommand;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Output;
pub struct REPL {}
impl REPL {
    pub fn read_input() -> String {
        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();
        command
    }
    pub fn eval(args: Vec<String>, paths: &Vec<PathBuf>) {
        let command_name = ShellCommand::from_str(&args[0]);
        let mut commands: Vec<Vec<String>> = vec![];
        let mut current_command: Vec<String> = vec![];

        for arg in args.clone().iter() {
            if arg.as_str() == ">" {
                commands.push(current_command.clone());
                current_command.clear();
                current_command.push(arg.clone().to_string());
                continue;
            }
            current_command.push(arg.clone());
        }
        commands.push(current_command);
        let mut std_out = String::new();
        for mut command in commands {
            let shell_command = ShellCommand::from_str(&command[0]);
            let output = match shell_command {
                Ok(ShellCommand::Exit) => ShellCommand::handle_exit(),
                Ok(ShellCommand::Echo) => ShellCommand::handle_echo(&command[1..].join(" ")),
                Ok(ShellCommand::Type) => ShellCommand::handle_type(&command[1].trim(), paths),
                Ok(ShellCommand::Pwd) => ShellCommand::handle_pwd(),
                Ok(ShellCommand::Cd) => ShellCommand::handle_cd(&command[1].trim()),
                _ => {
                    if &command[0] == ">" {
                        let _cmd = command.remove(0);
                        let file_path = command.remove(0);
                        ShellCommand::redirect_std_out(&std_out, file_path, command);
                        std_out = String::new();
                        String::new()
                    } else if let Some(execute_path) = REPL::check_in_path(&command[0].trim(), paths) {
                        let args_for_process: Vec<String> = match command.len() {
                            0 => vec![],
                            1 => vec![],
                            _ => command[1..].to_vec()
                        };
                        if let Ok(result) = ShellCommand::handle_process(&execute_path, args_for_process) {
                            if result.stderr.len() > 0 && let Ok(err) = String::from_utf8(result.stderr) {
                                err.trim_end().to_string()
                            }
                            else if let Ok(out) = String::from_utf8(result.stdout) {
                                out.trim_end().to_string()
                            } else {
                                String::new()
                            }
                        } else {
                            String::new()
                        }
                    } else {
                        ShellCommand::handle_not_found(&args[0].trim())
                    }
                }
            };
            std_out.push_str(&output);
        }
        if !std_out.is_empty() {
            REPL::print_string(&std_out);
            REPL::print_string("\r\n");
        }
    }
    pub fn print_string(text: &str) {
        print!("{}", text);
        io::stdout().flush().unwrap();
    }

    pub fn check_in_path(command: &str, paths: &Vec<PathBuf>) -> Option<String> {
        let paths_cloned = paths.clone();
        for p in paths_cloned {
            let command_check = p.join(command);
            if command_check.exists() && REPL::is_executable(&command_check) {
                return Some(command_check.into_os_string().into_string().unwrap());
            }
            continue;
        }
        None
    }
    #[cfg(unix)]
    pub fn is_executable(file: &PathBuf) -> bool {
        if let Ok(metadata) = file.as_path().metadata() {
            use std::os::unix::fs::PermissionsExt;

            let permissions = metadata.permissions();
            return permissions.mode() & 0o111 != 0;
        }
        false
    }

    // #[cfg(windows)]
    // pub fn is_executable(file: &PathBuf) -> bool {
    //     if let Ok(metadata) = file.as_path().metadata() {
    //         use std::os::windows::fs::PermissionsExt;
    //
    //         let permissions = metadata.permissions();
    //         return permissions.mode() & 0x21 != 0;
    //     }
    //     false
    // }
}
