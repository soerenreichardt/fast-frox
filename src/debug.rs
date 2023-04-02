use crate::{InstructionSize, op_code::OpCode};

pub trait ChunkDebug<I> where I: InstructionSize + TryInto<OpCode> {
    fn disassemblee_chunk(&self, name: &str);
    fn disassemble_instruction(&self, offset: usize) -> usize;
}