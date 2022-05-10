use crate::value::Value;
use num_enum::TryFromPrimitive;
use std::ops::Index;
use std::vec::Vec;

#[derive(Copy, Clone, TryFromPrimitive)]
#[repr(u8)]
pub enum OpCode {
    Constant,
    Add,
    Subtract,
    Multiply,
    Divide,
    Negate,
    Return,
}

pub struct Chunk {
    code: Vec<u8>,
    pub lines: Vec<usize>,
    pub constants: Vec<Value>,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            code: Vec::new(),
            lines: Vec::new(),
            constants: Vec::new(),
        }
    }

    pub fn write_chunk(&mut self, byte: u8, line: usize) {
        self.code.push(byte);
        self.lines.push(line);
    }

    pub fn len(&self) -> usize {
        self.code.len()
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }

    pub fn clear(&mut self) {
        self.code.clear();
        self.lines.clear();
        self.constants.clear();
    }
}

impl Index<usize> for Chunk {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.code[index]
    }
}
