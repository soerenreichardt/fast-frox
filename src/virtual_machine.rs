

use crate::{chunk::Chunk, op_code::OpCode, debug::ChunkDebug};

#[derive(Default)]
pub struct VirtualMachine {
}

struct InstructionPointer {
    ptr: *const u8
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
            if true {
                let offset = ip.address() - chunk.code.as_ptr() as usize;
                chunk.disassemble_instruction(offset);
            }
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
        InstructionPointer { ptr: code.as_ptr() }
    }

    fn next(&mut self) -> u8 {
        unsafe { 
            let value = self.ptr.read();
            self.ptr = self.ptr.add(1);
            value
        }
    }

    fn address(&self) -> usize {
        self.ptr as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_iterate_with_instruction_pointer() {
        let data = vec![0, 1, 2, 3];
        let mut ip = InstructionPointer::new(&data);

        assert_eq!(ip.next(), 0);
        assert_eq!(ip.next(), 1);
        assert_eq!(ip.next(), 2);
        assert_eq!(ip.next(), 3);
    }
}