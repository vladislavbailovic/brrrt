use brrrt_cli::load_program;
use brrrt_core::VM;
use disasm::disassemble;

fn main() -> Result<(), String> {
    let mut vm: VM = Default::default();
    let program = load_program();

    while !program.is_done(&vm) {
        let instr = program.peek(&vm)?;
        eprintln!("{}", disassemble(instr));
        vm.cpu.increment_pc();
    }
    Ok(())
}
