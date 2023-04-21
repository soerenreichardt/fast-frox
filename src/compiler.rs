use crate::{
    chunk::Chunk,
    error::CompileError,
    op_code::OpCode,
    scanner::{Scanner, Token, TokenType},
    value::Value,
};
use miette::{NamedSource, Result};

pub(crate) struct Compiler<'a> {
    parser: Parser,
    source: &'a str,
}

#[derive(Default)]
pub(crate) struct Parser {
    previous: Option<Token>,
    current: Option<Token>,
}

impl<'a> Compiler<'a> {
    pub(crate) fn new(parser: Parser, source: &'a str) -> Self {
        Compiler { parser, source }
    }

    pub(crate) fn compile(&mut self, chunk: &mut Chunk) -> Result<()> {
        let scanner = Scanner::new(self.source);
        let mut token_iterator = scanner.into_iter();

        self.advance(&mut token_iterator)?;
        self.expression()?;
        self.consume(TokenType::Eof, &mut token_iterator)?;

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
        token_iterator: &mut I
    ) -> Result<()> {
        if self.parser.current.as_ref().map(|token| &token.tpe) == Some(&expected_type) {
            self.advance(token_iterator)?;
            return Ok(());
        } else {
            let current_token = self.parser.current.take().unwrap();
            Err(CompileError {
                msg: format!("Expected token of type {:?}", expected_type),
                src: NamedSource::new("", self.source.to_owned()),
                span: current_token.into(),
            }
            .into())
        }
    }

    fn end_compiler(&self, chunk: &mut Chunk) {
        self.emit_return(chunk)
    }

    fn number(&self, chunk: &mut Chunk) {
        let previous = self.parser.previous.as_ref().expect("No previous value");
        let string_value = &self.source[previous.start..previous.start + previous.length];
        println!("here {}", string_value);
        let value = string_value.parse::<Value>().unwrap();
    
        self.emit_constant(value as Value, chunk)
    }

    fn grouping<I: Iterator<Item = Result<Token>>>(&mut self, token_iterator: &mut I) -> Result<()> {
        self.expression()?;
        self.consume(TokenType::RightParen, token_iterator)
    }

    fn unary(&self, chunk: &mut Chunk) -> Result<()> {
        let op_type = &self.parser.previous.as_ref().expect("No value present").tpe;
        self.expression()?;

        match op_type {
            TokenType::Minus => self.emit_byte(OpCode::OpSubtract as u8, chunk),
            _ => ()
        };
        Ok(())
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

    fn emit_bytes(&self, byte1: u8, byte2: u8, chunk: &mut Chunk) {
        self.emit_byte(byte1, chunk);
        self.emit_byte(byte2, chunk)
    }

    fn emit_return(&self, chunk: &mut Chunk) {
        self.emit_byte(OpCode::OpReturn as u8, chunk)
    }

    fn emit_constant(&self, value: Value, chunk: &mut Chunk) {
        self.emit_bytes(OpCode::OpConstant as u8, self.make_constant(value, chunk)?, chunk)
    }

    fn make_constant(&self, value: Value, chunk: &mut Chunk) -> Result<u8> {
        let constant_position = chunk.add_constant(value);
        match constant_position {
            u8::MAX => {
                let previous = self.parser.previous.as_ref().expect("No previous value");
                Err(CompileError {
                    msg: "Too many constants in one chunk.".to_owned(),
                    src: NamedSource::new("", self.source.to_owned()),
                    span: (previous.start, previous.length).into(),
                }
                .into())
            }
            _ => Ok(constant_position),
        }
    }

    fn expression(&self) -> Result<()> {
        todo!()
    }
}
