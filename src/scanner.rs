use anyhow::bail;

use crate::token::{Token, TokenType};

pub(crate) struct Scanner<'a> {
    source: &'a String,
    tokens: Vec<Token<'a>>,
    current: usize,
    start: usize,
    line: usize,
}

impl Scanner<'_> {
    pub fn new(source: &String) -> Scanner {
        Scanner {
            source,
            tokens: vec![],
            current: 0,
            start: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> anyhow::Result<&Vec<Token>> {
        // TODO(jw): could this panic?
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.add_token(TokenType::EOF, None);

        Ok(&self.tokens)
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) {
        if let Some(c) = self.advance() {
            match c {
                '(' => self.add_token(TokenType::LeftParen, None),
                ')' => self.add_token(TokenType::RightParen, None),
                '{' => self.add_token(TokenType::LeftBrace, None),
                '}' => self.add_token(TokenType::RightBrace, None),
                ',' => self.add_token(TokenType::Comma, None),
                '.' => self.add_token(TokenType::Dot, None),
                '-' => self.add_token(TokenType::Minus, None),
                '+' => self.add_token(TokenType::Plus, None),
                ';' => self.add_token(TokenType::Semicolon, None),
                '*' => self.add_token(TokenType::Star, None),
                // TODO: this is so verbose :/
                '!' => {
                    if self.match_char('=') {
                        self.add_token(TokenType::BangEqual, None)
                    } else {
                        self.add_token(TokenType::Bang, None)
                    }
                }
                '=' => {
                    if self.match_char('=') {
                        self.add_token(TokenType::EqualEqual, None)
                    } else {
                        self.add_token(TokenType::Equal, None)
                    }
                }
                '<' => {
                    if self.match_char('=') {
                        self.add_token(TokenType::LessEqual, None)
                    } else {
                        self.add_token(TokenType::Less, None)
                    }
                }
                '>' => {
                    if self.match_char('=') {
                        self.add_token(TokenType::GreaterEqual, None)
                    } else {
                        self.add_token(TokenType::Greater, None)
                    }
                }
                '/' => {
                    if self.match_char('/') {
                        // TODO: maybe add a token if we care about comments
                        while self.peek() != '\n' {
                            self.advance();
                        }
                    } else {
                        self.add_token(TokenType::Slash, None)
                    }
                }
                '0'..='9' => self.number(),
                ' ' => {}
                '\r' => {}
                '\t' => {}
                '"' => self.string(),
                '\n' => self.line += 1,
                '_' | 'a'..='z' | 'A'..='Z' => self.identifier(),
                _ => println!("Unexpected character at {}:{}", self.line, self.current),
            }
        } else {
            todo!("handle this better?")
        }
    }

    fn identifier(&mut self) {
        while Self::is_identifier_char(self.peek()) && !self.is_at_end() {
            self.advance();
        }
        let id = &self.source[self.start..self.current];
        self.add_token(Self::identifier_tokentype(id), None)
    }

    fn identifier_tokentype(id: &str) -> TokenType {
        match id {
            "and" => TokenType::And,
            "class" => TokenType::Class,
            "else" => TokenType::Else,
            "false" => TokenType::False,
            "for" => TokenType::For,
            "fun" => TokenType::Fun,
            "if" => TokenType::If,
            "nil" => TokenType::Nil,
            "or" => TokenType::Or,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            "super" => TokenType::Super,
            "this" => TokenType::This,
            "true" => TokenType::True,
            "var" => TokenType::Var,
            "while" => TokenType::While,
            _ => TokenType::Identifier,
        }
    }

    fn is_identifier_char(c: char) -> bool {
        match c {
            'a'..='z' | 'A'..='Z' | '_' | '0'..='9' => true,
            _ => false,
        }
    }

    fn number(&mut self) {
        while self.peek().is_digit(10) {
            self.advance();
        }
        if self.peek() == '.' && self.peek_next().is_digit(10) {
            self.advance();
            while self.peek().is_digit(10) {
                self.advance();
            }
        }
        // TODO: this is a string
        let value = &self.source[self.start..self.current];
        self.add_token(TokenType::Number, Some(String::from(value)))
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_at_end() {
            // TODO: handle this better
            eprintln!("Unterminated string.");
            return;
        }
        self.advance();
        let value = &self.source[self.start + 1..self.current - 1];
        self.add_token(TokenType::String, Some(String::from(value)))
    }

    fn peek(&mut self) -> char {
        match self.source.chars().nth(self.current) {
            Some(sc) => sc,
            None => '\0',
        }
    }

    fn peek_next(&self) -> char {
        match self.source.chars().nth(self.current + 1) {
            Some(sc) => sc,
            None => '\0',
        }
    }

    fn match_char(&mut self, c: char) -> bool {
        match self.source.chars().nth(self.current) {
            Some(sc) => {
                self.current += 1;
                sc == c
            }
            None => false,
        }
    }

    fn advance(&mut self) -> Option<char> {
        self.current += 1;
        self.source.chars().nth(self.current - 1)
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<String>) {
        let text = &self.source[self.start..self.current];
        self.tokens
            .push(Token::new(token_type, text, literal, self.line))
    }
}
