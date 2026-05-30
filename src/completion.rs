use rustyline::error::ReadlineError;
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use rustyline::Helper;
use rustyline::completion::{Candidate, Completer, Pair};
use std::path::PathBuf;
use std::env;

use crate::repl::REPL;
use crate::shell::{ShellCommand, ShellHelper};

impl ShellHelper {
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
    pub fn append_space_to_completion(candidates: Vec<Pair>) -> Vec<Pair> {
        let candidate_arr: Vec<Pair> = candidates
            .iter()
            .map(|candidate| {
                let mut new_rep = candidate.replacement.clone();
                if !new_rep.ends_with("/") {
                    new_rep.push_str(" ");
                }
                return Pair{display: candidate.replacement.clone(), replacement: new_rep}
            }).collect();
        candidate_arr
    }
}

impl Completer for ShellHelper {
    type Candidate = Pair;
    fn complete(
        &self, 
        line: &str,
        pos: usize,
        ctx: &rustyline::Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)>
    {

        let args: Vec<String> = line.split_whitespace().map(|res| res.to_string()).collect();
        // check for programmable completions
        if let Some(found_completions) = ShellHelper::run_completer_script(&args, &self.completions) {
            return Ok((pos, found_completions))
        }
        let last_char = line.to_string().chars().last();
        if  last_char.is_some_and(|ch| ch.is_whitespace()) {
            // add filepaths as completion options
            let file_candidates = self.file_names.complete_path(line, pos);
            match file_candidates {
                Ok(candidates) => {
                    let candidate_arr = ShellHelper::append_space_to_completion(candidates.1);
                    return Ok((pos, candidate_arr))
                }
                Err(_) => {}
            }
        }
        if args.len() > 1 {
            // only complete last argument
            // ex. if "ls ./", only autocomplete "./"
            let curr_pos = ShellHelper::get_pos_of_arg(&args);
            let array_length = args.len() - 1;
            let curr_arg = &args[array_length];

            let file_candidates = self.file_names.complete_path(curr_arg.as_str(), curr_arg.len());
            match file_candidates {
                Ok(candidates) => {
                    let candidate_arr = ShellHelper::append_space_to_completion(candidates.1);
                    return Ok((curr_pos, candidate_arr))
                }
                Err(_) => {}
            }
        }
        // Check builtin functions for completion
        let options = ["echo", "exit", "complete"];
        let matched_builtins: Vec<Pair> = options
            .into_iter()
            .filter(|option| {
                option.starts_with(&line)
            })
            .map(|option| {
                Pair{
                    display: option.to_string(),
                    replacement: format!("{} ", option).to_string()
                }
            })
            .collect();
        if matched_builtins.len() == 0 {
            // Check for other executables on path that match
            let matched_executables: Vec<Pair> = ShellHelper::check_executable_names(&line).into_iter().map(|mat| {
                let display = mat.clone().to_string();
                Pair {
                    display: display,
                    replacement: format!("{} ", mat).to_string()
                }
            }).collect();
            return Ok((0, matched_executables))
            
        }
        Ok((0, matched_builtins))
    }

}
impl Helper for ShellHelper {}
impl Validator for ShellHelper {}
impl Hinter for ShellHelper {
    type Hint = String;
}
impl Highlighter for ShellHelper {}
