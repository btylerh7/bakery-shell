#[derive(Clone, Debug, PartialEq)]
enum ParserState {
    SingleQuote,
    NoQuote,
    DoubleQuote
}

struct CharState {
    is_escaped: bool,
}

pub struct Parser {
    current_arg: String,
    args: Vec<String>,
    previous_char: Option<char>,
    current_state: ParserState,
    current_char_state: CharState
}
impl Parser {
    pub fn new() -> Self {
        return Parser {
            current_arg: String::new(),
            args: vec![],
            previous_char: None,
            current_state: ParserState::NoQuote,
            current_char_state: CharState {is_escaped: false}
        };
    }
    pub fn parse_arg_string(&mut self, input: &str) -> Vec<String> {
        let trimmed = input.trim();
        for char in trimmed.chars() {
            match char {
                '\\' => self.parse_escape_char(),
                char if self.current_char_state.is_escaped == true => {
                    self.current_arg.push(char.clone());
                    self.current_char_state.is_escaped = false
                },
                '\'' => self.parse_single_quote(),
                '\"' => self.parse_double_quote(),
                char if char.is_whitespace() => self.parse_whitespace(&char),
                char => self.parse_normal_char(&char),
            }
            self.previous_char = Some(char.clone());
        }
        self.add_current_arg(); // Add the last arg after looping
        self.args.clone()
    }
    fn add_current_arg(&mut self) {
        if !self.current_arg.is_empty() {
            self.args.push(self.current_arg.clone());
            self.current_arg = String::new();
        }
    }
    /// If an argument needs to be concatenated, for example 'test'test2. an argument
    /// 'test' has already been parsed but it needs to be concatenaded to test2.
    fn concat_arg(&mut self) {
        self.current_arg = self.args.pop().unwrap_or(String::new());
    }
    fn parse_escape_char(&mut self) {
        if let Some(prev) = self.previous_char && prev == '\\' {
            self.current_char_state.is_escaped = false;
            self.current_arg.push('\\');
        } else {
            self.current_char_state.is_escaped = true
        }
    }
    fn parse_whitespace(&mut self, current: &char) {
        match self.current_state {
            ParserState::NoQuote => {
                self.add_current_arg();
                self.current_state = ParserState::NoQuote;
            }
            _ => {
                self.current_arg.push(current.clone());
            }
        }
    }
    fn parse_normal_char(&mut self, char: &char) {
        if let Some(prev) = self.previous_char {
            match self.current_state {
                ParserState::NoQuote => {
                    if ['\'', '\"'].contains(&prev) {
                        self.concat_arg();
                    }
                },
                _ => {}
            }
        }
        self.current_arg.push(char.clone());
    }
    fn parse_single_quote(&mut self) {
        if let Some(prev) = self.previous_char {
            match self.current_state {
                // echo banana'orange' -> bananaorange
                // echo 'banana''orange' -> bananaorange
                ParserState::NoQuote => {
                    if matches!(&prev, '\"'|'\'') {
                        self.concat_arg();
                    }
                    // Start new quote or concatenate current args
                    // ex. current 'current'
                    // or current'current'
                    self.current_state = ParserState::SingleQuote;
                },
                ParserState::DoubleQuote => {
                    self.current_arg.push('\'');
                }
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
    fn parse_double_quote(&mut self) {
        if let Some(prev) = self.previous_char {
            match self.current_state {
                // echo banana'orange' -> bananaorange
                // echo 'banana''orange' -> bananaorange
                ParserState::NoQuote => {
                    if matches!(&prev, '\"'|'\'') {
                        self.concat_arg();
                    }
                    // Start new quote or concatenate current args
                    // ex. current 'current'
                    // or current'current'
                    self.current_state = ParserState::DoubleQuote;
                },
                ParserState::SingleQuote => {
                    self.current_arg.push('\"');
                }
                ParserState::DoubleQuote => {
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
