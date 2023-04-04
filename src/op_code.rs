use crate::{InstructionSize};

#[repr(u8)]
#[derive(Debug)]
pub enum OpCode  {
    OpReturn = 0,
    OpConstant = 1
}

impl InstructionSize for OpCode {
    fn size(&self) -> usize {
        match self {
            Self::OpReturn => 1,
            Self::OpConstant => 2
        }
    }
}

impl TryFrom<&u8> for OpCode {
    type Error = String;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(OpCode::OpReturn),
            1 => Ok(OpCode::OpConstant),
            _ => Err("unknown value".to_string())
        }
    }
}
