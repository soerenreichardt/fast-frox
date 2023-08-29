
use crate::{
    chunk::Chunk,
    error::{CompileError, RuntimeError},
    op_code::OpCode,
    scanner::{Scanner, Token, TokenType},
    value::Value, debug::ChunkDebug,
};
use miette::{NamedSource, Result};

pub(crate) struct Compiler<'a> {
    parser: Parser,
    source: &'a str,
    scanner: Scanner<'a>,
    chunk: &'a mut Chunk,
    debug: bool
}

#[derive(Default)]
pub(crate) struct Parser {
    previous: Option<Token>,
    current: Option<Token>,
}

#[repr(u8)]
#[derive(Clone, PartialEq, PartialOrd)]
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

struct ParseRule<'a> {
    prefix_fn: Option<fn(&mut Compiler<'a>) -> Result<()>>,
    infix_fn: Option<fn(&mut Compiler<'a>) -> Result<()>>,
    precedence: Precedence,
}

impl<'a> Compiler<'a> {
    pub(crate) fn new(parser: Parser, source: &'a str, chunk: &'a mut Chunk, debug: bool) -> Self {
        Compiler {
            parser,
            source,
            scanner: Scanner::new(source),
            chunk,
            debug,
        }
    }

    pub(crate) fn compile(&mut self) -> Result<()> {
        self.advance()?;
        self.expression()?;
        self.consume(TokenType::Eof)?;

        self.end_compiler();
        Ok(())
    }

    fn advance(&mut self) -> Result<()> {
        self.parser.previous = self.parser.current.take();

        match self.scanner.next() {
            Some(Ok(token)) => {
                self.parser.current = Some(token);
            }
            Some(Err(error)) => {
                return Err(error);
            }
            None => (),
        };

        Ok(())
    }

    fn consume(&mut self, expected_type: TokenType) -> Result<()> {
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

    fn end_compiler(&mut self) {
        if self.debug {
            self.chunk.disassemblee_chunk("code")
        }
        self.emit_return()
    }

    fn expression(&mut self) -> Result<()> {
        self.parse_precedence(Precedence::Assignment)
    }

    fn number(&mut self) -> Result<()> {
        let previous = self.parser.previous.as_ref().expect("No previous value");
        let string_value = &self.source[previous.start..previous.start + previous.length];
        let value = string_value.parse::<f64>().unwrap();

        self.emit_constant(Value::Number(value))
    }

    fn grouping(&mut self) -> Result<()> {
        self.expression()?;
        self.consume(TokenType::RightParen)
    }

    fn unary(&mut self) -> Result<()> {
        let op_type = self.previous().tpe.clone();
        self.parse_precedence(Precedence::Unary)?;

        match op_type {
            TokenType::Minus => self.emit_byte(OpCode::OpNegate as u8),
            _ => (),
        };
        Ok(())
    }

    fn binary(&mut self) -> Result<()> {
        let operator_type = self.previous().tpe.clone();
        let rule = Compiler::get_rule(&operator_type);
        self.parse_precedence(rule.precedence.next())?;
        match operator_type {
            TokenType::Plus => self.emit_byte(OpCode::OpAdd as u8),
            TokenType::Minus => self.emit_byte(OpCode::OpSubtract as u8),
            TokenType::Star => self.emit_byte(OpCode::OpMultiply as u8),
            TokenType::Slash => self.emit_byte(OpCode::OpDivide as u8),
            _ => (),
        };
        Ok(())
    }

    fn literal(&mut self) -> Result<()> {
        match self.previous().tpe.clone() {
            TokenType::False => self.emit_byte(OpCode::OpFalse as u8),
            TokenType::True => self.emit_byte(OpCode::OpTrue as u8),
            TokenType::Nil => self.emit_byte(OpCode::OpNil as u8),
            _ => return Err(RuntimeError::new("Not a literal".to_string()).into())
        }
        Ok(())
    }

    fn parse_precedence(&mut self, precedence: Precedence) -> Result<()> {
        self.advance()?;
        let prefix_rule = Compiler::get_rule(&self.previous().tpe)
            .prefix_fn
            .expect("No prefix function found");
        prefix_rule(self)?;

        while precedence <= Compiler::get_rule(&self.current().tpe).precedence {
            self.advance()?;
            let infix_rule = Compiler::get_rule(&self.previous().tpe)
                .infix_fn
                .expect("No infix function found");
            infix_rule(self)?;
        }
        Ok(())
    }

    fn get_rule(operator_type: &TokenType) -> ParseRule<'a> {
        match operator_type {
            TokenType::LeftParen => ParseRule {
                prefix_fn: Some(Compiler::grouping),
                infix_fn: None,
                precedence: Precedence::None,
            },
            TokenType::Minus => ParseRule {
                prefix_fn: Some(Compiler::unary),
                infix_fn: Some(Compiler::binary),
                precedence: Precedence::Term,
            },
            TokenType::Plus => ParseRule {
                prefix_fn: None,
                infix_fn: Some(Compiler::binary),
                precedence: Precedence::Term,
            },
            TokenType::Slash => ParseRule {
                prefix_fn: None,
                infix_fn: Some(Compiler::binary),
                precedence: Precedence::Factor,
            },
            TokenType::Star => ParseRule {
                prefix_fn: None,
                infix_fn: Some(Compiler::binary),
                precedence: Precedence::Factor,
            },
            TokenType::Number => ParseRule {
                prefix_fn: Some(Compiler::number),
                infix_fn: None,
                precedence: Precedence::None,
            },
            TokenType::True | TokenType::False | TokenType::Nil => ParseRule {
                prefix_fn: Some(Compiler::literal),
                infix_fn: None,
                precedence: Precedence::None
            },
            _ => ParseRule {
                prefix_fn: None,
                infix_fn: None,
                precedence: Precedence::None,
            },
        }
    }

    fn emit_byte(&mut self, byte: u8) {
        self.chunk.write_chunk(
            byte,
            self.parser
                .previous
                .as_ref()
                .map(|token| token.line)
                .unwrap_or(0),
        );
    }

    fn emit_bytes(&mut self, byte1: u8, byte2: u8) {
        self.emit_byte(byte1);
        self.emit_byte(byte2);
    }

    fn emit_return(&mut self) {
        self.emit_byte(OpCode::OpReturn as u8)
    }

    fn emit_constant(&mut self, value: Value) -> Result<()> {
        let constant_position = self.make_constant(value)?;
        self.emit_bytes(OpCode::OpConstant as u8, constant_position);
        Ok(())
    }

    fn make_constant(&mut self, value: Value) -> Result<u8> {
        let constant_position = self.chunk.add_constant(value);
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
