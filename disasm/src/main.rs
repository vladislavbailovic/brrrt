use brrrt_core::risc32i::{instr::instruction::Instruction, instr::operation::Operation};

mod branch;
mod jump;
mod math;
mod memory;
mod upper;

fn main() {
    // let raw = 0x00d00093; // addi x1, x0, 13
    let raw = 0x00c09a93; // slli x21, x1, 12

    let result = disassemble(raw);
    eprintln!("result: {:?}", result);
}

fn disassemble(raw: u32) -> String {
    let i = Instruction::parse(raw).expect("unable to parse");

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
