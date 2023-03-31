use crate::{InstructionSize, debug::Print};

pub enum OpCode  {
    OpReturn,
    OpConstant(usize)
}

impl Print for OpCode {
    fn print(&self, constants: &[crate::value::Value]) {
        match self {
            Self::OpReturn => println!("OP_RETURN"),
            Self::OpConstant(index) => {
                let value = constants.get(*index).unwrap();
                println!("OP_CONSTANT {:>4} {}", index, value)
            },
        }
    }
}

impl InstructionSize for OpCode {
    fn size(&self) -> usize {
        match self {
            Self::OpReturn | Self::OpConstant(_) => 1
        }
    }
}