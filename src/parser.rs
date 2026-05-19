
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
   pub fn new() -> Self {
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
                if let Some(prev) = previous_char && prev == '\'' {
                    if let Some(prev) = args.last() && prev.state == ParserState::SingleQuote {
                        let prev_arg = args.pop().unwrap();
                        current_arg.value = prev_arg.value; current_arg.state = ParserState::NoQuote;
                    }
                }
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



pub struct Parser {
    current_arg: String,
    args: Vec<String>,
    previous_char: Option<char>,
    current_state: ParserState
}
impl Parser {
    pub fn new() -> Self {
        return Parser {
            current_arg: String::new(),
            args: vec![],
            previous_char: None,
            current_state: ParserState::NoQuote

        }
    }
   pub fn parse_arg_string(&mut self, input: &str) -> Vec<String> {
        let trimmed = input.trim();
        for char in trimmed.chars() {
            match char {
                '\'' => self.parse_single_quote(),
                char if char.is_whitespace() => self.parse_whitespace(&char),
                char => self.parse_normal_char(&char)
            }
        }
        self.args.clone()
    }
    fn add_current_arg(&mut self) {
        if !self.current_arg.is_empty() {
            self.args.push(self.current_arg.clone());
            self.current_arg = String::new();
        }
    }
    fn parse_whitespace(&mut self, current: &char) {
        match self.current_state {
            ParserState::NoQuote => {
                self.add_current_arg();
                self.current_state = ParserState::NoQuote;
            },
            ParserState::SingleQuote => {
                self.current_arg.push(current.clone());
            }
        }

    }
    fn parse_normal_char(&mut self, char: &char) {
        self.current_arg.push(char.clone());
    }
    fn parse_single_quote(&mut self) {
        if let Some(prev) = self.previous_char {
            match self.current_state {
                // echo banana'orange' -> bananaorange
                // echo 'banana''orange' -> bananaorange
                ParserState::NoQuote => {
                    if vec!['\"', '\''].contains(&prev) {
                        self.current_arg = self.args.pop().unwrap_or(String::new());
                    } 
                    // Start new quote or concatenate current args
                    // ex. current 'current'
                    // or current'current'
                    self.current_state = ParserState::SingleQuote;
                },
                ParserState::SingleQuote => {
                    // Quote is end quote
                    self.args.push(self.current_arg.clone());
                    self.current_arg = String::new();
                    self.current_state = ParserState::NoQuote;
                }
            }
        } else {
            self.current_arg = String::new();
            self.current_state = ParserState::SingleQuote;
        }
    }
}
