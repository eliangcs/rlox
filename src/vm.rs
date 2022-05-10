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
            stack: [Value::Nil; STACK_MAX],
            stack_top: ptr::null_mut(),
        };
        vm.reset_stack();
        vm
    }

    #[inline]
    fn reset_stack(&mut self) {
        self.stack_top = &mut self.stack[0] as *mut Value;
    }

    unsafe fn runtime_error(&mut self, message: &str) {
        eprintln!("{}", message);

        let instruction = self.ip.offset_from(self.chunk[0] as *const u8) - 1;
        let line = self.chunk.lines[instruction as usize];
        eprintln!("[line {}] in script", line);

        self.reset_stack();
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

    unsafe fn peek(&self, distance: usize) -> Value {
        *self.stack_top.offset(-1 - (distance as isize))
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
                    OpCode::Add => match self.binary_op(&Add::add) {
                        Err(err) => break err,
                        _ => (),
                    },
                    OpCode::Subtract => match self.binary_op(&Sub::sub) {
                        Err(err) => break err,
                        _ => (),
                    },
                    OpCode::Multiply => match self.binary_op(&Mul::mul) {
                        Err(err) => break err,
                        _ => (),
                    },
                    OpCode::Divide => match self.binary_op(&Div::div) {
                        Err(err) => break err,
                        _ => (),
                    },
                    OpCode::Negate => match self.peek(0) {
                        Value::Number(value) => {
                            self.pop();
                            self.push(Value::number(-value.number));
                        }
                        _ => {
                            self.runtime_error("Operand must be a number.");
                            break InterpretResult::RuntimeErr;
                        }
                    },
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
    unsafe fn binary_op<F>(&mut self, op: F) -> Result<(), InterpretResult>
    where
        F: Fn(f64, f64) -> f64,
    {
        let b = self.peek(0);
        let a = self.peek(1);

        if let (Value::Number(a), Value::Number(b)) = (a, b) {
            self.pop();
            self.pop();
            self.push(Value::number(op(a.number, b.number)));
            Ok(())
        } else {
            self.runtime_error("Operands must be numbers.");
            Err(InterpretResult::RuntimeErr)
        }
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
