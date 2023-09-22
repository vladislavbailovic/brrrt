#[cfg(test)]
mod byte {
    use crate::*;
    use brrrt::risc32i::{instr::builder::Builder, instr::part::Part};

    #[test]
    fn load() {
        let i = Instruction::parse(
            Builder::opcode(Operation::Load)
                .pack(Part::Dest, Register::X12 as u32)
                .pack(Part::Funct3, 0b000)
                .pack(Part::Reg1, Register::X13 as u32)
                .pack(Part::Imm110, 161)
                .build(),
        )
        .expect("should parse");

        let neg = -6;

        let mut cpu: Cpu = Default::default();
        cpu.register.set(Register::X12, 1);
        cpu.register.set(Register::X13, 1);
        cpu.ram
            .set_byte_at(162, neg as u8)
            .expect("memory value set");

        assert_eq!(cpu.register.get(Register::PC), 0);
        cpu.execute(i).expect("should execute");

        assert_eq!(cpu.register.get(Register::X13), 1);
        assert_eq!(cpu.register.get(Register::X12), neg as u32);
        assert_eq!(cpu.register.get(Register::PC), 4);
        assert_eq!(cpu.ram.byte_at(162).expect("memory access"), neg as u8);
    }

    #[test]
    fn load_negative_offset() {
        let neg = -32;
        let i = Instruction::parse(
            Builder::opcode(Operation::Load)
                .pack(Part::Dest, Register::X12 as u32)
                .pack(Part::Funct3, 0b000)
                .pack(Part::Reg1, Register::X13 as u32)
                .pack(Part::Imm110, neg as u32)
                .build(),
        )
        .expect("should parse");

        let negval = -6;

        let mut cpu: Cpu = Default::default();
        cpu.register.set(Register::X12, 1);
        cpu.register.set(Register::X13, 161 + 32);
        cpu.ram
            .set_byte_at(161, negval as u8)
            .expect("memory value set");

        assert_eq!(cpu.register.get(Register::PC), 0);
        cpu.execute(i).expect("should execute");

        assert_eq!(cpu.register.get(Register::X13), 161 + 32);
        assert_eq!(cpu.register.get(Register::X12), negval as u32);
        assert_eq!(cpu.register.get(Register::PC), 4);
        assert_eq!(cpu.ram.byte_at(161).expect("memory access"), negval as u8);
    }
}

#[cfg(test)]
mod half_word {
    use crate::*;
    use brrrt::risc32i::{instr::builder::Builder, instr::part::Part};

    #[test]
    fn load() {
        let i = Instruction::parse(
            Builder::opcode(Operation::Load)
                .pack(Part::Dest, Register::X12 as u32)
                .pack(Part::Funct3, 0b001)
                .pack(Part::Reg1, Register::X13 as u32)
                .pack(Part::Imm110, 161)
                .build(),
        )
        .expect("should parse");

        let negval = -1312;

        let mut cpu: Cpu = Default::default();
        cpu.register.set(Register::X12, 1);
        cpu.register.set(Register::X13, 1);
        cpu.ram
            .set_hw_at(162, negval as u16)
            .expect("memory value set");

        assert_eq!(cpu.register.get(Register::PC), 0);
        cpu.execute(i).expect("should execute");

        assert_eq!(cpu.register.get(Register::X13), 1);
        assert_eq!(cpu.register.get(Register::X12), negval as u32);
        assert_eq!(cpu.register.get(Register::PC), 4);
        assert_eq!(cpu.ram.hw_at(162).expect("memory access"), negval as u16);
    }

    #[test]
    fn load_negative_offset() {
        let neg = -32;
        let i = Instruction::parse(
            Builder::opcode(Operation::Load)
                .pack(Part::Dest, Register::X12 as u32)
                .pack(Part::Funct3, 0b001)
                .pack(Part::Reg1, Register::X13 as u32)
                .pack(Part::Imm110, neg as u32)
                .build(),
        )
        .expect("should parse");

        let negval = -1312;

        let mut cpu: Cpu = Default::default();
        cpu.register.set(Register::X12, 1);
        cpu.register.set(Register::X13, 161 + 32);
        cpu.ram
            .set_hw_at(161, negval as u16)
            .expect("memory value set");

        assert_eq!(cpu.register.get(Register::PC), 0);
        cpu.execute(i).expect("should execute");

        assert_eq!(cpu.register.get(Register::X13), 161 + 32);
        assert_eq!(cpu.register.get(Register::X12), negval as u32);
        assert_eq!(cpu.register.get(Register::PC), 4);
        assert_eq!(cpu.ram.hw_at(161).expect("memory access"), negval as u16);
    }
}

#[cfg(test)]
mod word {
    use crate::*;
    use brrrt::risc32i::{instr::builder::Builder, instr::part::Part};

    #[test]
    fn load() {
        let i = Instruction::parse(
            Builder::opcode(Operation::Load)
                .pack(Part::Dest, Register::X12 as u32)
                .pack(Part::Funct3, 0b010)
                .pack(Part::Reg1, Register::X13 as u32)
                .pack(Part::Imm110, 161)
                .build(),
        )
        .expect("should parse");

        let mut cpu: Cpu = Default::default();
        cpu.register.set(Register::X12, 1);
        cpu.register.set(Register::X13, 1);
        cpu.ram.set_word_at(162, 1611312).expect("memory value set");

        assert_eq!(cpu.register.get(Register::PC), 0);
        cpu.execute(i).expect("should execute");

        assert_eq!(cpu.register.get(Register::X13), 1);
        assert_eq!(cpu.register.get(Register::X12), 1611312);
        assert_eq!(cpu.register.get(Register::PC), 4);
        assert_eq!(cpu.ram.word_at(162).expect("memory access"), 1611312);
    }

    #[test]
    fn load_negative_offset() {
        let neg = -32;
        let i = Instruction::parse(
            Builder::opcode(Operation::Load)
                .pack(Part::Dest, Register::X12 as u32)
                .pack(Part::Funct3, 0b010)
                .pack(Part::Reg1, Register::X13 as u32)
                .pack(Part::Imm110, neg as u32)
                .build(),
        )
        .expect("should parse");

        let mut cpu: Cpu = Default::default();
        cpu.register.set(Register::X12, 1);
        cpu.register.set(Register::X13, 161 + 32);
        cpu.ram.set_word_at(161, 1611312).expect("memory value set");

        assert_eq!(cpu.register.get(Register::PC), 0);
        cpu.execute(i).expect("should execute");

        assert_eq!(cpu.register.get(Register::X13), 161 + 32);
        assert_eq!(cpu.register.get(Register::X12), 1611312);
        assert_eq!(cpu.register.get(Register::PC), 4);
        assert_eq!(cpu.ram.word_at(161).expect("memory access"), 1611312);
    }
}

#[cfg(test)]
mod byte_unsigned {
    use crate::*;
    use brrrt::risc32i::{instr::builder::Builder, instr::part::Part};

    #[test]
    fn load() {
        let i = Instruction::parse(
            Builder::opcode(Operation::Load)
                .pack(Part::Dest, Register::X12 as u32)
                .pack(Part::Funct3, 0b100)
                .pack(Part::Reg1, Register::X13 as u32)
                .pack(Part::Imm110, 161)
                .build(),
        )
        .expect("should parse");

        let mut cpu: Cpu = Default::default();
        cpu.register.set(Register::X12, 1);
        cpu.register.set(Register::X13, 1);
        cpu.ram.set_byte_at(162, 6).expect("memory value set");

        assert_eq!(cpu.register.get(Register::PC), 0);
        cpu.execute(i).expect("should execute");

        assert_eq!(cpu.register.get(Register::X13), 1);
        assert_eq!(cpu.register.get(Register::X12), 6);
        assert_eq!(cpu.register.get(Register::PC), 4);
        assert_eq!(cpu.ram.byte_at(162).expect("memory access"), 6);
    }

    #[test]
    fn load_negative_offset() {
        let neg = -32;
        let i = Instruction::parse(
            Builder::opcode(Operation::Load)
                .pack(Part::Dest, Register::X12 as u32)
                .pack(Part::Funct3, 0b100)
                .pack(Part::Reg1, Register::X13 as u32)
                .pack(Part::Imm110, neg as u32)
                .build(),
        )
        .expect("should parse");

        let mut cpu: Cpu = Default::default();
        cpu.register.set(Register::X12, 1);
        cpu.register.set(Register::X13, 161 + 32);
        cpu.ram.set_byte_at(161, 6).expect("memory value set");

        assert_eq!(cpu.register.get(Register::PC), 0);
        cpu.execute(i).expect("should execute");

        assert_eq!(cpu.register.get(Register::X13), 161 + 32);
        assert_eq!(cpu.register.get(Register::X12), 6);
        assert_eq!(cpu.register.get(Register::PC), 4);
        assert_eq!(cpu.ram.byte_at(161).expect("memory access"), 6);
    }
}

#[cfg(test)]
mod half_word_unsigned {
    use crate::*;
    use brrrt::risc32i::{instr::builder::Builder, instr::part::Part};

    #[test]
    fn load() {
        let i = Instruction::parse(
            Builder::opcode(Operation::Load)
                .pack(Part::Dest, Register::X12 as u32)
                .pack(Part::Funct3, 0b101)
                .pack(Part::Reg1, Register::X13 as u32)
                .pack(Part::Imm110, 161)
                .build(),
        )
        .expect("should parse");

        let mut cpu: Cpu = Default::default();
        cpu.register.set(Register::X12, 1);
        cpu.register.set(Register::X13, 1);
        cpu.ram.set_hw_at(162, 1312).expect("memory value set");

        assert_eq!(cpu.register.get(Register::PC), 0);
        cpu.execute(i).expect("should execute");

        assert_eq!(cpu.register.get(Register::X13), 1);
        assert_eq!(cpu.register.get(Register::X12), 1312);
        assert_eq!(cpu.register.get(Register::PC), 4);
        assert_eq!(cpu.ram.hw_at(162).expect("memory access"), 1312);
    }

    #[test]
    fn load_negative_offset() {
        let neg = -32;
        let i = Instruction::parse(
            Builder::opcode(Operation::Load)
                .pack(Part::Dest, Register::X12 as u32)
                .pack(Part::Funct3, 0b101)
                .pack(Part::Reg1, Register::X13 as u32)
                .pack(Part::Imm110, neg as u32)
                .build(),
        )
        .expect("should parse");

        let mut cpu: Cpu = Default::default();
        cpu.register.set(Register::X12, 1);
        cpu.register.set(Register::X13, 161 + 32);
        cpu.ram.set_hw_at(161, 1312).expect("memory value set");

        assert_eq!(cpu.register.get(Register::PC), 0);
        cpu.execute(i).expect("should execute");

        assert_eq!(cpu.register.get(Register::X13), 161 + 32);
        assert_eq!(cpu.register.get(Register::X12), 1312);
        assert_eq!(cpu.register.get(Register::PC), 4);
        assert_eq!(cpu.ram.hw_at(161).expect("memory access"), 1312);
    }
}
