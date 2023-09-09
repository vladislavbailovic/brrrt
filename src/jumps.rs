#[cfg(test)]
mod immediate {
    use crate::*;
    use risc32i::{
        instr::builder::Builder, instr::format::Format, instr::operation::*, instr::part::Part, *,
    };

    #[test]
    fn jal() {
        let i = Instruction::parse(
            Builder::opcode(Operation::JAL)
                .pack(Part::Dest, Register::X1 as u32)
                .pack(Part::Imm1912, 13)
                .pack(Part::B11j, 0)
                .pack(Part::Imm101, 0)
                .pack(Part::B20j, 0)
                .build(),
        )
        .expect("should parse");

        let mut cpu: Cpu = Default::default();

        assert_eq!(cpu.register.get(Register::PC), 0);
        cpu.execute(i).expect("should execute");

        assert_eq!(cpu.register.get(Register::X1), 1);
        assert_eq!(cpu.register.get(Register::PC), 13);
    }
}
