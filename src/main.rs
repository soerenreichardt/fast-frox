use fast_frox::{chunk::Chunk, debug::ChunkDebug, op_code::OpCode, virtual_machine::VirtualMachine};

fn main() {
    let mut chunk = Chunk::new();
    let constant = chunk.add_constant(1.2);
    chunk.write_chunk(OpCode::OpConstant(constant), 123);
    chunk.write_chunk(OpCode::OpReturn, 123);

    let vm = VirtualMachine::new();
    vm.init();

    chunk.disassemblee_chunk("test chunk");
    vm.interpret(&chunk);

    drop(vm);
}
