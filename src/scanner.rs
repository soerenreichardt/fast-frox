use std::{iter::Enumerate, str::Chars};
use miette::{Result, NamedSource, SourceSpan};

use crate::{peek_peek_iterator::PeekPeekIterator, error::CompileError};

pub(crate) struct Scanner<'a> {
    source_iterator: PeekPeekIterator<Enumerate<Chars<'a>>>,
    source: &'a str,
    line: usize,
    start: usize,
}

#[derive(Debug, PartialEq)]
pub(crate) struct Token {
    pub(crate) tpe: TokenType,
    pub(crate) start: usize,
    pub(crate) length: usize,
    pub(crate) line: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    Identifier,
    String,
    Number,

    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

impl<'a> Scanner<'a> {
    pub(crate) fn new(source: &'a str) -> Self {
        Scanner {
            source_iterator: PeekPeekIterator::new(source.chars().enumerate()),
            source,
            line: 1,
            start: 0,
        }
    }

    pub(crate) fn scan_token(&mut self) -> Result<Token> {
        self.skip_whitespace();
        let next_char = self.source_iterator.next();
        match next_char {
            None => return Ok(Token::new(TokenType::Eof, 0, 0, 0)),
            Some((pos, c)) => {
                self.start = pos;
                return self.match_char(c);
            }
        };

        // Err(CompileError {
        //     msg: format!("Unexpected character `{:?}`", next_char),
        //     src: NamedSource::new("", self.source.to_owned()),
        //     span: (self.start, self.source_iterator.peek().map(|(pos, _)| pos - 1).unwrap_or(self.source.len())).into()
        // }.into())
    }

    fn match_char(&mut self, c: char) -> Result<Token> {
        match c {
            '(' => self.token(TokenType::LeftParen),
            ')' => self.token(TokenType::RightParen),
            '{' => self.token(TokenType::LeftBrace),
            '}' => self.token(TokenType::RightBrace),
            ';' => self.token(TokenType::Semicolon),
            ',' => self.token(TokenType::Comma),
            '.' => self.token(TokenType::Dot),
            '+' => self.token(TokenType::Plus),
            '-' => self.token(TokenType::Minus),
            '*' => self.token(TokenType::Star),
            '/' => self.token(TokenType::Slash),
            '!' => {
                if self.match_token('=') {
                    self.token(TokenType::BangEqual)
                } else {
                    self.token(TokenType::Bang)
                }
            }
            '=' => {
                if self.match_token('=') {
                    self.token(TokenType::EqualEqual)
                } else {
                    self.token(TokenType::Equal)
                }
            }
            '<' => {
                if self.match_token('=') {
                    self.token(TokenType::LessEqual)
                } else {
                    self.token(TokenType::Less)
                }
            }
            '>' => {
                if self.match_token('=') {
                    self.token(TokenType::GreaterEqual)
                } else {
                    self.token(TokenType::Greater)
                }
            }
            '"' => self.string(),
            c if c.is_ascii_digit() => self.number(),
            c if c.is_alphabetic() => self.identifier(),
            _ => todo!(),
        }
    }

    fn match_token(&mut self, expected: char) -> bool {
        match self.source_iterator.peek() {
            None => false,
            Some((_, c)) if c == &expected => {
                self.source_iterator.next();
                true
            }
            _ => false,
        }
    }

    fn token(&mut self, tpe: TokenType) -> Result<Token> {
        let source_len = &self.source.len();
        let head_position = self
            .source_iterator
            .peek()
            .map(|(pos, _)| pos)
            .unwrap_or(source_len);
        Ok(Token::new(tpe, self.start, head_position - self.start, self.line))
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.source_iterator.peek() {
                None => return,
                Some((_, c)) => match c {
                    ' ' | '\r' | '\t' => {
                        self.source_iterator.next();
                        ()
                    }
                    '\n' => {
                        self.line += 1;
                        self.source_iterator.next();
                        ()
                    }
                    '/' => match self.source_iterator.peek_peek() {
                        None => return,
                        Some((_, '/')) => loop {
                            match self.source_iterator.next() {
                                None | Some((_, '\n')) => break,
                                _ => (),
                            }
                        },
                        _ => return,
                    },
                    _ => return,
                },
            }
        }
    }

    fn string(&mut self) -> Result<Token> {
        loop {
            match self.source_iterator.peek() {
                None => return Err(CompileError { 
                    msg: "Unterminated string.".to_owned(),
                    src: NamedSource::new("", self.source.to_owned()),
                    span: (self.start, self.source.len()).into()
                }.into()),
                Some((_, '"')) => break,
                Some((_, c)) => {
                    if c == &'\n' {
                        self.line += 1;
                    }
                    self.source_iterator.next();
                    ()
                }
            }
        }

        self.source_iterator.next();
        self.token(TokenType::String)
    }

    fn number(&mut self) -> Result<Token> {
        self.consume_digits();

        if let Some((_, '.')) = self.source_iterator.peek() {
            match self.source_iterator.peek_peek() {
                Some((_, c)) if c.is_ascii_digit() => {
                    self.source_iterator.next();
                    self.consume_digits();
                }
                _ => (),
            }
        }

        self.token(TokenType::Number)
    }

    fn consume_digits(&mut self) {
        loop {
            match self.source_iterator.peek() {
                None => break,
                Some((_, c)) if !c.is_ascii_digit() => break,
                _ => {
                    self.source_iterator.next();
                    ()
                }
            }
        }
    }

    fn identifier(&mut self) -> Result<Token> {
        loop {
            match self.source_iterator.peek() {
                Some((_, c)) if c.is_ascii_digit() || c.is_alphabetic() => {
                    self.source_iterator.next();
                    ()
                }
                _ => break,
            }
        }

        let current_pos = self
            .source_iterator
            .peek()
            .map(|(pos, _)| pos - 1)
            .unwrap_or(self.source.len());
        self.token(self.identifier_type(current_pos))
    }

    // TODO: implement with trie
    fn identifier_type(&self, current_pos: usize) -> TokenType {
        let slice = &self.source[self.start..current_pos];
        match slice {
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
}

impl<'a> Iterator for Scanner<'a> {
    type Item = Result<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.scan_token() {
            // Ok(Token { tpe: TokenType::Eof, .. }) => None,
            token => Some(token)
        }
    }
}

impl Token {
    pub(crate) fn new(tpe: TokenType, start: usize, length: usize, line: usize) -> Self {
        Token {
            tpe,
            start,
            length,
            line,
        }
    }
}

impl From<Token> for SourceSpan {
    fn from(value: Token) -> Self {
        SourceSpan::new(value.start.into(), value.length.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_scan_digit() {
        let mut scanner = Scanner::new("1337.42");
        let token = scanner.scan_token().unwrap();
        assert_eq!(token, Token::new(TokenType::Number, 0, 7, 1));
    }

    #[test]
    fn should_scan_parenthesis() {
        let mut scanner = Scanner::new("(");
        let token = scanner.scan_token().unwrap();
        assert!(matches!(
            token,
            Token {
                tpe: TokenType::LeftParen,
                ..
            }
        ));
    }

    #[test]
    fn should_ignore_comment() {
        let mut scanner = Scanner::new("//foo\n+");
        let token = scanner.scan_token().unwrap();
        assert!(matches!(
            token,
            Token {
                tpe: TokenType::Plus,
                ..
            }
        ));
    }

    #[test]
    fn should_scan_keyword() {
        let mut scanner = Scanner::new("while");
        let token = scanner.scan_token().unwrap();
        assert!(matches!(
            token,
            Token {
                tpe: TokenType::While,
                ..
            }
        ));
    }
}
