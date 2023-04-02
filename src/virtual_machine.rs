

use crate::{chunk::Chunk, op_code::OpCode};

#[derive(Default)]
pub struct VirtualMachine {
}

struct InstructionPointer {
    ptr: *const u8,
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
            match (&ip.next()).try_into() {
                Ok(OpCode::OpReturn) => return InterpretResult::Ok,
                Ok(OpCode::OpConstant) => {
                    let constant_index = ip.next();
                    let constant_value = chunk.constants.get(constant_index as usize).unwrap();
                    println!("{}", constant_value);
                },
                _ => return InterpretResult::RuntimeError
            }
        }
    }
}

impl Drop for VirtualMachine {
    fn drop(&mut self) {
        
    }
}

impl InstructionPointer {
    fn new(code: &[u8]) -> Self {
        InstructionPointer { ptr: code.as_ptr(), offset: 0 }
    }

    fn next(&mut self) -> u8 {
        let op_code = unsafe { self.ptr.add(self.offset).read() };
        self.offset += 1;
        op_code
    }
}