use brrrt_cli::load_execution_set;
use brrrt_core::{Program, VM};

fn main() -> Result<(), String> {
    let mut vm: VM = Default::default();
    let mut program: Program = Default::default();

    vm.cpu.initialize();
    load_execution_set(&mut program, &mut vm);

    while !program.is_done(&vm) {
        program.run(&mut vm)?;
    }

    eprintln!("{:?}", vm);

    Ok(())
}
