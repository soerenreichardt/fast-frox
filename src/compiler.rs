use crate::scanner::{Scanner, TokenType};

pub(crate) struct Compiler {}

impl Compiler {
    pub(crate) fn compile(source: &str) {
        let mut scanner = Scanner::new(source);
        let mut line = usize::MAX;

        loop {
            let token = scanner.scan_token();
            if token.line != line {
                println!("{:>4}", token.line);
                line = token.line;
            } else {
                println!("   | ")
            }

            let lexeme = &source[token.start..token.start + token.length];
            println!("{:?} '{}'", token.tpe, lexeme);

            match token.tpe {
                TokenType::Eof => break,
                _ => todo!(),
            }
        }
    }
}
