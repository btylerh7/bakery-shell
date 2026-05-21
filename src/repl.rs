use crate::shell::{CommandError, ShellCommand};
use crate::builtins::run_builtin;
use std::io::{self, Write};
use std::path::PathBuf;
pub struct REPL {}
impl REPL {
    pub fn read_input() -> String {
        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();
        command
    }
    pub fn eval(args: Vec<String>, paths: &Vec<PathBuf>) {
        let mut commands: Vec<Vec<String>> = vec![];
        let mut current_command: Vec<String> = vec![];

        for arg in args.clone().iter() {
            if arg.as_str() == ">" || arg.as_str() == "1>" {
                commands.push(current_command.clone());
                current_command.clear();
                current_command.push(arg.clone().to_string());
                continue;
            }
            current_command.push(arg.clone());
        }
        commands.push(current_command);
        let mut std_out: Vec<String> = vec![];
        let mut std_err: Vec<String> = vec![];
        for mut command in commands {
            let shell_command = ShellCommand::from_str(&command[0]);
            match shell_command {
                Ok(shell_cmd) => {
                    let result = run_builtin(shell_cmd, command, &paths);
                    match result {
                        Ok(result_string) => std_out.push(result_string),
                        Err(error) => match error {
                            CommandError::Process(err_message) => {
                                // std_out = String::new();
                                std_err.push(err_message.trim_end().to_string());
                            },
                            _ => {}
                        }
                    }
                },
                _ => {
                    if &command[0] == ">" || &command[0] == "1>" {
                        let _cmd = command.remove(0);
                        let file_path = command.remove(0);
                        ShellCommand::redirect_std_out(&std_out.join("\n"), file_path, command);
                        std_out.clear();
                    } else if let Some(execute_path) = REPL::check_in_path(&command[0].trim(), paths) {
                        if let Ok(result) = ShellCommand::handle_process(&execute_path, command.to_vec()) {
                            if result.stderr.len() > 0 && let Ok(err) = String::from_utf8(result.stderr) {
                                std_err.push(err.trim_end().to_string());
                            }
                            if let Ok(out) = String::from_utf8(result.stdout) {
                                std_out.push(out.trim_end().to_string());
                            } else {}
                        } else {
                            // Std out clear?
                        }
                    } else {
                        std_err.push(ShellCommand::handle_not_found(&args[0].trim()));
                    }
                }
            };
        }
        std_out = std_out
            .into_iter()
            .filter(|output| !output.is_empty())
            .collect();
        if !std_err.is_empty() {
            REPL::print_string(&std_err.join("\n"));
            REPL::print_string("\r\n");

        }else if !std_out.is_empty() {
            REPL::print_string(&std_out.join("\n"));
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
