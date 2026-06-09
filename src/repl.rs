use rustyline::history::FileHistory;

use crate::builtins::run_builtin;
use crate::shell::{CommandError, RunningJob, ShellBuiltin, ShellHelper};
use std::io::{self, BufReader, Read, Write};
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::thread;

#[derive(Clone, Debug, PartialEq)]
pub struct ShellCommand {
    args: Vec<String>, // args[0] will be command name that was passed
    executable_path: String,
    background: bool,
    is_builtin: bool,
    std_out: Option<PathBuf>,
    std_err: Option<PathBuf>,
    append: bool,
}
impl ShellCommand {
    pub fn new() -> Self {
        ShellCommand {
            args: vec![],
            executable_path: String::new(),
            background: false,
            is_builtin: false,
            std_out: None,
            std_err: None,
            append: false,
        }
    }
    pub fn is_empty(&self) -> bool {
        return *self == ShellCommand::new();
    }
}

pub struct REPL {
    std_out: Vec<String>,
    std_err: Vec<String>,
}
impl REPL {
    pub fn new() -> Self {
        REPL {
            std_out: vec![],
            std_err: vec![],
        }
    }
    pub fn determine_commands(
        &mut self,
        args: Vec<String>,
        paths: &Vec<PathBuf>,
    ) -> Vec<ShellCommand> {
        let mut commands: Vec<ShellCommand> = vec![];
        let redirect_symbols = vec![">", "1>", ">>", "1>>", "2>", "2>>"];
        let mut current_command = ShellCommand::new();
        let mut iter = args.into_iter();
        while let Some(arg) = iter.next() {
            // output or error needs to be redirected
            if redirect_symbols.contains(&arg.as_str()) {
                if let Some(file_path_str) = iter.next() {
                    let path_new = Path::new(&file_path_str);
                    if arg == ">" || arg == "1>" {
                        current_command.std_out = Some(path_new.to_path_buf());
                    }
                    if arg.as_str() == "2>" {
                        current_command.std_err = Some(path_new.to_path_buf());
                    }
                    if arg == ">>" || arg == "1>>" {
                        current_command.std_out = Some(path_new.to_path_buf());
                        current_command.append = true;
                    }
                    if arg == "2>>" {
                        current_command.std_err = Some(path_new.to_path_buf());
                        current_command.append = true;
                    }
                }
            } else {
                current_command.args.push(arg);
            }
        }
        if !current_command.is_empty() {
            commands.push(current_command);
        }
        commands.iter_mut().for_each(|command| {
            let shell_command = ShellBuiltin::from_str(&command.args[0]);
            match shell_command {
                // Is a builtin
                Ok(_) => command.is_builtin = true,
                // Not a builtin, locate executable
                Err(_) => {
                    if let Some(execute_path) = REPL::check_in_path(&command.args[0].trim(), paths)
                    {
                        command.executable_path = execute_path;
                    }
                }
            }
            // Run as background job
            if command.args[command.args.len() - 1] == "&" {
                let _ = command.args.pop();
                command.background = true;
            }
        });
        commands
    }
    pub fn eval(
        &mut self,
        args: Vec<String>,
        paths: &Vec<PathBuf>,
        rl: &mut rustyline::Editor<ShellHelper, FileHistory>,
    ) {
        let commands = self.determine_commands(args, paths);
        for command in commands {
            if command.is_builtin {
                self.handle_builtin(&command, paths, rl);
            } else {
                if command.background == true {
                    self.handle_background_thread(&command, rl);
                } else {
                    self.handle_executable(&command, paths);
                }
            }
            if let Some(out_path) = command.std_out {
                let path = out_path.as_os_str().to_str().unwrap();
                ShellHelper::redirect_output(
                    &self.std_out.join(""),
                    String::from(path),
                    command.args.clone(),
                    command.append,
                );
            } else {
                self.read_std_out();
            }
            if let Some(out_path) = command.std_err {
                let path = out_path.as_os_str().to_str().unwrap();
                ShellHelper::redirect_output(
                    &self.std_err.join(""),
                    String::from(path),
                    command.args.clone(),
                    command.append,
                );
            } else {
                self.read_std_err();
            }
        }
    }
    pub fn read_std_out(&mut self) {
        self.std_out = self
            .std_out
            .clone()
            .into_iter()
            .filter(|output| !output.is_empty())
            .collect();
        if !self.std_out.is_empty() {
            REPL::print_string(&self.std_out.join("\n"));
            REPL::print_string("\r\n");
        }
        self.std_out.clear();
    }
    pub fn read_std_err(&mut self) {
        self.std_err = self
            .std_err
            .clone()
            .into_iter()
            .filter(|output| !output.is_empty())
            .collect();
        if !self.std_err.is_empty() {
            REPL::print_string(&self.std_err.join("\n"));
            REPL::print_string("\r\n");
        }
        self.std_err.clear();
    }
    pub fn handle_background_thread(
        &mut self,
        command: &ShellCommand,
        rl: &mut rustyline::Editor<ShellHelper, FileHistory>,
    ) {
        let original = format!("{} &", command.args.clone().join(" "));
        let mut args = command.args.clone();
        let arg0 = args.remove(0);
        let p = std::process::Command::new(&command.executable_path)
            .arg0(&arg0)
            .args(args.iter().map(|arg| return arg.trim()))
            .spawn();
        if let Ok(process_info) = p {
            let completions = match rl.helper_mut() {
                Some(helper) => helper,
                None => &mut ShellHelper::new(),
            };
            let count = completions.running_jobs.len() + 1;
            let job = RunningJob {
                job_number: count,
                process_id: process_info.id(),
                command_string: original,
                status: "Running".to_string(),
            };
            completions.running_jobs.push(job);
            let out = format!("[{}] {}", count, process_info.id());
            self.std_out.push(out.to_string());
        }
    }
    pub fn handle_builtin(
        &mut self,
        command: &ShellCommand,
        paths: &Vec<PathBuf>,
        rl: &mut rustyline::Editor<ShellHelper, FileHistory>,
    ) {
        // Check if there are any registered completions from the complete builtin
        let completions = match rl.helper_mut() {
            Some(helper) => helper,
            None => &mut ShellHelper::new(),
        };
        // Already confirmed builtin
        let shell_builtin = ShellBuiltin::from_str(&command.args[0]).unwrap();
        let result = run_builtin(shell_builtin, command.args.clone(), &paths, completions);
        match result {
            Ok(result_string) => self.std_out.push(result_string),
            Err(error) => match error {
                CommandError::Process(err_message) => {
                    self.std_err.push(err_message.trim_end().to_string());
                }
                _ => {}
            },
        }
    }
    pub fn handle_executable(&mut self, command: &ShellCommand, paths: &Vec<PathBuf>) {
        match REPL::check_in_path(&&command.executable_path.trim(), paths) {
            Some(execute_path) => {
                if let Ok(result) =
                    ShellHelper::handle_process(&execute_path, command.args.to_vec())
                {
                    if result.stderr.len() > 0
                        && let Ok(err) = String::from_utf8(result.stderr)
                    {
                        self.std_err.push(err.trim_end().to_string());
                    }
                    if let Ok(out) = String::from_utf8(result.stdout) {
                        self.std_out.push(out.trim_end().to_string());
                    } else {
                    }
                } else {
                    // Std out clear?
                }
            }
            None => {
                self.std_err
                    .push(ShellHelper::handle_not_found(&command.args[0].trim()));
            }
        }
    }
    pub fn print_string(text: &str) {
        print!("{}", text);
        io::stdout().flush().unwrap();
    }

    pub fn check_in_path(command: &str, paths: &Vec<PathBuf>) -> Option<String> {
        if command.is_empty() {
            return None;
        }
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
