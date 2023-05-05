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
    scanner: Scanner<'a>
}

#[derive(Default)]
pub(crate) struct Parser {
    previous: Option<Token>,
    current: Option<Token>,
}

#[repr(u8)]
#[derive(Clone)]
enum Precedence {
    None,
    Assignment,
    Or,
    And,
    Equality,
    Comparison,
    Term,
    Factor,
    Unary,
    Call,
    Primary,
}

struct ParseRule {
    prefix_fn: fn() -> (),
    infix_fn: Option<fn() -> ()>,
    precedence: Precedence,
}

impl<'a,> Compiler<'a> {
    pub(crate) fn new(parser: Parser, source: &'a str) -> Self {
        Compiler { parser, source, scanner: Scanner::new(source) }
    }

    pub(crate) fn compile(&mut self, chunk: &mut Chunk) -> Result<()> {
        self.advance()?;
        self.expression()?;
        self.consume(TokenType::Eof)?;

        self.end_compiler(chunk);
        Ok(())
    }

    fn advance(&mut self) -> Result<()> {
        self.parser.previous = self.parser.current.take();

        loop {
            match self.scanner.next() {
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

    fn consume(
        &mut self,
        expected_type: TokenType,
    ) -> Result<()> {
        if self.parser.current.as_ref().map(|token| &token.tpe) == Some(&expected_type) {
            self.advance()?;
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

    fn expression(&self) -> Result<()> {
        self.parse_precedence(Precedence::Assignment);
        todo!()
    }

    fn number(&self, chunk: &mut Chunk) -> Result<()> {
        let previous = self.parser.previous.as_ref().expect("No previous value");
        let string_value = &self.source[previous.start..previous.start + previous.length];
        let value = string_value.parse::<Value>().unwrap();

        self.emit_constant(value as Value, chunk)
    }

    fn grouping(&mut self) -> Result<()> {
        self.expression()?;
        self.consume(TokenType::RightParen)
    }

    fn unary(&self, chunk: &mut Chunk) -> Result<()> {
        let op_type = &self.previous().tpe;
        self.parse_precedence(Precedence::Unary);

        match op_type {
            TokenType::Minus => self.emit_byte(OpCode::OpSubtract as u8, chunk),
            _ => (),
        };
        Ok(())
    }

    fn binary(&self, chunk: &mut Chunk) -> Result<()> {
        let operator_type = &self.previous().tpe;
        let rule = self.get_rule(operator_type);
        self.parse_precedence(rule.precedence.next());
        match operator_type {
            TokenType::Plus => self.emit_byte(OpCode::OpAdd as u8, chunk),
            TokenType::Minus => self.emit_byte(OpCode::OpSubtract as u8, chunk),
            TokenType::Star => self.emit_byte(OpCode::OpMultiply as u8, chunk),
            TokenType::Slash => self.emit_byte(OpCode::OpDivide as u8, chunk),
            _ => (),
        };
        Ok(())
    }

    fn parse_precedence(&self, precedence: Precedence) {}

    fn get_rule(&self, operator_type: &TokenType) -> ParseRule {
        todo!()
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

    fn emit_constant(&self, value: Value, chunk: &mut Chunk) -> Result<()> {
        let constant_position = self.make_constant(value, chunk)?;
        self.emit_bytes(OpCode::OpConstant as u8, constant_position, chunk);
        Ok(())
    }

    fn make_constant(&self, value: Value, chunk: &mut Chunk) -> Result<u8> {
        let constant_position = chunk.add_constant(value);
        match constant_position {
            u8::MAX => {
                let previous = self.previous();
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

    fn current(&self) -> &Token {
        self.parser.current.as_ref().expect("No value present")
    }

    fn previous(&self) -> &Token {
        self.parser.previous.as_ref().expect("No value present")
    }
}

impl Precedence {
    fn next(&self) -> Precedence {
        let enum_value: u8 = self.clone() as u8;
        (enum_value + 1).try_into().unwrap()
    }
}

impl TryFrom<u8> for Precedence {
    type Error = String;

    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Precedence::None),
            1 => Ok(Precedence::Assignment),
            2 => Ok(Precedence::Or),
            3 => Ok(Precedence::And),
            4 => Ok(Precedence::Equality),
            5 => Ok(Precedence::Comparison),
            6 => Ok(Precedence::Term),
            7 => Ok(Precedence::Factor),
            8 => Ok(Precedence::Unary),
            9 => Ok(Precedence::Call),
            10 => Ok(Precedence::Primary),
            _ => Err("unknown value".to_string()),
        }
    }
}
