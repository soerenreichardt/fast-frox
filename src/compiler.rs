use crate::{
    chunk::Chunk,
    error::CompileError,
    scanner::{Scanner, Token, TokenType},
};
use miette::{ErrReport, NamedSource, Result};

pub(crate) struct Compiler {
    parser: Parser,
}

#[derive(Default)]
pub(crate) struct Parser {
    previous: Option<Token>,
    current: Option<Token>,
    had_error: bool,
    panic_mode: bool,
}

impl Compiler {
    pub(crate) fn new(parser: Parser) -> Self {
        Compiler { parser }
    }

    pub(crate) fn compile(&mut self, source: &str, chunk: &mut Chunk) -> bool {
        let scanner = Scanner::new(source);
        let mut token_iterator = scanner.into_iter();

        self.parser.had_error = false;
        self.parser.panic_mode = false;

        self.advance(&mut token_iterator);
        self.consume(TokenType::Eof, &mut token_iterator, source);

        !self.parser.had_error
    }

    fn advance<I: Iterator<Item = Result<Token>>>(&mut self, token_iterator: &mut I) {
        self.parser.previous = self.parser.current.take();

        loop {
            match token_iterator.next() {
                Some(Ok(token)) => {
                    self.parser.current = Some(token);
                    break;
                }
                Some(Err(error)) => {
                    self.error(error);
                }
                None => break,
            };
        }

        todo!()
    }

    fn error(&mut self, error: ErrReport) {
        if self.parser.panic_mode {
            return;
        }
        self.parser.panic_mode = true;
        println!("{}", error);
        self.parser.had_error = true;
    }

    fn consume<I: Iterator<Item = Result<Token>>>(
        &mut self,
        expected_type: TokenType,
        token_iterator: &mut I,
        src: &str
    ) {
        if self.parser.current.as_ref().map(|token| &token.tpe) == Some(&expected_type) {
            self.advance(token_iterator);
        } else {
            let current_token = self.parser.current.take().unwrap();
            self.error(
                CompileError {
                    msg: format!("Expected token of type {:?}", expected_type),
                    src: NamedSource::new("", src.to_owned()),
                    span: current_token.into(),
                }
                .into()
            )
        }
    }
}
