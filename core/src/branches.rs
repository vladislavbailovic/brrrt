#[cfg(test)]
use crate::rv32i::{instr::builder::Builder, instr::part::Part};
#[cfg(test)]
use crate::*;

#[cfg(test)]
struct Test {
    funct3: u32,
    left: u32,
    right: u32,
    address: u8,
    expected: u32,
}

#[cfg(test)]
fn mkinstr(test: &Test) -> Instruction {
    let a = test.address as u32;
    let im41 = if test.address > 16 {
        (a << 1) & 0b0000_0000_0000_0000_0000_0000_0001_1110
    } else {
        a
    };
    let im105 = if test.address > 16 { a >> 4 } else { 0 };
    Instruction::parse(
        Builder::opcode(Operation::Branch)
            .pack(Part::Funct3, test.funct3)
            .pack(Part::B11b, 0)
            .pack(Part::Imm41, im41)
            .pack(Part::Reg1, Register::X12 as u32)
            .pack(Part::Reg2, Register::X13 as u32)
            .pack(Part::Imm105, im105)
            .pack(Part::B12b, 0)
            .build(),
    )
    .expect("should parse")
}

#[cfg(test)]
fn apply(test: Test) {
    let i = mkinstr(&test);
    let mut vm: VM = Default::default();

    vm.cpu.register.set(Register::X12, test.left);
    vm.cpu.register.set(Register::X13, test.right);

    assert_eq!(vm.cpu.register.get(Register::PC), 0);
    vm.execute(i).expect("should execute");

    assert_eq!(vm.cpu.register.get(Register::PC), test.expected);
}

#[cfg(test)]
mod beq {
    use super::*;

    #[test]
    fn zero_eq_zero() {
        apply(Test {
            funct3: 0b000,
            left: 0,
            right: 0,
            address: 12,
            expected: 24,
        });
    }

    #[test]
    fn num_eq_num() {
        apply(Test {
            funct3: 0b000,
            left: 161,
            right: 161,
            address: 160,
            expected: 320,
        });
    }

    #[test]
    fn num_neq_num() {
        apply(Test {
            funct3: 0b000,
            left: 13,
            right: 12,
            address: 160,
            expected: 4,
        });
    }
}

#[cfg(test)]
mod bne {
    use super::*;

    #[test]
    fn zero_eq_zero() {
        apply(Test {
            funct3: 0b001,
            left: 0,
            right: 0,
            address: 12,
            expected: 4,
        });
    }

    #[test]
    fn num_eq_num() {
        apply(Test {
            funct3: 0b001,
            left: 161,
            right: 161,
            address: 160,
            expected: 4,
        });
    }

    #[test]
    fn num_neq_num() {
        apply(Test {
            funct3: 0b001,
            left: 13,
            right: 12,
            address: 160,
            expected: 320,
        });
    }
}

#[cfg(test)]
mod blt {
    use super::*;

    #[test]
    fn zero_eq_zero() {
        apply(Test {
            funct3: 0b100,
            left: 0,
            right: 0,
            address: 12,
            expected: 4,
        });
    }

    #[test]
    fn pos_neg() {
        let neg = -12;
        apply(Test {
            funct3: 0b100,
            left: neg as u32,
            right: 12,
            address: 12,
            expected: 24,
        });
    }
}

#[cfg(test)]
mod bltu {
    use super::*;

    #[test]
    fn zero_eq_zero() {
        apply(Test {
            funct3: 0b110,
            left: 0,
            right: 0,
            address: 12,
            expected: 4,
        });
    }

    #[test]
    fn pos_neg() {
        let neg = -12;
        apply(Test {
            funct3: 0b110,
            left: neg as u32,
            right: 12,
            address: 12,
            expected: 4,
        });
    }
}

//---

#[cfg(test)]
mod bge {
    use super::*;

    #[test]
    fn zero_eq_zero() {
        apply(Test {
            funct3: 0b101,
            left: 0,
            right: 0,
            address: 12,
            expected: 4,
        });
    }

    #[test]
    fn pos_neg() {
        let neg = -12;
        apply(Test {
            funct3: 0b101,
            left: neg as u32,
            right: 12,
            address: 12,
            expected: 4,
        });
    }
}

#[cfg(test)]
mod bgeu {
    use super::*;

    #[test]
    fn zero_eq_zero() {
        apply(Test {
            funct3: 0b111,
            left: 0,
            right: 0,
            address: 12,
            expected: 4,
        });
    }

    #[test]
    fn pos_neg() {
        let neg = -12;
        apply(Test {
            funct3: 0b111,
            left: neg as u32,
            right: 12,
            address: 12,
            expected: 24,
        });
    }
}
