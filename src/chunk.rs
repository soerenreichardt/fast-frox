use crate::{debug::ChunkDebug, op_code::OpCode, value::Value};

pub struct Chunk {
    code: Vec<OpCode>,
    constants: Vec<Value>,
    lines: Vec<usize>
}

impl Chunk {
    pub fn new() -> Self {
        Chunk { code: Vec::new(), constants: Vec::new(), lines: Vec::new() }
    }

    pub fn write_chunk(&mut self, chunk: OpCode, line: usize) {
        self.code.push(chunk);
        self.lines.push(line);
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }
}

impl ChunkDebug<OpCode> for Chunk {
    fn data(&self) -> &[OpCode] {
        &self.code
    }

    fn constants(&self) -> &[Value] {
        &self.constants
    }

    fn lines(&self) -> &[usize] {
        &self.lines
    }


}
