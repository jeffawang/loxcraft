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
        while self.current < self.source.len() {
            self.start = self.current;
            self.scan_token();
        }
        self.add_token(TokenType::EOF, None);

        println!("Source: {}", self.source);
        Ok(&self.tokens)
    }

    fn scan_token(&mut self) {
        if let Some(c) = self.advance() {
            println!("Got {}", c);
            self.current += 1;
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
                _ => {}
            }
        } else {
            todo!("handle this better?")
        }
    }

    fn advance(&mut self) -> Option<char> {
        self.source.chars().nth(self.current as usize)
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<String>) {
        let text = &self.source[self.start..self.current];
        self.tokens
            .push(Token::new(token_type, text, literal, self.line))
    }
}
