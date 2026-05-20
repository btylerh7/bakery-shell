#[allow(unused_imports)]
mod parser;
mod repl;
mod shell;
use std::{env, path::PathBuf};
use crate::repl::REPL;
use crate::parser::Parser;

fn main() {
    // Load path environment variable
    let mut paths: Vec<PathBuf> = vec![];
    if let Some(path_list) = std::env::var_os("PATH") {
        paths = env::split_paths(&path_list).collect();
    }

    // Eval loop
    loop {
        REPL::print_string("$ ");
        let input = REPL::read_input();
        let mut arg_parser = Parser::new();
        let args = arg_parser.parse_arg_string(&input);
        REPL::eval(args, &paths);
    }
}


