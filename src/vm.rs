use crate::chunk::{Chunk, OpCode};
use crate::compiler::Parser;
use crate::debug::disassemble_instruction;
use crate::value::Value;
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

macro_rules! binary_op {
    ($parser: ident, $value_type: path, $operator: tt) => {
        {
            let b = $parser.peek(0);
            let a = $parser.peek(1);

            if let (Value::Number(a), Value::Number(b)) = (a, b) {
                $parser.pop();
                $parser.pop();
                $parser.push($value_type(a $operator b));
            } else {
                $parser.runtime_error("Operands must be numbers.");
                break InterpretResult::RuntimeErr
            }
        }
    };
}

#[inline]
fn is_falsey(value: Value) -> bool {
    match value {
        Value::Nil => true,
        Value::Bool(value) => !value,
        _ => false,
    }
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
                    OpCode::Nil => self.push(Value::Nil),
                    OpCode::True => self.push(Value::Bool(true)),
                    OpCode::False => self.push(Value::Bool(false)),
                    OpCode::Equal => {
                        let b = self.pop();
                        let a = self.pop();
                        self.push(Value::Bool(a == b));
                    }
                    OpCode::Greater => binary_op!(self, Value::Bool, >),
                    OpCode::Less => binary_op!(self, Value::Bool, <),
                    OpCode::Add => binary_op!(self, Value::Number, +),
                    OpCode::Subtract => binary_op!(self, Value::Number, -),
                    OpCode::Multiply => binary_op!(self, Value::Number, *),
                    OpCode::Divide => binary_op!(self, Value::Number, /),
                    OpCode::Not => {
                        let value = self.pop();
                        self.push(Value::Bool(is_falsey(value)));
                    }
                    OpCode::Negate => match self.peek(0) {
                        Value::Number(value) => {
                            self.pop();
                            self.push(Value::Number(-value));
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
