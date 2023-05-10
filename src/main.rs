use std::{
    env, fs,
    io::{self, BufRead, Write},
};
use fast_frox::error::ArgumentError;
use fast_frox::virtual_machine::VirtualMachine;
use miette::Result;

pub(crate) static DEBUG: bool = true;

fn main() -> Result<()> {
    let args = env::args().collect::<Vec<_>>();

    let mut vm = VirtualMachine::new(DEBUG);
    vm.init();

    let result = match args.as_slice() {
        [_] => repl(&mut vm),
        [_, path] => run_file(path, &mut vm),
        _ => Err(ArgumentError { msg: "Usage: fast-frox [path]".to_owned() }.into() ),
    };

    drop(vm);
    result
}

fn repl(vm: &mut VirtualMachine) -> Result<()> {
    let stdin = io::stdin();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut buffer = String::new();
        stdin.lock().read_line(&mut buffer).unwrap();
        buffer = buffer.trim().to_string();
        if buffer.is_empty() {
            return Ok(());
        }
        vm.interpret(buffer.as_str())?;
    }
}

fn run_file(path: &str, vm: &mut VirtualMachine) -> Result<()> {
    let source = fs::read_to_string(path).expect("Should have been able to read the file");
    vm.interpret(source.as_str())
}
