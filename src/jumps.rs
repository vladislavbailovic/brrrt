#[cfg(test)]
mod immediate {
    use crate::*;
    use risc32i::{instr::builder::Builder, instr::part::Part};

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

    #[test]
    fn jal_with_pc() {
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

        cpu.register.set(Register::PC, 12);
        cpu.execute(i).expect("should execute");

        assert_eq!(cpu.register.get(Register::X1), 13);
        assert_eq!(cpu.register.get(Register::PC), 25);
    }

    #[test]
    fn j_pseudo_op() {
        let i = Instruction::parse(
            Builder::opcode(Operation::JAL)
                .pack(Part::Dest, Register::X0 as u32)
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

        assert_eq!(cpu.register.get(Register::X0), 0);
        assert_eq!(cpu.register.get(Register::PC), 13);
    }
}

#[cfg(test)]
mod register {
    use crate::*;
    use risc32i::{instr::builder::Builder, instr::part::Part};

    #[test]
    fn jalr() {
        let i = Instruction::parse(
            Builder::opcode(Operation::JALR)
                .pack(Part::Dest, Register::X1 as u32)
                .pack(Part::Funct3, 0b000)
                .pack(Part::Reg1, Register::X16 as u32)
                .pack(Part::Imm110, 13)
                .build(),
        )
        .expect("should parse");

        let mut cpu: Cpu = Default::default();

        assert_eq!(cpu.register.get(Register::PC), 0);
        cpu.execute(i).expect("should execute");

        assert_eq!(cpu.register.get(Register::X1), 1);
        assert_eq!(cpu.register.get(Register::PC), 13);
    }

    #[test]
    fn jalr_ignores_pc() {
        let i = Instruction::parse(
            Builder::opcode(Operation::JALR)
                .pack(Part::Dest, Register::X1 as u32)
                .pack(Part::Funct3, 0b000)
                .pack(Part::Reg1, Register::X16 as u32)
                .pack(Part::Imm110, 13)
                .build(),
        )
        .expect("should parse");

        let mut cpu: Cpu = Default::default();

        cpu.register.set(Register::PC, 100);
        cpu.execute(i).expect("should execute");

        assert_eq!(cpu.register.get(Register::X1), 101);
        assert_eq!(cpu.register.get(Register::PC), 13);
    }
}
