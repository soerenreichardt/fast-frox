use fast_frox::{
    chunk::Chunk, debug::ChunkDebug, op_code::OpCode, virtual_machine::VirtualMachine,
};

pub(crate) static DEBUG: bool = false;

fn main() {
    let mut chunk = Chunk::new();
    let constant = chunk.add_constant(1.2);
    chunk.write_chunk(OpCode::OpConstant as u8, 123);
    chunk.write_chunk(constant, 123);
    chunk.write_chunk(OpCode::OpReturn as u8, 123);

    let mut vm = VirtualMachine::new(DEBUG);
    vm.init();

    chunk.disassemblee_chunk("test chunk");
    vm.interpret(&chunk);

    drop(vm);
}
