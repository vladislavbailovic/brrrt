#[cfg(test)]
mod byte {
    use crate::rv32i::{instr::builder::Builder, instr::part::Part};
    use crate::*;

    #[test]
    fn store() {
        let i = Instruction::parse(
            Builder::opcode(Operation::Store)
                .pack(Part::Imm40, 13)
                .pack(Part::Funct3, 0b000)
                .pack(Part::Reg1, Register::X12 as u32)
                .pack(Part::Reg2, Register::X13 as u32)
                .pack(Part::Imm115, 0b000)
                .build(),
        )
        .expect("should parse");

        let expected = 161;

        let mut vm: VM = Default::default();
        vm.cpu.register.set(Register::X12, 1);
        vm.cpu.register.set(Register::X13, expected);

        assert_eq!(vm.cpu.register.get(Register::PC), 0);
        vm.execute(i).expect("should execute");

        assert_eq!(vm.cpu.register.get(Register::X13), expected);
        assert_eq!(vm.cpu.register.get(Register::X12), 1);
        assert_eq!(vm.cpu.register.get(Register::PC), 4);
        assert_eq!(vm.ram.byte_at(14).expect("memory access") as u32, expected);
    }

    #[test]
    fn store_negative_offset() {
        let i = Instruction::parse(
            Builder::opcode(Operation::Store)
                .pack(Part::Imm40, 0)
                .pack(Part::Funct3, 0b000)
                .pack(Part::Reg1, Register::X12 as u32)
                .pack(Part::Reg2, Register::X13 as u32)
                .pack(Part::Imm115, 127)
                .build(),
        )
        .expect("should parse");

        let expected = 161;

        let mut vm: VM = Default::default();
        vm.cpu.register.set(Register::X12, 161 + 32);
        vm.cpu.register.set(Register::X13, expected);

        assert_eq!(vm.cpu.register.get(Register::PC), 0);
        vm.execute(i).expect("should execute");

        assert_eq!(vm.cpu.register.get(Register::X13), expected);
        assert_eq!(vm.cpu.register.get(Register::X12), 161 + 32);
        assert_eq!(vm.cpu.register.get(Register::PC), 4);
        assert_eq!(vm.ram.byte_at(161).expect("memory access") as u32, expected);
    }
}

#[cfg(test)]
mod half_word {
    use crate::rv32i::{instr::builder::Builder, instr::part::Part};
    use crate::*;

    #[test]
    fn store() {
        let i = Instruction::parse(
            Builder::opcode(Operation::Store)
                .pack(Part::Imm40, 13)
                .pack(Part::Funct3, 0b001)
                .pack(Part::Reg1, Register::X12 as u32)
                .pack(Part::Reg2, Register::X13 as u32)
                .pack(Part::Imm115, 0b000)
                .build(),
        )
        .expect("should parse");

        let expected = 1312;

        let mut vm: VM = Default::default();
        vm.cpu.register.set(Register::X12, 1);
        vm.cpu.register.set(Register::X13, expected);

        assert_eq!(vm.cpu.register.get(Register::PC), 0);
        vm.execute(i).expect("should execute");

        assert_eq!(vm.cpu.register.get(Register::X13), expected);
        assert_eq!(vm.cpu.register.get(Register::X12), 1);
        assert_eq!(vm.cpu.register.get(Register::PC), 4);
        assert_eq!(vm.ram.hw_at(14).expect("memory access") as u32, expected);
    }

    #[test]
    fn store_negative_offset() {
        let i = Instruction::parse(
            Builder::opcode(Operation::Store)
                .pack(Part::Imm40, 0)
                .pack(Part::Funct3, 0b001)
                .pack(Part::Reg1, Register::X12 as u32)
                .pack(Part::Reg2, Register::X13 as u32)
                .pack(Part::Imm115, 127)
                .build(),
        )
        .expect("should parse");

        let expected = 1312;

        let mut vm: VM = Default::default();
        vm.cpu.register.set(Register::X12, 161 + 32);
        vm.cpu.register.set(Register::X13, expected);

        assert_eq!(vm.cpu.register.get(Register::PC), 0);
        vm.execute(i).expect("should execute");

        assert_eq!(vm.cpu.register.get(Register::X13), expected);
        assert_eq!(vm.cpu.register.get(Register::X12), 161 + 32);
        assert_eq!(vm.cpu.register.get(Register::PC), 4);
        assert_eq!(vm.ram.hw_at(161).expect("memory access") as u32, expected);
    }
}

#[cfg(test)]
mod word {
    use crate::rv32i::{instr::builder::Builder, instr::part::Part};
    use crate::*;

    #[test]
    fn store() {
        let i = Instruction::parse(
            Builder::opcode(Operation::Store)
                .pack(Part::Imm40, 13)
                .pack(Part::Funct3, 0b010)
                .pack(Part::Reg1, Register::X12 as u32)
                .pack(Part::Reg2, Register::X13 as u32)
                .pack(Part::Imm115, 0b000)
                .build(),
        )
        .expect("should parse");

        let expected = 1611312;

        let mut vm: VM = Default::default();
        vm.cpu.register.set(Register::X12, 1);
        vm.cpu.register.set(Register::X13, expected);

        assert_eq!(vm.cpu.register.get(Register::PC), 0);
        vm.execute(i).expect("should execute");

        assert_eq!(vm.cpu.register.get(Register::X13), expected);
        assert_eq!(vm.cpu.register.get(Register::X12), 1);
        assert_eq!(vm.cpu.register.get(Register::PC), 4);
        assert_eq!(vm.ram.word_at(14).expect("memory access") as u32, expected);
    }

    #[test]
    fn store_negative_offset() {
        let i = Instruction::parse(
            Builder::opcode(Operation::Store)
                .pack(Part::Imm40, 0)
                .pack(Part::Funct3, 0b010)
                .pack(Part::Reg1, Register::X12 as u32)
                .pack(Part::Reg2, Register::X13 as u32)
                .pack(Part::Imm115, 127)
                .build(),
        )
        .expect("should parse");

        let expected = 1611312;

        let mut vm: VM = Default::default();
        vm.cpu.register.set(Register::X12, 161 + 32);
        vm.cpu.register.set(Register::X13, expected);

        assert_eq!(vm.cpu.register.get(Register::PC), 0);
        vm.execute(i).expect("should execute");

        assert_eq!(vm.cpu.register.get(Register::X13), expected);
        assert_eq!(vm.cpu.register.get(Register::X12), 161 + 32);
        assert_eq!(vm.cpu.register.get(Register::PC), 4);
        assert_eq!(vm.ram.word_at(161).expect("memory access") as u32, expected);
    }
}
