#[cfg(test)]
mod normal {
    use crate::*;
    use brrrt::risc32i::{instr::builder::Builder, instr::part::Part};

    struct Test {
        funct3: u32,
        immediate: u32,
        rs1: u32,
        expected: u32,
    }

    fn apply(t: Test) {
        let i = Instruction::parse(
            Builder::opcode(Operation::ImmediateMath)
                .pack(Part::Funct3, t.funct3)
                .pack(Part::Imm110, t.immediate)
                .pack(Part::Reg1, Register::X21 as u32)
                .pack(Part::Dest, Register::X1 as u32)
                .build(),
        )
        .expect("should parse");

        let mut cpu: Cpu = Default::default();
        cpu.register.set(Register::X21, t.rs1);

        assert_eq!(cpu.register.get(Register::PC), 0);
        cpu.execute(i).expect("should execute");

        assert_eq!(cpu.register.get(Register::X21), t.rs1);
        assert_eq!(cpu.register.get(Register::X1), t.expected);
        assert_eq!(cpu.register.get(Register::PC), 4);
    }

    #[test]
    fn addi() {
        apply(Test {
            funct3: 0b000,
            rs1: 1,
            immediate: 1,
            expected: 2,
        });
    }

    #[test]
    fn addi_negative() {
        let neg = -12;
        apply(Test {
            funct3: 0b000,
            rs1: 13,
            immediate: neg as u32,
            expected: 1,
        });
    }

    #[test]
    fn slti_reg_lt_immediate_unsigned() {
        apply(Test {
            funct3: 0b010,
            rs1: 1,
            immediate: 2,
            expected: 1,
        });
    }

    #[test]
    fn slti_reg_not_lt_immediate_unsigned() {
        apply(Test {
            funct3: 0b010,
            rs1: 2,
            immediate: 1,
            expected: 0,
        });
    }

    #[test]
    fn slti_reg_lt_immediate_signed() {
        let neg = -3;
        apply(Test {
            funct3: 0b010,
            rs1: neg as u32,
            immediate: 2,
            expected: 1,
        });
    }

    #[test]
    fn sltiu_reg_lt_immediate_unsigned() {
        apply(Test {
            funct3: 0b011,
            rs1: 1,
            immediate: 2,
            expected: 1,
        });
    }

    #[test]
    fn sltiu_reg_not_lt_immediate_unsigned() {
        apply(Test {
            funct3: 0b011,
            rs1: 2,
            immediate: 1,
            expected: 0,
        });
    }

    #[test]
    fn sltiu_reg_not_lt_immediate_note_1() {
        apply(Test {
            funct3: 0b011,
            rs1: 0,
            immediate: 1,
            expected: 1,
        });
        apply(Test {
            funct3: 0b011,
            rs1: 1,
            immediate: 1,
            expected: 0,
        });
    }

    #[test]
    fn sltiu_reg_lt_immediate_signed() {
        let neg = -3;
        apply(Test {
            funct3: 0b011,
            rs1: neg as u32,
            immediate: 2,
            expected: 0,
        });
    }

    #[test]
    fn xori() {
        apply(Test {
            funct3: 0b100,
            rs1: 4,
            immediate: 2,
            expected: 6,
        });
    }

    #[test]
    fn xori_note() {
        let neg = -1;
        let neg4 = -4;
        apply(Test {
            funct3: 0b100,
            rs1: 3,
            immediate: neg as u32,
            expected: neg4 as u32,
        });
    }

    #[test]
    fn ori() {
        apply(Test {
            funct3: 0b110,
            rs1: 5,
            immediate: 3,
            expected: 7,
        });
    }

    #[test]
    fn andi() {
        apply(Test {
            funct3: 0b111,
            rs1: 5,
            immediate: 3,
            expected: 1,
        });
    }
}

#[cfg(test)]
mod shift {
    use crate::*;
    use brrrt::risc32i::{instr::builder::Builder, instr::part::Part};

    struct Test {
        funct3: u32,
        immediate: u32,
        shift: u32,
        rs1: u32,
        expected: u32,
    }

    fn apply(t: Test) {
        let immediate = t.shift | t.immediate;
        let i = Instruction::parse(
            Builder::opcode(Operation::ImmediateMath)
                .pack(Part::Funct3, t.funct3)
                .pack(Part::Imm110, immediate)
                .pack(Part::Reg1, Register::X21 as u32)
                .pack(Part::Dest, Register::X1 as u32)
                .build(),
        )
        .expect("should parse");

        let mut cpu: Cpu = Default::default();
        cpu.register.set(Register::X21, t.rs1);

        assert_eq!(cpu.register.get(Register::PC), 0);
        cpu.execute(i).expect("should execute");

        assert_eq!(cpu.register.get(Register::X21), t.rs1);
        assert_eq!(cpu.register.get(Register::X1), t.expected);
        assert_eq!(cpu.register.get(Register::PC), 4);
    }

    #[test]
    fn slli() {
        apply(Test {
            funct3: 0b001,
            shift: 0b0000000,
            immediate: 2,
            rs1: 4,
            expected: 16,
        });
    }

    #[test]
    fn srli() {
        apply(Test {
            funct3: 0b101,
            shift: 0b0000000,
            immediate: 2,
            rs1: 4,
            expected: 1,
        });
    }

    #[test]
    fn srai_simple_case() {
        apply(Test {
            funct3: 0b101,
            shift: 0b0100000,
            immediate: 2,
            rs1: 4,
            expected: 1,
        });
    }
}
