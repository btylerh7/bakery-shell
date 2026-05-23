mod builtins;
#[allow(unused_imports)]
mod parser;
mod repl;
mod shell;
use rustyline::{Cmd, ConditionalEventHandler, DefaultEditor, EventHandler, KeyEvent};
use std::{env, path::PathBuf};

use crate::parser::Parser;
use crate::repl::REPL;

struct TabEventHandler;
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
        }
        Some(Cmd::Insert(n, String::from("\x07")))
    }
}

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
        // REPL::print_string("$ ");
        let input = rl.readline("$ ").unwrap_or(String::new());
        // let input = REPL::read_input();
        let mut arg_parser = Parser::new();
        let args = arg_parser.parse_arg_string(&input);
        REPL::eval(args, &paths);
    }
}
