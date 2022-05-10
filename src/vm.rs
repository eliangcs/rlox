use crate::chunk::{Chunk, OpCode};
use crate::compiler::Parser;
use crate::debug::disassemble_instruction;
use crate::value::Value;
use std::ops::{Add, Div, Mul, Sub};
use std::ptr;

const STACK_MAX: usize = 256;

pub struct VM {
    chunk: Chunk,
    ip: *const u8,
    stack: [Value; STACK_MAX],
    stack_top: *mut Value,
}

#[repr(u8)]
pub enum InterpretResult {
    Ok,
    CompileErr,
    RuntimeErr,
}

// TODO: Allocate vm as a global static instance
impl VM {
    pub fn new() -> Self {
        let mut vm = VM {
            chunk: Chunk::new(),
            ip: ptr::null_mut(),
            stack: [0.0; STACK_MAX],
            stack_top: ptr::null_mut(),
        };
        vm.reset_stack();
        vm
    }

    #[inline]
    fn reset_stack(&mut self) {
        self.stack_top = &mut self.stack[0] as *mut Value;
    }

    pub unsafe fn interpret(&mut self, source: &str) -> InterpretResult {
        let mut parser = Parser::new(source, &mut self.chunk);
        if !parser.compile() {
            self.chunk.clear();
            return InterpretResult::CompileErr;
        }

        self.ip = &self.chunk[0] as *const u8;

        let result = self.run();

        self.chunk.clear();
        result
    }

    unsafe fn push(&mut self, value: Value) {
        *self.stack_top = value;
        self.stack_top = self.stack_top.add(1);
    }

    unsafe fn pop(&mut self) -> Value {
        self.stack_top = self.stack_top.offset(-1);
        *self.stack_top
    }

    unsafe fn run(&mut self) -> InterpretResult {
        loop {
            if cfg!(feature = "debug-trace-execution") {
                print!("          ");
                let mut slot = &self.stack[0] as *const Value;
                while slot < self.stack_top {
                    print!("[ {} ]", *slot);
                    slot = slot.add(1);
                }
                println!();

                disassemble_instruction(
                    &self.chunk,
                    self.ip.offset_from(&self.chunk[0] as *const u8) as usize,
                );
            }
            let instruction = OpCode::try_from(self.read_byte());
            match instruction {
                Ok(opcode) => match opcode {
                    OpCode::Constant => {
                        let constant = self.read_constant();
                        self.push(constant);
                    }
                    OpCode::Add => self.binary_op(&Add::add),
                    OpCode::Subtract => self.binary_op(&Sub::sub),
                    OpCode::Multiply => self.binary_op(&Mul::mul),
                    OpCode::Divide => self.binary_op(&Div::div),
                    OpCode::Negate => {
                        let value = self.pop();
                        self.push(-value);
                    }
                    OpCode::Return => {
                        println!("{}", self.pop());
                        break InterpretResult::Ok;
                    }
                },
                Err(_) => break InterpretResult::RuntimeErr,
            }
        }
    }

    #[inline]
    unsafe fn binary_op(&mut self, op: &dyn Fn(Value, Value) -> Value) {
        let b = self.pop();
        let a = self.pop();
        self.push(op(a, b));
    }

    #[inline]
    unsafe fn read_constant(&mut self) -> Value {
        let constant = self.read_byte() as usize;
        self.chunk.constants[constant]
    }

    #[inline]
    unsafe fn read_byte(&mut self) -> u8 {
        let byte = *self.ip;
        self.ip = self.ip.add(1);
        byte
    }
}
