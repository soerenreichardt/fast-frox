pub mod chunk;
pub(crate) mod compiler;
pub mod debug;
pub mod op_code;
pub(crate) mod peek_peek_iterator;
pub(crate) mod scanner;
pub mod value;
pub mod virtual_machine;

pub trait InstructionSize {
    fn size(&self) -> usize;
}
