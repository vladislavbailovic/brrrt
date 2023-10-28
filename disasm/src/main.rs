use brrrt_cli::load_program;
use brrrt_core::{
    rv32i::{instr::instruction::Instruction, instr::operation::Operation},
    VM,
};

mod branch;
mod jump;
mod math;
mod memory;
mod upper;

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

fn disassemble(i: Instruction) -> String {
    match i.opcode {
        Operation::LUI => upper::load(i),
        Operation::AUIPC => upper::add(i),
        Operation::Math => math::register(i),
        Operation::ImmediateMath => math::immediate(i),
        Operation::JAL => jump::unconditional(i),
        Operation::JALR => jump::register(i),
        Operation::Branch => branch::disassemble(i),
        Operation::Load => memory::load(i),
        Operation::Store => memory::store(i),
        _ => unreachable!("invalid instruction opcode"),
    }
}
