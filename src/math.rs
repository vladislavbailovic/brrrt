#[cfg(test)]
mod r2r {
    use crate::*;
    use risc32i::{instr::builder::Builder, instr::part::Part};

    struct Test {
        funct3: u32,
        funct7: u32,
        rs1: u32,
        rs2: u32,
        expected: u32,
    }

    fn apply(t: Test) {
        let i = Instruction::parse(
            Builder::opcode(Operation::Math)
                .pack(Part::Funct3, t.funct3)
                .pack(Part::Funct7, t.funct7)
                .pack(Part::Reg1, Register::X12 as u32)
                .pack(Part::Reg2, Register::X13 as u32)
                .pack(Part::Dest, Register::X16 as u32)
                .build(),
        )
        .expect("should parse");

        let mut cpu: Cpu = Default::default();
        cpu.register.set(Register::X12, t.rs1);
        cpu.register.set(Register::X13, t.rs2);

        assert_eq!(cpu.register.get(Register::PC), 0);
        cpu.execute(i).expect("should execute");

        assert_eq!(cpu.register.get(Register::X12), t.rs1);
        assert_eq!(cpu.register.get(Register::X13), t.rs2);
        assert_eq!(cpu.register.get(Register::X16), t.expected);
        assert_eq!(cpu.register.get(Register::PC), 1);
    }

    #[test]
    fn add() {
        apply(Test {
            funct3: 0b000,
            funct7: 0b0000000,
            rs1: 4,
            rs2: 2,
            expected: 6,
        });
    }

    #[test]
    fn sub() {
        apply(Test {
            funct3: 0b000,
            funct7: 0b0100000,
            rs1: 193,
            rs2: 32,
            expected: 161,
        });
    }

    #[test]
    fn slt_less_than() {
        apply(Test {
            funct3: 0b010,
            funct7: 0b0000000,
            rs1: 2,
            rs2: 4,
            expected: 1,
        });
    }

    #[test]
    fn slt_not_less_than_with_unsigned_no_overflow() {
        let neg = -2;
        apply(Test {
            funct3: 0b010,
            funct7: 0b0000000,
            rs1: neg as u32,
            rs2: 2,
            expected: 1, // -2 < 2, no overflow
        });
    }

    #[test]
    fn slt_not_less_than() {
        apply(Test {
            funct3: 0b010,
            funct7: 0b0000000,
            rs1: 4,
            rs2: 2,
            expected: 0,
        });
    }

    #[test]
    fn sltu_simple_case_less_than() {
        apply(Test {
            funct3: 0b010,
            funct7: 0b0000000,
            rs1: 2,
            rs2: 4,
            expected: 1,
        });
    }

    #[test]
    fn sltu_simple_case_less_than_with_unsigned_overflow() {
        let neg = -2;
        apply(Test {
            funct3: 0b010,
            funct7: 0b0000000,
            rs1: 2,
            rs2: neg as u32,
            expected: 0, // expect negative overflow
        });
    }

    #[test]
    fn sltu_simple_case_not_less_than() {
        apply(Test {
            funct3: 0b010,
            funct7: 0b0000000,
            rs1: 4,
            rs2: 2,
            expected: 0,
        });
    }

    #[test]
    fn sll_shifts_left() {
        apply(Test {
            funct3: 0b001,
            funct7: 0b0000000,
            rs1: 4,
            rs2: 2,
            expected: 16,
        });
    }

    #[test]
    fn sll_shifts_left_only_top_5_bits() {
        apply(Test {
            funct3: 0b001,
            funct7: 0b0000000,
            rs1: 4,
            rs2: 34,
            expected: 16,
        });
    }

    #[test]
    fn srl_shifts_left() {
        apply(Test {
            funct3: 0b101,
            funct7: 0b0000000,
            rs1: 4,
            rs2: 2,
            expected: 1,
        });
    }

    #[test]
    fn srl_shifts_left_only_top_5_bits() {
        apply(Test {
            funct3: 0b101,
            funct7: 0b0000000,
            rs1: 4,
            rs2: 34,
            expected: 1,
        });
    }

    #[test]
    fn sra_simple_case_shifts_left() {
        apply(Test {
            funct3: 0b101,
            funct7: 0b0100000,
            rs1: 4,
            rs2: 2,
            expected: 1,
        });
    }

    #[test]
    fn sra_simple_case_shifts_left_only_top_5_bits() {
        apply(Test {
            funct3: 0b101,
            funct7: 0b0100000,
            rs1: 4,
            rs2: 34,
            expected: 1,
        });
    }

    #[test]
    fn xor() {
        apply(Test {
            funct3: 0b100,
            funct7: 0b0000000,
            rs1: 4,
            rs2: 2,
            expected: 6,
        });
    }

    #[test]
    fn or() {
        apply(Test {
            funct3: 0b110,
            funct7: 0b0000000,
            rs1: 5,
            rs2: 3,
            expected: 7,
        });
    }

    #[test]
    fn and() {
        apply(Test {
            funct3: 0b111,
            funct7: 0b0000000,
            rs1: 5,
            rs2: 3,
            expected: 1,
        });
    }
}
