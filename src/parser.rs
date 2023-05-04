use crate::lexer::{Lexer, Token};

enum AstNode {
    Program(Vec<AstNode>),
    Comment(String),
    Identifier(String),
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Import(String),
    If(Box<AstNode>, Vec<AstNode>, Vec<AstNode>),
    Print(Vec<AstNode>),
    Error,
}

struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Self {
        let mut lexer = Lexer::new(input);
        let current_token = lexer.get_next_token();
        Parser {
            lexer,
            current_token,
        }
    }

    fn eat(&mut self, token: Token) {
        if self.current_token == token {
            self.current_token = self.lexer.get_next_token();
        }
    }

    fn parse_comment(&mut self) -> AstNode {
        match self.current_token {
            Token::Comment(ref comment) => {
                self.eat(Token::Comment(comment.clone()));
                AstNode::Comment(comment.clone())
            }
            _ => AstNode::Error,
        }
    }

    fn parse_identifier(&mut self) -> AstNode {
        match self.current_token {
            Token::Identifier(ref identifier) => {
                self.eat(Token::Identifier(identifier.clone()));
                AstNode::Identifier(identifier.clone())
            }
            _ => AstNode::Error,
        }
    }

    fn parse_number(&mut self) -> AstNode {
        match self.current_token {
            Token::Integer(number) => {
                self.eat(Token::Integer(number));
                AstNode::Integer(number)
            }
            Token::Float(number) => {
                self.eat(Token::Float(number));
                AstNode::Float(number)
            }
            _ => AstNode::Error,
        }
    }

    fn parse_string(&mut self) -> AstNode {
        match self.current_token {
            Token::String(ref string) => {
                self.eat(Token::String(string.clone()));
                AstNode::String(string.clone())
            }
            _ => AstNode::Error,
        }
    }

    fn parse_boolean(&mut self) -> AstNode {
        match self.current_token {
            Token::Boolean(boolean) => {
                self.eat(Token::Boolean(boolean));
                AstNode::Boolean(boolean)
            }
            _ => AstNode::Error,
        }
    }

    fn parse_import(&mut self) -> AstNode {
        self.eat(Token::Import);
        match self.current_token {
            Token::String(ref string) => {
                self.eat(Token::String(string.clone()));
                AstNode::Import(string.clone())
            }
            _ => AstNode::Error,
        }
    }

    fn parse_factor(&mut self) -> AstNode {
        match self.current_token {
            Token::Comment(_) => self.parse_comment(),
            Token::Identifier(_) => self.parse_identifier(),
            Token::Integer(_) | Token::Float(_) => self.parse_number(),
            Token::String(_) => self.parse_string(),
            Token::Boolean(_) => self.parse_boolean(),
            Token::Import => self.parse_import(),
            Token::If => self.parse_if(),
            Token::Print => self.parse_print(),
            _ => AstNode::Error,
        }
    }

    fn parse_expression(&mut self) -> AstNode {
        self.parse_factor()
    }

    fn parse_if(&mut self) -> AstNode {
        self.eat(Token::If);
        let condition = Box::new(self.parse_expression());
        let mut if_block = Vec::new();
        let mut else_block = Vec::new();
        while self.current_token != Token::Else {
            if_block.push(self.parse_expression());
        }
        self.eat(Token::Else);
        while self.current_token != Token::Error {
            else_block.push(self.parse_expression());
        }
        AstNode::If(condition, if_block, else_block)
    }

    fn parse_print(&mut self) -> AstNode {
        self.eat(Token::Print);
        let mut args = Vec::new();
        while self.current_token != Token::Error {
            args.push(self.parse_expression());
        }
        AstNode::Print(args)
    }

    fn parse(&mut self) -> AstNode {
        let mut program = Vec::new();
        while self.current_token != Token::Error {
            program.push(self.parse_expression());
        }
        AstNode::Program(program)
    }
}
