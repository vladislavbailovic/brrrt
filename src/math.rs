#[cfg(test)]
mod r2r {
    use crate::*;
    use risc32i::{
        instr::builder::Builder, instr::format::Format, instr::operation::*, instr::part::Part, *,
    };

    #[test]
    fn add() {
        let i = Instruction::parse(
            Builder::opcode(Operation::Math)
                .pack(Part::Funct3, 0b000)
                .pack(Part::Funct7, 0b0000000)
                .pack(Part::Reg1, Register::X12 as u32)
                .pack(Part::Reg2, Register::X13 as u32)
                .pack(Part::Dest, Register::X16 as u32)
                .build(),
        )
        .expect("should parse");

        let mut cpu: Cpu = Default::default();
        cpu.register.set(Register::X12, 4);
        cpu.register.set(Register::X13, 2);

        assert_eq!(cpu.register.get(Register::PC), 0);
        cpu.execute(i).expect("should execute");

        assert_eq!(cpu.register.get(Register::X12), 4);
        assert_eq!(cpu.register.get(Register::X13), 2);
        assert_eq!(cpu.register.get(Register::X16), 6);
        assert_eq!(cpu.register.get(Register::PC), 1);
    }

    #[test]
    fn sub() {
        let i = Instruction::parse(
            Builder::opcode(Operation::Math)
                .pack(Part::Funct3, 0b000)
                .pack(Part::Funct7, 0b0100000)
                .pack(Part::Reg1, Register::X12 as u32)
                .pack(Part::Reg2, Register::X13 as u32)
                .pack(Part::Dest, Register::X16 as u32)
                .build(),
        )
        .expect("should parse");

        let mut cpu: Cpu = Default::default();
        cpu.register.set(Register::X12, 193);
        cpu.register.set(Register::X13, 32);

        assert_eq!(cpu.register.get(Register::PC), 0);
        cpu.execute(i).expect("should execute");

        assert_eq!(cpu.register.get(Register::X12), 193);
        assert_eq!(cpu.register.get(Register::X13), 32);
        assert_eq!(cpu.register.get(Register::X16), 161);
        assert_eq!(cpu.register.get(Register::PC), 1);
    }
}
