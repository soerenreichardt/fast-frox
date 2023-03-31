

use crate::{chunk::Chunk, op_code::OpCode, debug::ChunkDebug};

pub struct VirtualMachine {
}

struct InstructionPointer<'a> {
    code: &'a [OpCode],
    offset: usize
}

pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError
}

impl VirtualMachine {
    pub fn new() -> Self {
        VirtualMachine {
        }
    }

    pub fn init(&self) {}

    pub fn interpret(&self, chunk: &Chunk) -> InterpretResult {
        let ip = InstructionPointer::new(&chunk.code);
        self.run(ip, chunk)
    }

    fn run(&self, mut ip: InstructionPointer, chunk: &Chunk) -> InterpretResult {
        loop {
            match ip.next() {
                OpCode::OpReturn => return InterpretResult::Ok,
                OpCode::OpConstant(index) => {
                    let constant = chunk.constants().get(*index as usize).unwrap();
                    println!("{}", constant)
                },
            }
        }
    }
}

impl Drop for VirtualMachine {
    fn drop(&mut self) {
        
    }
}

impl<'a> InstructionPointer<'a> {
    fn new(code: &'a [OpCode]) -> Self {
        InstructionPointer { code, offset: 0 }
    }

    fn next(&mut self) -> &OpCode {
        let op_code = unsafe { self.code.get_unchecked(self.offset) };
        self.offset = self.offset + 1;
        op_code
    }
}