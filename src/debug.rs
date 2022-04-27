use crate::chunk::{Chunk, OpCode};

pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("=== {} ===", name);

    let len = chunk.len();
    let mut offset = 0;
    while offset < len {
        offset = disassemble_instruction(chunk, offset);
    }
}

pub fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
    print!("{:04} ", offset);

    if offset > 0 && chunk.lines[offset] == chunk.lines[offset - 1] {
        print!("   | ");
    } else {
        print!("{:>4} ", chunk.lines[offset]);
    }

    let byte = chunk[offset];
    let instruction = OpCode::try_from(byte);
    match instruction {
        Ok(opcode) => match opcode {
            OpCode::Constant => constant_instruction("OP_CONSTANT", chunk, offset),
            OpCode::Add => simple_instruction("OP_ADD", offset),
            OpCode::Subtract => simple_instruction("OP_SUBTRACT", offset),
            OpCode::Multiply => simple_instruction("OP_MULTIPLY", offset),
            OpCode::Divide => simple_instruction("OP_DIVIDE", offset),
            OpCode::Negate => simple_instruction("OP_NEGATE", offset),
            OpCode::Return => simple_instruction("OP_RETURN", offset),
        },
        Err(_) => {
            println!("Unknown opcode {}\n", byte);
            offset + 1
        }
    }
}

fn constant_instruction(name: &str, chunk: &Chunk, offset: usize) -> usize {
    let constant = chunk[offset + 1] as usize;
    print!("{:<16} {:4} '", name, constant);
    print!("{}", chunk.constants[constant]);
    println!("'");
    offset + 2
}

fn simple_instruction(name: &str, offset: usize) -> usize {
    println!("{}", name);
    offset + 1
}
