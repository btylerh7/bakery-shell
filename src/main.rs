#[allow(unused_imports)]
use std::io::{self, Write};

enum Command { 
}

fn main() {
    // TODO: Uncomment the code below to pass the first stage
    print!("$ ");
    let mut command = String::new();
    io::stdin().read_line(&mut command).unwrap();
    io::stdout().flush().unwrap();
    println!("{}: command not found", command.trim());
}


