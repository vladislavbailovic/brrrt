use brrrt_core::{
    rv32i::{instr::instruction::Instruction, instr::operation::Operation},
    Program, VM,
};
use std::fs;

mod branch;
mod jump;
mod math;
mod memory;
mod upper;

fn main() -> Result<(), String> {
    let mut vm: VM = Default::default();
    let program = load_program("data/c/return-add.bin");

    while !program.is_done(&vm) {
        let instr = program.peek(&vm)?;
        eprintln!("{}", disassemble(instr));
        vm.cpu.increment_pc();
    }
    Ok(())
}

fn load_program(path: &str) -> Program {
    let mut prg: Program = Default::default();
    let src = fs::read(path)
        .expect("Unable to read file")
        .into_iter()
        .enumerate();
    for (i, x) in src {
        prg.write(i as u32, x);
    }
    prg
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
