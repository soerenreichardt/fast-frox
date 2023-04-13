use std::{iter::{Enumerate, Peekable}, str::Chars};

pub(crate) struct Scanner<'a> {
    source_iterator: Peekable<Enumerate<Chars<'a>>>,
    source: &'a str,
    line: usize,
    start: usize
}

#[derive(Debug, PartialEq)]
pub(crate) struct Token {
    pub(crate) tpe: TokenType,
    pub(crate) start: usize,
    pub(crate) length: usize,
    pub(crate) line: usize,
}

#[derive(Debug, PartialEq)]
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

    Error(String),
    EOF,
}

impl<'a> Scanner<'a> {
    pub(crate) fn new(source: &'a str) -> Self {
        Scanner {
            source_iterator: source.chars().enumerate().peekable(),
            source,
            line: 1,
            start: 0
        }
    }

    pub(crate) fn scan_token(&mut self) -> Token {
        self.skip_whitespace();
        match self.source_iterator.next() {
            None => Token::new(TokenType::EOF, 0, 0, 0),
            Some((pos, c)) => {
                self.start = pos;
                return self.match_char(c);
            }
        };

        self.error("Unexpected character.".to_owned())
    }

    fn match_char(&mut self, c: char) -> Token {
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
            '!' => if self.match_token('=') { 
                self.token(TokenType::BangEqual)
            } else { 
                self.token(TokenType::Bang)
            },
            '=' => if self.match_token('=') {
                self.token(TokenType::EqualEqual)
            } else {
                self.token(TokenType::Equal)
            },
            '<' => if self.match_token('=') {
                self.token(TokenType::LessEqual)
            } else {
                self.token(TokenType::Less)
            },
            '>' => if self.match_token('=') {
                self.token(TokenType::GreaterEqual)
            } else {
                self.token(TokenType::Greater)
            }
            '"' => self.string(),
            c if c.is_digit(10) => self.number(),
            c if c.is_alphabetic() => self.identifier(),
            _ => todo!()
        }
    }

    fn match_token(&mut self, expected: char) -> bool {
        match self.source_iterator.peek() {
            None => false,
            Some((_, c)) if c == &expected => {
                self.source_iterator.next();
                true
            },
            _ => false
        }
    }

    fn token(&mut self, tpe: TokenType) -> Token {
        Token::new(tpe, self.start, self.source_iterator.peek().map(|(pos, _)| pos).unwrap_or(&self.source.len()) - self.start, self.line)
    }

    fn error(&self, message: String) -> Token {
        Token::new(TokenType::Error(message), self.start, self.source.len() - self.start, self.line)
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.source_iterator.peek() {
                None => return,
                Some((pos, c)) => match c {
                    ' ' | '\r' | '\t' => {
                        self.source_iterator.next();
                        ()
                    }
                    '\n' => {
                        self.line += 1;
                        self.source_iterator.next();
                        ()
                    },
                    '/' => match &self.source[*pos+1..*pos+2].chars().peekable().peek() {
                        None => return,
                        Some('/') => {
                            loop {
                                match self.source_iterator.next() {
                                    None | Some((_, '\n')) => break,
                                    _ => ()
                                }
                            }
                        }
                        _ => todo!()
                    }
                    _ => return
                }
            }
        }
    }

    fn string(&mut self) -> Token {
        loop {
            match self.source_iterator.peek() {
                None => return self.error("Unterminated string.".to_owned()),
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

    fn number(&mut self) -> Token {
        self.consume_digits();

        if let Some((pos, '.')) = self.source_iterator.peek() {
            match self.source[*pos+1..*pos+2].chars().peekable().peek() {
                Some( c) if c.is_digit(10) => {
                    self.source_iterator.next();
                    self.consume_digits();
                },
                _ => ()
            }
        }

        self.token(TokenType::Number)
    }

    fn consume_digits(&mut self) {
        loop {
            match self.source_iterator.peek() {
                None => break,
                Some((_, c)) if !c.is_digit(10) => break,
                _ => {
                    self.source_iterator.next();
                    ()
                }
            }
        }
    }

    fn identifier(&mut self) -> Token {
        loop {
            match self.source_iterator.peek() {
                Some((_, c)) if c.is_digit(10) || c.is_alphabetic() => {
                    self.source_iterator.next();
                    ()
                },
                _ => ()
            }
        }

        self.token(TokenType::Identifier)
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_scan_digit() {
        let mut scanner = Scanner::new("1337.42");
        let token = scanner.scan_token();
        assert_eq!(token, Token::new(TokenType::Number, 0, 7, 1));
    }

    #[test]
    fn should_parenthesis() {
        let mut scanner = Scanner::new("(");
        let token = scanner.scan_token();
        assert!(matches!(token, Token { tpe: TokenType::LeftParen, .. }));
    }

    #[test]
    fn should_ignore_comment() {
        let mut scanner = Scanner::new("//foo\n+");
        let token = scanner.scan_token();
        assert!(matches!(token, Token { tpe: TokenType::Plus, .. }));
    }
}
