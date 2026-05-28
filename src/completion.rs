use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use rustyline::Helper;
use rustyline::completion::{Candidate, Completer, Pair, FilenameCompleter};
use std::path::PathBuf;
use std::env;

pub struct TabEventHandler {
    executables: Vec<String>,
    file_names: FilenameCompleter,
    tab_press_count: u8
}
impl TabEventHandler {
    pub fn new() -> TabEventHandler {
        return TabEventHandler {
            executables: vec![],
            file_names: FilenameCompleter::new(),
            tab_press_count: 0
        }
    }
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
        found_executables.sort_by(|a, b| a.replacement().cmp(&b.replacement()));
        found_executables.dedup_by(|a, b| a.display() == b.display());
        found_executables
    }
    pub fn get_pos_of_arg(args: &Vec<String>) -> usize {
        let index = args.len() - 1;
        let mut recreated_string = args[0..index].to_vec().join(" ");
        recreated_string.push_str(" ");
        recreated_string.len()
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

        let args: Vec<String> = line.split_whitespace().map(|res| res.to_string()).collect();
        
        if let Some(last_char) = line.to_string().chars().last() && last_char.is_whitespace() {
            let file_candidates = self.file_names.complete_path(line, pos);
            match file_candidates {
                Ok(candidates) => {
                    let candidate_arr: Vec<Pair> = candidates.1.iter()
                        .map(|candidate| {
                            let mut new_rep = candidate.replacement.clone();
                            if !new_rep.ends_with("/") {
                                new_rep.push_str(" ");
                            }
                            return Pair{display: candidate.replacement.clone(), replacement: new_rep}
                        }).collect();
                    return Ok((pos, candidate_arr))
                }
                Err(_) => {}
            }
        }
        if args.len() > 1 {
            let curr_pos = TabEventHandler::get_pos_of_arg(&args);
            let array_length = args.len() - 1;
            let curr_arg = &args[array_length];

            let file_candidates = self.file_names.complete_path(curr_arg.as_str(), curr_arg.len());
            match file_candidates {
                Ok(candidates) => {
                    let candidate_arr: Vec<Pair> = candidates.1.iter()
                        .map(|candidate| {
                            let mut new_rep = candidate.replacement.clone();
                            if !new_rep.ends_with("/") {
                                new_rep.push_str(" ");
                            }
                            return Pair{display: candidate.replacement.clone(), replacement: new_rep}
                        }).collect();
                    return Ok((curr_pos, candidate_arr))
                }
                Err(_) => {}
            }
        }
        let options = ["echo", "exit", "complete"];
        let matched:Vec<Pair> = options.into_iter()
            .filter(|option| {
                option.starts_with(&line)
            })
            .map(|option| {
                return Pair{
                    display: option.to_string(),
                    replacement: format!("{} ", option).to_string()
                }
            })
            
            .collect();
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
