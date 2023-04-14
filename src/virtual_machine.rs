use std::mem::size_of;

use crate::{chunk::Chunk, debug::ChunkDebug, op_code::OpCode, value::Value, compiler::{Compiler, Parser}, scanner::Token};

pub struct VirtualMachine {
    stack: [Value; 256],
    stack_top: *mut f64,
    compiler: Compiler,
    debug: bool,
}

struct InstructionPointer {
    ptr: *const u8,
}

pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

impl VirtualMachine {
    pub fn new(debug: bool) -> Self {
        let mut stack = [0.0; 256];
        let parser = Parser::default();
        VirtualMachine {
            stack,
            stack_top: stack.as_mut_ptr(),
            compiler: Compiler::new(parser),
            debug,
        }
    }

    pub fn init(&mut self) {
        self.stack_top = self.stack.as_mut_ptr();
    }

    pub fn interpret(&mut self, source: &str) -> InterpretResult {
        let mut chunk = Chunk::new();

        if !self.compiler.compile(source, &mut chunk,) {
            return InterpretResult::CompileError;
        }

        let ip = InstructionPointer::new(&chunk.code);

        self.run(ip, &chunk)
    }

    fn run(&mut self, mut ip: InstructionPointer, chunk: &Chunk) -> InterpretResult {
        loop {
            if self.debug {
                self.debug(&ip, chunk);
            }

            let instruction: OpCode = match (&ip.next()).try_into() {
                Ok(instruction) => instruction,
                Err(_) => return InterpretResult::RuntimeError,
            };
            match instruction {
                OpCode::OpReturn => {
                    println!("{}", self.pop());
                    return InterpretResult::Ok;
                }
                OpCode::OpConstant => {
                    let constant_index = ip.next();
                    let constant_value = chunk.constants.get(constant_index as usize).unwrap();
                    self.push(*constant_value);
                }
                OpCode::OpNegate => unsafe {
                    let addr = self.stack_top.sub(1);
                    *addr = -*addr;
                },
                OpCode::OpAdd => self.binary_operation(std::ops::Add::add),
                OpCode::OpSubtract => self.binary_operation(std::ops::Sub::sub),
                OpCode::OpMultiply => self.binary_operation(std::ops::Mul::mul),
                OpCode::OpDivide => self.binary_operation(std::ops::Div::div),
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

    fn binary_operation<Op: FnOnce(Value, Value) -> Value>(&mut self, op: Op) {
        let rhs = self.pop();
        let lhs = self.pop();
        self.push(op(lhs, rhs));
    }

    fn debug(&self, ip: &InstructionPointer, chunk: &Chunk) {
        for slot_address in
            (self.stack.as_ptr() as usize..self.stack_top as usize).step_by(size_of::<f64>())
        {
            let slot_value = unsafe { *(slot_address as *const f64) };
            println!("[{}]", slot_value)
        }
        let offset = ip.address() - chunk.code.as_ptr() as usize;
        chunk.disassemble_instruction(offset);
    }
}

impl Drop for VirtualMachine {
    fn drop(&mut self) {}
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
