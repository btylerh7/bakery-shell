use rustyline::history::FileHistory;

use crate::shell::{CommandError, ShellCommand, ShellHelper};
use crate::builtins::run_builtin;
use std::clone;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, PartialEq)]
pub struct Command {
    args: Vec<String>, // args[0] will be command name that was passed
    executable_path: String,
    background: bool,
    is_builtin: bool,
    redirect: bool,
    std_out: Option<PathBuf>,
    std_err: Option<PathBuf>,
    append: bool
}
impl Command {
    pub fn new() -> Self {
        Command {
            args: vec![],
            executable_path: String::new(),
            background: false,
            is_builtin: false,
            redirect: false,
            std_out: None,
            std_err: None,
            append: false
        }
    }
}

pub struct REPL;
impl REPL {
    pub fn determine_commands(mut args:Vec<String>, paths: &Vec<PathBuf>) -> Vec<Command> {
        let mut commands: Vec<Command> = vec![];
        let redirect_symbols = vec![">", "1>", ">>", "1>>", "2>", "2>>"];
        let mut current_command = Command::new();
        let mut iter = args.into_iter();
        while let Some(arg) = iter.next() {

            // output or error needs to be redirected
            if redirect_symbols.contains(&arg.as_str()) {
                if let Some(file_path_str) = iter.next() {
                    let path_new = Path::new(&file_path_str);
                    if vec![">", "1>"].contains(&arg.as_str()) {
                        current_command.std_out = Some(path_new.to_path_buf());
                        current_command.redirect = true;
                    }
                    if arg.as_str() == "2>" {
                        current_command.std_err = Some(path_new.to_path_buf());
                        current_command.redirect = true;
                    }
                    if vec![">>", "1>>"].contains(&arg.as_str()) {
                        current_command.std_out = Some(path_new.to_path_buf());
                        current_command.redirect = true;
                        current_command.append = true;
                    }
                    if vec!["2>>"].contains(&arg.as_str()) {
                        current_command.std_err = Some(path_new.to_path_buf());
                        current_command.redirect = true;
                        current_command.append = true;
                    }
                }
                commands.push(current_command.clone());
                current_command = Command::new();

            } else {
                current_command.args.push(arg);
            }
        }
        commands
    }
    pub fn eval(mut args: Vec<String>, paths: &Vec<PathBuf>, rl: &mut rustyline::Editor<ShellHelper, FileHistory>) {
        let mut commands: Vec<Vec<String>> = vec![];
        let mut current_command: Vec<String> = vec![];

        // spawn background process
        if args[args.len() - 1] == "&".to_string() {
            let _ = args.pop();
            REPL::eval(args, paths, rl);
            return
        }
        // Redirect standard out or standard error
        for arg in args.clone().iter() {
            if [">", "1>", ">>", "1>>", "2>", "2>>" ].contains(&&arg.as_str()) {
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
                    // Check if there are any registered completions from the complete builtin
                    let completions = match rl.helper_mut() {
                        Some(helper) => helper,
                        None => &mut ShellHelper::new()
                    };
                    let result = run_builtin(shell_cmd, command, &paths, completions);
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
                    let command_string = command[0].as_str();
                    match command_string {
                        command_string if [">", "1>"].contains(&command_string) => {
                            let _cmd = command.remove(0);
                            let file_path = command.remove(0);
                            ShellHelper::redirect_output(&std_out.join("\n"), file_path, command, false);
                            std_out.clear();
                        },
                        "2>" => {
                            let _cmd = command.remove(0);
                            let file_path = command.remove(0);
                            ShellHelper::redirect_output(&std_err.join("\n"), file_path, command, false);
                            std_err.clear();
                        },
                        command_string if [">>", "1>>"].contains(&command_string) => {
                            let _cmd = command.remove(0);
                            let file_path = command.remove(0);
                            ShellHelper::redirect_output(&std_out.join("\n"), file_path, command, true);
                            std_out.clear();
                        },
                        "2>>" => {
                            let _cmd = command.remove(0);
                            let file_path = command.remove(0);
                            ShellHelper::redirect_output(&std_err.join("\n"), file_path, command, true);
                            std_err.clear();
                        },
                        _ => {
                            if let Some(execute_path) = REPL::check_in_path(&command[0].trim(), paths) {
                                if let Ok(result) = ShellHelper::handle_process(&execute_path, command.to_vec()) {
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
                                std_err.push(ShellHelper::handle_not_found(&args[0].trim()));
                            }
                        }
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
