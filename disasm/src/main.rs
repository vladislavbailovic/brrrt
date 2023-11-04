use brrrt_cli::load_execution_set;
use brrrt_core::{Program, VM};
use disasm::disassemble;

fn main() -> Result<(), String> {
    let mut vm: VM = Default::default();
    let mut program: Program = Default::default();
    load_execution_set(&mut program, &mut vm)?;

    while !program.is_done(&vm) {
        let instr = program.peek(&vm)?;
        eprintln!("{}", disassemble(instr));
        vm.cpu.increment_pc();
    }
    Ok(())
}
