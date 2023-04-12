use crate::{op_code::OpCode, InstructionSize};

pub trait ChunkDebug<I>
where
    I: InstructionSize + TryInto<OpCode>,
{
    fn disassemblee_chunk(&self, name: &str);
    fn disassemble_instruction(&self, offset: usize) -> usize;
}
