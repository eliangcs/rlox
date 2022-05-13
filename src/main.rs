mod chunk;
mod common;
mod compiler;
mod debug;
mod object;
mod scanner;
mod value;
mod vm;

use std::env;
use std::fs;
use std::io;
use std::io::{ErrorKind, Write};
use std::process;
use vm::{InterpretResult, VM};

fn repl(vm: &mut VM) {
    loop {
        let mut buffer = String::new();
        print!("> ");
        io::stdout().flush().unwrap();
        match io::stdin().read_line(&mut buffer) {
            Ok(0) => {
                println!();
                break;
            }
            Ok(_) => unsafe {
                vm.interpret(&buffer);
            },
            Err(error) => eprintln!("error: {}", error),
        }
    }
}

fn run_file(vm: &mut VM, path: &str) {
    match fs::read_to_string(path) {
        Ok(contents) => unsafe {
            match vm.interpret(&contents) {
                InterpretResult::CompileErr => process::exit(65),
                InterpretResult::RuntimeErr => process::exit(70),
                _ => (),
            }
        },
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                eprintln!("File not found \"{}\"", path);
                process::exit(74);
            }
            ErrorKind::PermissionDenied => {
                eprintln!("Permission denied reading file \"{}\"", path);
                process::exit(74);
            }
            _ => {
                eprintln!("{}", error);
                process::exit(74);
            }
        },
    }
}

fn main() {
    let mut vm = VM::new();

    let args: Vec<String> = env::args().collect();
    match args.len() {
        0 | 1 => repl(&mut vm),
        2 => run_file(&mut vm, &args[1]),
        _ => {
            eprintln!("Usage: rlox [path]");
            process::exit(64);
        }
    }
}
