mod chunk;
mod common;
mod compiler;
mod debug;
mod scanner;
mod value;
mod vm;

use chunk::Chunk;
use std::env;
use std::fs;
use std::io;
use std::io::{ErrorKind, Write};
use std::process;
use vm::VM;

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

fn run_file(path: &str) {
    match fs::read_to_string(path) {
        Ok(contents) => {
            println!("{}", contents);
        }
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
    let mut chunk = Chunk::new();
    let mut vm = VM::new(&mut chunk);

    let args: Vec<String> = env::args().collect();
    match args.len() {
        0 | 1 => repl(&mut vm),
        2 => run_file(&args[1]),
        _ => {
            eprintln!("Usage: rlox [path]");
            process::exit(64);
        }
    }
}
