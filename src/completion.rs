use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use rustyline::{Cmd, ConditionalEventHandler, Helper};
use rustyline::completion::{Completer, Pair};
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
        println!("\n input is {}", current_arg);
        let matched:Vec<&str> = options.into_iter().filter(|option| {
            option.starts_with(&current_arg)
        }).collect();
        if matched.len() > 0 {
            let mut auto_fill = matched[0].replace(&current_arg, "");
            auto_fill.push_str(" ");
            return Some(Cmd::Insert(n, String::from(auto_fill)));
        } else {
            // if let Some(matched_executable) = TabEventHandler::check_executable_names(&current_arg) {
            //     return Some(Cmd::Insert(n, matched_executable));
            // }
        }
        Some(Cmd::Insert(n, String::from("\x07")))
    }
}
impl TabEventHandler {
    pub fn check_executable_names(current: &str) -> Vec<String> {
        let mut paths: Vec<PathBuf> = vec![];
        if let Some(path_list) = std::env::var_os("PATH") {
            paths = env::split_paths(&path_list).collect();
        }
        let mut found_executables: Vec<String> = vec![];
        let paths_cloned = paths.clone();
        for p in paths_cloned {
            if p.exists() {
                if let Ok(entries) = p.read_dir() {
                    for entry in entries {
                        if let Ok(entry_path) = entry {
                            let path_to_save = entry_path.file_name().into_string().unwrap();
                            // let path_to_save = entry_path.path().into_os_string().into_string().unwrap();
                            if path_to_save.starts_with(current) {
                                found_executables.push(path_to_save)
                            }
                        }
                    }
                }
            }
            continue;
        }
        found_executables
    }
}

impl Completer for TabEventHandler {
    type Candidate = Pair;
    fn complete(
        &self, 
        line: &str,
        pos: usize,
        ctx: &rustyline::Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)>
    {

        let options = ["echo", "exit"];
        let matched:Vec<Pair> = options.into_iter().filter(|option| {
            option.starts_with(&line)
        }).map(|option| {
                return Pair{
                    display: option.to_string(),
                    replacement: format!("{} ", option).to_string()
                }
            }).collect();
        if matched.len() == 0 {
            let matched_executables: Vec<Pair> = TabEventHandler::check_executable_names(&line).into_iter().map(|mat| {
                let display = mat.clone().to_string();
                Pair {
                    display: display,
                    replacement: format!("{} ", mat).to_string()
                }
            }).collect();
            return Ok((0, matched_executables))
            
        }
        Ok((0, matched))
    }

}
impl Helper for TabEventHandler {}
impl Validator for TabEventHandler {}
impl Hinter for TabEventHandler {
    type Hint = String;
}
impl Highlighter for TabEventHandler {}
