mod builtins;
#[allow(unused_imports)]
mod parser;
mod repl;
mod shell;
mod completion;
use rustyline::config::Configurer;
use rustyline::history::FileHistory;
use rustyline::{ Editor, CompletionType, DefaultEditor, EventHandler, KeyEvent, Cmd};

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

    let mut rl: Editor<TabEventHandler, FileHistory> = Editor::new().unwrap();
    rl.set_helper(Some(TabEventHandler));
    rl.set_completion_type(CompletionType::List);
    rl.bind_sequence(
        KeyEvent::from('\t'),
        Cmd::Complete
    );

    // Eval loop
    loop {
        let input = rl.readline("$ ").unwrap_or(String::new());
        let mut arg_parser = Parser::new();
        let args = arg_parser.parse_arg_string(&input);
        REPL::eval(args, &paths);
    }
}
