#[cfg(test)]
mod immediate {
    use crate::rv32i::{instr::builder::Builder, instr::part::Part};
    use crate::*;

    #[test]
    fn jal() {
        let i = Instruction::parse(
            Builder::opcode(Operation::JAL)
                .pack(Part::Dest, Register::X1 as u32)
                .pack(Part::Imm1912, 0)
                .pack(Part::B11j, 0)
                .pack(Part::Imm101, 13)
                .pack(Part::B20j, 0)
                .build(),
        )
        .expect("should parse");

        let mut vm: VM = Default::default();

        assert_eq!(vm.cpu.register.get(Register::PC), 0);
        vm.execute(i).expect("should execute");

        assert_eq!(vm.cpu.register.get(Register::X1), 4);
        assert_eq!(vm.cpu.register.get(Register::PC), 26);
    }

    #[test]
    fn jal_with_pc() {
        let i = Instruction::parse(
            Builder::opcode(Operation::JAL)
                .pack(Part::Dest, Register::X1 as u32)
                .pack(Part::Imm1912, 0)
                .pack(Part::B11j, 0)
                .pack(Part::Imm101, 13)
                .pack(Part::B20j, 0)
                .build(),
        )
        .expect("should parse");

        let mut vm: VM = Default::default();

        vm.cpu.register.set(Register::PC, 12);
        vm.execute(i).expect("should execute");

        assert_eq!(vm.cpu.register.get(Register::X1), 16);
        assert_eq!(vm.cpu.register.get(Register::PC), 38);
    }

    #[test]
    fn j_pseudo_op() {
        let i = Instruction::parse(
            Builder::opcode(Operation::JAL)
                .pack(Part::Dest, Register::X0 as u32)
                .pack(Part::Imm1912, 0)
                .pack(Part::B11j, 0)
                .pack(Part::Imm101, 13)
                .pack(Part::B20j, 0)
                .build(),
        )
        .expect("should parse");

        let mut vm: VM = Default::default();

        assert_eq!(vm.cpu.register.get(Register::PC), 0);
        vm.execute(i).expect("should execute");

        assert_eq!(vm.cpu.register.get(Register::X0), 0);
        assert_eq!(vm.cpu.register.get(Register::PC), 26);
    }
}

#[cfg(test)]
mod register {
    use crate::rv32i::{instr::builder::Builder, instr::part::Part};
    use crate::*;

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

        let mut vm: VM = Default::default();

        assert_eq!(vm.cpu.register.get(Register::PC), 0);
        vm.execute(i).expect("should execute");

        assert_eq!(vm.cpu.register.get(Register::X1), 4);
        assert_eq!(vm.cpu.register.get(Register::PC), 13);
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

        let mut vm: VM = Default::default();

        vm.cpu.register.set(Register::PC, 100);
        vm.execute(i).expect("should execute");

        assert_eq!(vm.cpu.register.get(Register::X1), 104);
        assert_eq!(vm.cpu.register.get(Register::PC), 13);
    }
}
