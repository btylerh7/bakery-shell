enum ParserState {
    SingleQuote,
    NoQuote,
}

pub fn parse_arg_string(input: &str) -> Vec<String> {
    let mut current_state = ParserState::NoQuote;
    let mut current_arg = String::new();
    let mut result: Vec<String> = vec![];

    for thing in input.chars() {
        match thing {
            '\'' => match current_state {
                ParserState::NoQuote => {
                    result.push(current_arg.clone());
                    current_arg = String::new();
                    current_state = ParserState::SingleQuote;
                }
                ParserState::SingleQuote => {
                    result.push(current_arg.clone());
                    current_arg = String::new();
                    current_state = ParserState::NoQuote;
                }
            },
            thing if thing.is_whitespace() => match current_state {
                ParserState::SingleQuote => current_arg.push(thing),
                ParserState::NoQuote => {
                    result.push(current_arg.clone());
                    current_arg = String::new();
                }
            },
            _ => current_arg.push(thing),
        }
    }

    result
        .into_iter()
        .filter(|result_string| !result_string.is_empty())
        .collect()
}
