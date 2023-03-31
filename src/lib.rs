pub mod chunk;
pub mod debug;
pub mod op_code;
pub mod value;

pub trait InstructionSize {
    fn size(&self) -> usize;
}