use crate::{repl::REPL, shell::CommandError};

pub fn handle_complete(args: &Vec<String>) -> Result<String, CommandError> {
    if args.len() > 1 && args[1].starts_with("-") {
        match args[1].as_str() {
            "-p" => {
                let message = format!("complete: {}: no completion specification", &args[2]).to_string();
                REPL::print_string(&message);
            },
            _ => {}
        }
    }
    Ok(String::new())
}
