use crate::{
    chunk::Chunk,
    error::CompileError,
    scanner::{Scanner, Token, TokenType}, op_code::OpCode,
};
use miette::{NamedSource, Result};


pub(crate) struct Compiler {
    parser: Parser,
}

#[derive(Default)]
pub(crate) struct Parser {
    previous: Option<Token>,
    current: Option<Token>,
}

impl Compiler {
    pub(crate) fn new(parser: Parser) -> Self {
        Compiler { parser }
    }

    pub(crate) fn compile(&mut self, source: &str, chunk: &mut Chunk) -> Result<()> {
        let scanner = Scanner::new(source);
        let mut token_iterator = scanner.into_iter();

        self.advance(&mut token_iterator)?;
        self.consume(TokenType::Eof, &mut token_iterator, source)?;

        self.end_compiler(chunk);
        Ok(())
    }

    fn advance<I: Iterator<Item = Result<Token>>>(&mut self, token_iterator: &mut I) -> Result<()> {
        self.parser.previous = self.parser.current.take();

        loop {
            match token_iterator.next() {
                Some(Ok(token)) => {
                    self.parser.current = Some(token);
                    break;
                }
                Some(Err(error)) => {
                    return Err(error);
                }
                None => break,
            };
        }

        Ok(())
    }

    fn consume<I: Iterator<Item = Result<Token>>>(
        &mut self,
        expected_type: TokenType,
        token_iterator: &mut I,
        src: &str,
    ) -> Result<()> {
        if self.parser.current.as_ref().map(|token| &token.tpe) == Some(&expected_type) {
            self.advance(token_iterator)?;
            return Ok(())
        } else {
            let current_token = self.parser.current.take().unwrap();
            Err(
                CompileError {
                    msg: format!("Expected token of type {:?}", expected_type),
                    src: NamedSource::new("", src.to_owned()),
                    span: current_token.into(),
                }.into()
            )
        }
    }

    fn emit_byte(&self, byte: u8, chunk: &mut Chunk) {
        chunk.write_chunk(
            byte,
            self.parser
                .previous
                .as_ref()
                .map(|token| token.line)
                .unwrap_or(0),
        );
    }

    fn emit_bytes(&self, byte1: u8, byte2: u8, chunk: &mut Chunk)  {
        self.emit_byte(byte1, chunk);
        self.emit_byte(byte2, chunk);
    }

    fn emit_return(&self, chunk: &mut Chunk) {
        self.emit_byte(OpCode::OpReturn as u8, chunk)
    }

    fn end_compiler(&self, chunk: &mut Chunk) {
        self.emit_return(chunk)
    }
}
