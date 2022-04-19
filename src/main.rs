mod chunk;
mod debug;

use chunk::{Chunk, OpCode};
use debug::disassemble_chunk;

fn main() {
    let mut chunk = Chunk::new();
    chunk.push(OpCode::Return);

    disassemble_chunk(&chunk, "test chunk");
}
