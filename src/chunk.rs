use crate::{debug::ChunkDebug, op_code::OpCode, value::Value};

pub struct Chunk {
    code: Vec<OpCode>,
    constants: Vec<Value>,
    lines: Vec<Line>
}

#[derive(Debug)]
struct Line {
    line: usize,
    length: u16
}

impl Chunk {
    pub fn new() -> Self {
        Chunk { code: Vec::new(), constants: Vec::new(), lines: Vec::new() }
    }

    pub fn write_chunk(&mut self, chunk: OpCode, line: usize) {
        self.code.push(chunk);
        self.set_line(line);
    }

    pub fn add_constant(&mut self, value: Value) -> u8 {
        self.constants.push(value);
        (self.constants.len() - 1) as u8
    }

    fn set_line(&mut self, line: usize) {
        if self.lines.len() > 0 && self.lines.last().unwrap().line == line {
            let last = self.lines.last_mut().unwrap();
            last.length = last.length + 1;
            return
        }
        self.lines.push(Line { line, length: 1 });
    }
}

impl ChunkDebug<OpCode> for Chunk {
    fn data(&self) -> &[OpCode] {
        &self.code
    }

    fn constants(&self) -> &[Value] {
        &self.constants
    }

    fn get_line(&self, offset: usize) -> usize {
        let mut length = 0;
        let mut last_line = self.lines.first().unwrap();
        for line in &self.lines {
            if offset < length {
                return last_line.line
            }
            length = length + line.length as usize;
            last_line = line;
        }

        if offset < length {
            return last_line.line
        } else {
            panic!()
        }
    }
}
