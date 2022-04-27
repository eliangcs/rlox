mod chunk;
mod common;
mod debug;
mod value;
mod vm;

use chunk::{Chunk, OpCode};
use debug::disassemble_chunk;
use vm::VM;

fn main() {
    let mut chunk = Chunk::new();

    let constant = chunk.add_constant(1.2);
    chunk.write_chunk(OpCode::Constant as u8, 123);
    chunk.write_chunk(constant as u8, 123);

    let constant = chunk.add_constant(3.4);
    chunk.write_chunk(OpCode::Constant as u8, 123);
    chunk.write_chunk(constant as u8, 123);

    chunk.write_chunk(OpCode::Add as u8, 123);

    let constant = chunk.add_constant(5.6);
    chunk.write_chunk(OpCode::Constant as u8, 123);
    chunk.write_chunk(constant as u8, 123);

    chunk.write_chunk(OpCode::Divide as u8, 123);

    chunk.write_chunk(OpCode::Negate as u8, 123);
    chunk.write_chunk(OpCode::Return as u8, 123);
    chunk.write_chunk(200, 123);
    disassemble_chunk(&chunk, "test chunk");

    let mut vm = VM::new(&chunk);
    unsafe {
        vm.interpret();
    }
}
