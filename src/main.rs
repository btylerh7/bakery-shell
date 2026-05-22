#[allow(unused_imports)]
mod parser;
mod repl;
mod shell;
mod builtins;
use std::{env, path::PathBuf};
use rustyline::{Cmd, ConditionalEventHandler, DefaultEditor, EventHandler, KeyEvent};

use crate::repl::REPL;
use crate::parser::Parser;

struct TabEventHandler;
impl ConditionalEventHandler for TabEventHandler {

    fn handle(
        &self,
        evt: &rustyline::Event,
        n: rustyline::RepeatCount,
        positive: bool,
        ctx: &rustyline::EventContext,
    ) -> Option<rustyline::Cmd>
    {
        let current_arg = ctx.line().replace("$ ", "");
        if current_arg.contains("ech") && current_arg.len() == 3 {
            return Some(Cmd::Insert(n, String::from("o ")))
        }
            Some(Cmd::Insert(n, String::from("Hiiiii")))
    }
}

fn main() {
    // Load path environment variable
    let mut paths: Vec<PathBuf> = vec![];
    if let Some(path_list) = std::env::var_os("PATH") {
        paths = env::split_paths(&path_list).collect();
    }

    let mut rl = DefaultEditor::new().unwrap();
    rl.bind_sequence(KeyEvent::from('\t'), EventHandler::Conditional(Box::new(TabEventHandler)));

    // Eval loop
    loop {
        // REPL::print_string("$ ");
        let input = rl.readline("$ ").unwrap_or(String::new());
        // let input = REPL::read_input();
        let mut arg_parser = Parser::new();
        let args = arg_parser.parse_arg_string(&input);
        REPL::eval(args, &paths);
    }
}


