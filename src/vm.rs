use crate::chunk::{Chunk, OpCode};
use crate::debug::disassemble_instruction;
use crate::value::Value;
use std::ptr;

pub struct VM<'a> {
    chunk: &'a Chunk,
    ip: *const u8,
}

#[repr(u8)]
pub enum InterpretResult {
    Ok,
    CompileErr,
    RuntimeErr,
}

impl<'a> VM<'a> {
    pub fn new(chunk: &'a Chunk) -> Self {
        VM {
            chunk: chunk,
            ip: ptr::null_mut(),
        }
    }

    pub unsafe fn interpret(&mut self) -> InterpretResult {
        self.ip = &self.chunk[0] as *const u8;
        self.run()
    }

    unsafe fn run(&mut self) -> InterpretResult {
        loop {
            if cfg!(feature = "debug-trace-execution") {
                disassemble_instruction(
                    self.chunk,
                    self.ip.offset_from(&self.chunk[0] as *const u8) as usize,
                );
            }
            let instruction = OpCode::try_from(self.read_byte());
            match instruction {
                Ok(opcode) => match opcode {
                    OpCode::Constant => {
                        let constant = self.read_constant();
                        println!("{}", constant);
                    }
                    OpCode::Return => break InterpretResult::Ok,
                },
                Err(_) => break InterpretResult::RuntimeErr,
            }
        }
    }

    #[inline]
    unsafe fn read_constant(&mut self) -> Value {
        self.chunk.constants[self.read_byte() as usize]
    }

    #[inline]
    unsafe fn read_byte(&mut self) -> u8 {
        let byte = *self.ip;
        self.ip = self.ip.add(1);
        byte
    }
}
