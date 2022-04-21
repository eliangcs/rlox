mod chunk;
mod debug;
mod value;

use chunk::{Chunk, OpCode};
use debug::disassemble_chunk;

fn main() {
    let mut chunk = Chunk::new();
    let constant = chunk.add_constant(1.2);
    chunk.write_chunk(OpCode::Constant as u8, 123);
    chunk.write_chunk(constant as u8, 123);
    chunk.write_chunk(OpCode::Return as u8, 123);
    chunk.write_chunk(200, 123);
    disassemble_chunk(&chunk, "test chunk");
}
