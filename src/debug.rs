use crate::{InstructionSize, value::Value};

pub trait Print {
    fn print(&self, constant: &[Value]);
}

pub trait ChunkDebug<I> where I: Print + InstructionSize {
    fn data(&self) -> &[I];
    fn constants(&self) -> &[Value];
    fn get_line(&self, offset: usize) -> usize;

    fn disassemblee_chunk(&self, name: &str) {
        println!("== {} ==", name);
        let mut offset = 0;
        while offset < self.data().len() {
            offset = self.disassemble_instruction(offset);
        }
    }

    fn disassemble_instruction(&self, offset: usize) -> usize {
        print!("{:0>4} ", offset);
        if offset > 0 && self.get_line(offset) == self.get_line(offset - 1) {
            print!("   | ");
        } else {
            print!("{:>4} ", self.get_line(offset))
        }

        let data = self.data();
        let instruction = data.get(offset);
        match instruction {
            Some(instruction) => {
                instruction.print(self.constants());
                offset + instruction.size()
            }
            None => panic!("Overflow")
        }
    }
}