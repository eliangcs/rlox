use std::vec::Vec;

#[derive(Copy, Clone)]
#[repr(u8)]
pub enum OpCode {
    Return,
}

pub struct Chunk {
    pub code: Vec<OpCode>,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk { code: Vec::new() }
    }

    pub fn push(&mut self, byte: OpCode) {
        self.code.push(byte);
    }

    pub fn len(&self) -> usize {
        self.code.len()
    }
}
