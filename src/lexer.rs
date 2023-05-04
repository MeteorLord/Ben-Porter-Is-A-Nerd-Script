use std::str::Chars;

enum Token {
    Comment(String),
    Identifier(String),
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Import,
    If,
    Else,
    Print,
    Error,
}

struct Lexer<'a> {
    input: Chars<'a>,
    current_char: Option<char>,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
        let mut lexer = Lexer {
            input: input.chars(),
            current_char: None,
        };
        lexer.advance();
        lexer
    }

    fn advance(&mut self) {
        self.current_char = self.input.next();
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char {
            if !c.is_whitespace() {
                break;
            }
            self.advance();
        }
    }

    fn skip_comment(&mut self) -> Token {
        let mut comment = String::new();
        while let Some(c) = self.current_char {
            if c == '\n' {
                break;
            }
            comment.push(c);
            self.advance();
        }
        Token::Comment(comment)
    }

    fn get_identifier(&mut self) -> String {
        let mut identifier = String::new();
        while let Some(c) = self.current_char {
            if !c.is_alphanumeric() && c != '_' {
                break;
            }
            identifier.push(c);
            self.advance();
        }
        identifier
    }

    fn get_number(&mut self) -> Token {
        let mut number = String::new();
        let mut is_float = false;
        while let Some(c) = self.current_char {
            if !c.is_digit(10) && c != '.' {
                break;
            }
            if c == '.' {
                if is_float {
                    break;
                }
                is_float = true;
            }
            number.push(c);
            self.advance();
        }
        if is_float {
            Token::Float(number.parse().unwrap())
        } else {
            Token::Integer(number.parse().unwrap())
        }
    }

    fn get_string(&mut self) -> Token {
        let mut string = String::new();
        self.advance();
        while let Some(c) = self.current_char {
            if c == '"' {
                self.advance();
                return Token::String(string);
            }
            string.push(c);
            self.advance();
        }
        Token::Error
    }

    fn get_boolean(&mut self) -> Token {
        if self.current_char == Some('t') {
            self.advance();
            if self.current_char == Some('r') {
                self.advance();
                if self.current_char == Some('u') {
                    self.advance();
                    if self.current_char == Some('e') {
                        self.advance();
                        return Token::Boolean(true);
                    }
                }
            }
        } else if self.current_char == Some('f') {
            self.advance();
            if self.current_char == Some('a') {
                self.advance();
                if self.current_char == Some('l') {
                    self.advance();
                    if self.current_char == Some('s') {
                        self.advance();
                        if self.current_char == Some('e') {
                            self.advance();
                            return Token::Boolean(false);
                        }
                    }
                }
            }
        }
        Token::Error
    }

    fn get_next_token(&mut self) -> Token {
        self.skip_whitespace();
        match self.current_char {
            Some('#') => self.skip_comment(),
            Some('"') => self.get_string(),
            Some(c) => {
                if c.is_alphabetic() {
                    let identifier = self.get_identifier();
                    match identifier.as_str() {
                        "import" => Token::Import,
                        "if" => Token::If,
                        "else" => Token::Else,
                        "print" => Token::Print,
                        "true" => Token::Boolean(true),
                        "false" => Token::Boolean(false),
                        _ => Token::Identifier(identifier),
                    }
                } else if c.is_digit(10) {
                    self.get_number()
                } else {
                    Token::Error
                }
            }
            None => Token::Error,
        }
    }
}
