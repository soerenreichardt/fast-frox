use crate::{debug::ChunkDebug, op_code::OpCode, value::Value, InstructionSize};

#[derive(Default)]
pub struct Chunk {
    pub(crate) code: Vec<u8>,
    pub(crate) constants: Vec<Value>,
    lines: Vec<Line>,
}

#[derive(Debug)]
struct Line {
    line: usize,
    length: u16,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            code: Vec::new(),
            constants: Vec::new(),
            lines: Vec::new(),
        }
    }

    pub fn write_chunk(&mut self, chunk: u8, line: usize) {
        self.code.push(chunk);
        self.set_line(line);
    }

    pub fn add_constant(&mut self, value: Value) -> u8 {
        self.constants.push(value);
        (self.constants.len() - 1) as u8
    }

    fn set_line(&mut self, line: usize) {
        if !self.lines.is_empty() && self.lines.last().unwrap().line == line {
            let last = self.lines.last_mut().unwrap();
            last.length += 1;
            return;
        }
        self.lines.push(Line { line, length: 1 });
    }

    fn get_line(&self, offset: usize) -> usize {
        let mut length = 0;
        let mut last_line = self.lines.first().unwrap();
        for line in &self.lines {
            if offset < length {
                return last_line.line;
            }
            length += line.length as usize;
            last_line = line;
        }

        if offset < length {
            last_line.line
        } else {
            panic!()
        }
    }
}

impl ChunkDebug<OpCode> for Chunk {
    fn disassemblee_chunk(&self, name: &str) {
        println!("== {} ==", name);
        let mut offset = 0;
        while offset < self.code.len() {
            offset += self.disassemble_instruction(offset);
        }

        println!("== {} ==", name);
    }

    fn disassemble_instruction(&self, offset: usize) -> usize {
        print!("{:0>4} ", offset);
        if offset > 0 && self.get_line(offset) == self.get_line(offset - 1) {
            print!("   | ");
        } else {
            print!("{:>4} ", self.get_line(offset))
        }

        let instruction: OpCode = self.code.get(offset).unwrap().try_into().unwrap();
        match instruction {
            OpCode::OpReturn => println!("OP_RETURN"),
            OpCode::OpConstant => {
                let constant_index = *self.code.get(offset + 1).unwrap() as usize;
                let constant_value = self.constants.get(constant_index).unwrap();
                println!("OP_CONSTANT {:>4} {}", constant_index, constant_value)
            }
            OpCode::OpNegate => println!("OP_NEGATE"),
            OpCode::OpAdd => println!("OP_ADD"),
            OpCode::OpSubtract => println!("OP_SUBTRACT"),
            OpCode::OpMultiply => println!("OP_MULTIPLY"),
            OpCode::OpDivide => println!("OP_DIVIDE"),
        }
        instruction.size()
    }
}
