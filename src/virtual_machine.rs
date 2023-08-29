use std::mem::size_of;

use crate::{chunk::Chunk, debug::ChunkDebug, op_code::OpCode, value::Value, compiler::{Compiler, Parser}, error::RuntimeError};
use miette::Result;

pub struct VirtualMachine {
    stack: [Value; 256],
    stack_top: *mut Value,
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
        let mut stack = [Value::Nil; 256];
        VirtualMachine {
            stack,
            stack_top: stack.as_mut_ptr(),
            debug,
        }
    }

    pub fn init(&mut self) {
        self.stack_top = self.stack.as_mut_ptr();
    }

    pub fn interpret(&mut self, source: &str) -> Result<()> {
        let mut chunk = Chunk::new();
        let parser = Parser::default();
        let mut compiler = Compiler::new(parser, source, &mut chunk, self.debug);

        compiler.compile()?;

        let mut ip = InstructionPointer::new(&chunk.code);

        self.run(&mut ip, &chunk).map_err(|err| self.runtime_error(err.to_string(), &ip, &chunk).into())
    }

    fn run(&mut self, ip: &mut InstructionPointer, chunk: &Chunk) -> Result<()> {
        loop {
            if self.debug {
                self.debug(&ip, chunk);
            }

            let instruction: OpCode = match (&ip.next()).try_into() {
                Ok(instruction) => instruction,
                Err(error) => return Err(RuntimeError { msg: error }.into()),
            };
            match instruction {
                OpCode::OpReturn => {
                    println!("{}", self.pop());
                    return Ok(());
                }
                OpCode::OpConstant => {
                    let constant_index = ip.next();
                    let constant_value = chunk.constants.get(constant_index as usize).unwrap();
                    self.push(*constant_value);
                }
                OpCode::OpNegate => unsafe {
                    let addr = self.stack_top.sub(1);
                    *addr = (-*addr)?;
                },
                OpCode::OpAdd => self.binary_operation(std::ops::Add::add)?,
                OpCode::OpSubtract => self.binary_operation(std::ops::Sub::sub)?,
                OpCode::OpMultiply => self.binary_operation(std::ops::Mul::mul)?,
                OpCode::OpDivide => self.binary_operation(std::ops::Div::div)?,
                OpCode::OpTrue => self.push(Value::Boolean(true)),
                OpCode::OpFalse => self.push(Value::Boolean(false)),
                OpCode::OpNil => self.push(Value::Nil)
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

    fn peek(&mut self, distance: usize) -> Value {
        unsafe {
            *self.stack_top.sub(distance + 1)
        }
    }

    fn binary_operation<Op: FnOnce(Value, Value) -> Result<Value>>(&mut self, op: Op) -> Result<()> {
        let rhs = self.peek(0);
        let lhs = self.peek(1);
        let result = op(lhs, rhs)?;
        self.pop();
        self.pop();
        self.push(result);
        Ok(())
    }

    fn runtime_error(&self, message: String, ip: &InstructionPointer, chunk: &Chunk) -> RuntimeError {
        let offset = ip.address() - chunk.code.as_ptr() as usize - 1;
        let line = chunk.get_line(offset);
        let message = format!("{}\n[{}] {}", message, offset, line);
        RuntimeError { msg: message }
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
