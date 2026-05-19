
#[derive(Clone, Debug, PartialEq)]
enum ParserState {
    SingleQuote,
    NoQuote,
}

#[derive(Clone, Debug)]
struct Arg {
    value: String,
    state: ParserState
}
impl Arg {
    fn new() -> Self {
        return Arg {
            value: String::new(),
            state: ParserState::NoQuote
        }
    }
}

pub fn parse_arg_string(input: &str) -> Vec<String> {
    let mut current_arg = Arg::new();
    let mut args: Vec<Arg> = vec![];
    let mut previous_char: Option<char> = None;
    let trimmed_input = input.trim();
    println!("Input to parser was {}", input.trim());

    for thing in trimmed_input.chars() {
        match thing {
            // If the last character was a ', the current character is a ', and
            // it is the start of a new pair of ', concatenate the strings
            '\'' if let Some(prev) = previous_char && prev == '\'' => {
                if let Some(prev) = args.last() && prev.state == ParserState::SingleQuote {
                    let prev_arg = args.pop().unwrap();
                    current_arg.value = prev_arg.value;
                    current_arg.state = ParserState::SingleQuote;
                }
            },
            '\'' => {
                args.push(current_arg.clone());
                current_arg = Arg::new();
                current_arg.state = match current_arg.state {
                    ParserState::SingleQuote => ParserState::NoQuote,
                    ParserState::NoQuote => ParserState::SingleQuote
                };
            },
            thing if thing.is_whitespace() => {
                match current_arg.state {
                    ParserState::SingleQuote => current_arg.value.push(thing),
                    ParserState::NoQuote => {
                        args.push(current_arg.clone());
                        current_arg = Arg::new();
                    }
                }
            },
            _ => {
                current_arg.value.push(thing);
            }
        }
        previous_char = Some(thing.clone());
    }
    if !current_arg.value.is_empty() {
        args.push(current_arg);
    }
    let result = args
        .into_iter()
        .map(|arg| {
            arg.value.trim().to_string()
        })
        .filter(|arg| {
            !arg.is_empty()
        })
        .collect();
    result

}

