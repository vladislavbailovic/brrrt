use brrrt_core::rv32i::{instr::instruction::Instruction, instr::operation::Operation};
mod branch;
mod jump;
mod math;
mod memory;
mod upper;

pub fn disassemble(i: Instruction) -> String {
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
