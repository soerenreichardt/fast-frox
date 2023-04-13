use std::{
    env, fs,
    io::{self, BufRead, Write},
};

use fast_frox::virtual_machine::{InterpretResult, VirtualMachine};

pub(crate) static DEBUG: bool = false;

fn main() {
    let args = env::args().collect::<Vec<_>>();

    let mut vm = VirtualMachine::new(DEBUG);
    vm.init();

    match args.as_slice() {
        [_] => repl(&mut vm),
        [_, path] => run_file(path, &mut vm),
        _ => panic!("Usage: fast-frox [path]"),
    }

    drop(vm);
}

fn repl(vm: &mut VirtualMachine) {
    let stdin = io::stdin();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut buffer = String::new();
        stdin.lock().read_line(&mut buffer).unwrap();
        buffer = buffer.trim().to_string();
        if buffer.is_empty() {
            return;
        }
        vm.interpret(buffer.as_str());
    }
}

fn run_file(path: &str, vm: &mut VirtualMachine) {
    let source = fs::read_to_string(path).expect("Should have been able to read the file");
    match vm.interpret(source.as_str()) {
        InterpretResult::CompileError => std::process::exit(65),
        InterpretResult::RuntimeError => std::process::exit(70),
        _ => (),
    }
}
