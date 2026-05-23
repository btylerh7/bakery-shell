mod builtins;
#[allow(unused_imports)]
mod parser;
mod repl;
mod shell;
mod completion;
use rustyline::{DefaultEditor, EventHandler, KeyEvent};

use std::{env, path::PathBuf};

use crate::parser::Parser;
use crate::repl::REPL;
use crate::completion::TabEventHandler;


fn main() {
    // Load path environment variable
    let mut paths: Vec<PathBuf> = vec![];
    if let Some(path_list) = std::env::var_os("PATH") {
        paths = env::split_paths(&path_list).collect();
    }

    let mut rl = DefaultEditor::new().unwrap();
    rl.bind_sequence(
        KeyEvent::from('\t'),
        EventHandler::Conditional(Box::new(TabEventHandler)),
    );

    // Eval loop
    loop {
        let input = rl.readline("$ ").unwrap_or(String::new());
        let mut arg_parser = Parser::new();
        let args = arg_parser.parse_arg_string(&input);
        REPL::eval(args, &paths);
    }
}
