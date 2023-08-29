use crate::InstructionSize;

#[repr(u8)]
#[derive(Debug)]
pub enum OpCode {
    OpReturn = 0,
    OpConstant = 1,
    OpNegate = 2,
    OpAdd = 3,
    OpSubtract = 4,
    OpMultiply = 5,
    OpDivide = 6,
    OpNil = 7,
    OpTrue = 8,
    OpFalse = 9
}

impl InstructionSize for OpCode {
    fn size(&self) -> usize {
        match self {
            Self::OpReturn
            | Self::OpNegate
            | Self::OpAdd
            | Self::OpSubtract
            | Self::OpMultiply
            | Self::OpDivide
            | Self::OpNil
            | Self::OpTrue
            | Self::OpFalse => 1,
            Self::OpConstant => 2,
        }
    }
}

impl TryFrom<&u8> for OpCode {
    type Error = String;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(OpCode::OpReturn),
            1 => Ok(OpCode::OpConstant),
            2 => Ok(OpCode::OpNegate),
            3 => Ok(OpCode::OpAdd),
            4 => Ok(OpCode::OpSubtract),
            5 => Ok(OpCode::OpMultiply),
            6 => Ok(OpCode::OpDivide),
            7 => Ok(OpCode::OpNil),
            8 => Ok(OpCode::OpTrue),
            9 => Ok(OpCode::OpFalse),
            _ => Err("unknown value".to_string()),
        }
    }
}
