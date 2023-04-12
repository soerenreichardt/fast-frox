use fast_frox::{
    chunk::Chunk, debug::ChunkDebug, op_code::OpCode, virtual_machine::VirtualMachine, value::Value,
};

pub(crate) static DEBUG: bool = false;

fn main() {
    let mut chunk = Chunk::new();
    write_constant(1.2, &mut chunk);
    write_constant(3.4, &mut chunk);

    chunk.write_chunk(OpCode::OpAdd as u8, 123);
    write_constant(5.6, &mut chunk);

    chunk.write_chunk(OpCode::OpDivide as u8, 123);
    chunk.write_chunk(OpCode::OpNegate as u8, 123);
    chunk.write_chunk(OpCode::OpReturn as u8, 123);

    let mut vm = VirtualMachine::new(DEBUG);
    vm.init();

    chunk.disassemblee_chunk("test chunk");
    vm.interpret(&chunk);

    drop(vm);
}

fn write_constant(value: Value, chunk: &mut Chunk) {
    let constant = chunk.add_constant(value);
    chunk.write_chunk(OpCode::OpConstant as u8, 123);
    chunk.write_chunk(constant, 123);
}
