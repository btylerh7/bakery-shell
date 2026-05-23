use rustyline::{Cmd, ConditionalEventHandler};

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
        }
        Some(Cmd::Insert(n, String::from("\x07")))
    }
}
