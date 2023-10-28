use brrrt_cli::load_program;
use brrrt_core::VM;

fn main() -> Result<(), String> {
    let mut vm: VM = Default::default();
    let program = load_program();

    vm.cpu.initialize();

    program.run(&mut vm)?;
    Ok(())
}
