use rustyline::{Cmd, ConditionalEventHandler};
use std::path::{Path, PathBuf};
use std::env;

pub struct TabEventHandler;
impl ConditionalEventHandler for TabEventHandler {
    fn handle(
        &self,
        evt: &rustyline::Event,
        n: rustyline::RepeatCount,
        positive: bool,
        ctx: &rustyline::EventContext,
    ) -> Option<rustyline::Cmd> {
        let options = ["echo", "exit"];
        let current_arg = ctx.line().replace("$ ", "");
        let matched:Vec<&str> = options.into_iter().filter(|option| {
            option.starts_with(&current_arg)
        }).collect();
        if matched.len() > 0 {
            let mut auto_fill = matched[0].replace(&current_arg, "");
            auto_fill.push_str(" ");
            return Some(Cmd::Insert(n, String::from(auto_fill)));
        } else {
            TabEventHandler::check_executable_names(&current_arg);
        }
        Some(Cmd::Insert(n, String::from("\x07")))
    }
}
impl TabEventHandler {
    pub fn check_executable_names(current: &str) -> Option<String> {
        let mut paths: Vec<PathBuf> = vec![];
        if let Some(path_list) = std::env::var_os("PATH") {
            paths = env::split_paths(&path_list).collect();
        }
        let paths_cloned = paths.clone();
        let completion_opts:Vec<PathBuf> = paths.into_iter().filter(|path| {
            if !path.exists() {
                return false
            }
            match Path::read_dir(&path) {
                Ok(dir_read) => {
                    println!("checking files in dir {:?}", &dir_read);
                    let checked_files = dir_read.filter(|file| {
                        println!("File {:?}", &file);
                        if let Ok(file_path) = file {
                            println!("file path name {:?}", file_path.file_name());
                        }
                        return false
                    });
                    return true
                },
                Err(_) => return false
            };
        }).collect();
        for p in paths_cloned {
            let command_check = p.join(current);
            // if command_check.exists() && REPL::is_executable(&command_check) {
            //     return Some(command_check.into_os_string().into_string().unwrap());
            // }
            continue;
        }
        None
    }
}
