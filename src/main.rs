use fast_frox::{chunk::Chunk, debug::ChunkDebug, op_code::OpCode};

fn main() {
    let mut chunk = Chunk::new();
    let constant = chunk.add_constant(1.2);
    chunk.write_chunk(OpCode::OpConstant(constant), 123);
    chunk.write_chunk(OpCode::OpReturn, 123);

    chunk.disassemblee_chunk("test chunk");
}
