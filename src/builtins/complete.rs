use crate::{shell::CommandError};
use std::collections::HashMap;


pub fn handle_complete(args: &Vec<String>, completions: &mut HashMap<String, String>) -> Result<String, CommandError> {
    if args.len() > 1 && args[1].starts_with("-") {
        match args[1].as_str() {
            "-p" => {
                if args.len() >= 3 && !args[2].is_empty() {
                    println!("{:?}", completions);
                    if let Some(path) = completions.get(&args[2]) {
                        let message = format!("complete -C '{}' {}", path, &args[2] );
                        return Ok(message)
                    }
                }
                let message = format!("complete: {}: no completion specification", &args[2]).to_string();
                return Ok(message)
            },
            "-C" => {
                if args.len() >= 4 && !args[2].is_empty() && !args[3].is_empty() {
                    let key = args[3].clone().to_string();
                    let value = args[2].clone().to_string();
                    completions.insert(key, value);
                }
            }
            _ => {println!("Args were {:?}", &args)}
        }
    }
    Ok(String::new())
}
