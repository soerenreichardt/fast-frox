

use std::mem::size_of;

use crate::{chunk::Chunk, op_code::OpCode, debug::ChunkDebug, value::Value};

pub struct VirtualMachine {
    stack: [Value; 256],
    stack_top: *mut f64
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
        let mut stack = [0.0; 256];
        VirtualMachine {
            stack,
            stack_top: stack.as_mut_ptr(), 
        }
    }

    pub fn init(&mut self) {
        self.stack_top = self.stack.as_mut_ptr();
    }

    pub fn interpret(&mut self, chunk: &Chunk) -> InterpretResult {
        let ip = InstructionPointer::new(&chunk.code);
        self.run(ip, chunk)
    }

    fn run(&mut self, mut ip: InstructionPointer, chunk: &Chunk) -> InterpretResult {
        loop {
            if true {
                self.debug(&ip, chunk);
            }
            match (&ip.next()).try_into() {
                Ok(OpCode::OpReturn) => return InterpretResult::Ok,
                Ok(OpCode::OpConstant) => {
                    let constant_index = ip.next();
                    let constant_value = chunk.constants.get(constant_index as usize).unwrap();
                    self.push(*constant_value);
                },
                _ => return InterpretResult::RuntimeError
            }
        }
    }

    fn push(&mut self, value: Value) {
        unsafe {
            *self.stack_top = value;
            self.stack_top = self.stack_top.add(1);
        }
    }

    fn pop(&mut self) -> Value {
        unsafe {
            self.stack_top = self.stack_top.sub(1);
            *self.stack_top
        }
    }

    fn debug(&self, ip: &InstructionPointer, chunk: &Chunk) {
        for slot_address in (self.stack.as_ptr() as usize..self.stack_top as usize).step_by(size_of::<f64>()) {
            let slot_value = unsafe { *(slot_address as *const f64) };
            println!("[{}]", slot_value)
        }
        let offset = ip.address() - chunk.code.as_ptr() as usize;
        chunk.disassemble_instruction(offset);
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