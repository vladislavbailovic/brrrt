use brrrt_core::{
    rv32i::{instr::instruction::Instruction, instr::part::Part},
    Register,
};

pub fn disassemble(i: Instruction) -> String {
    let rs1: Register = i
        .value(Part::Reg1)
        .expect("invalid reg1")
        .try_into()
        .expect("invalid register");
    let rs1: String = rs1.try_into().unwrap();
    let rs2: Register = i
        .value(Part::Reg2)
        .expect("invalid reg2")
        .try_into()
        .expect("invalid register");
    let rs2: String = rs2.try_into().unwrap();
    let f3 = i.value(Part::Funct3).expect("invalid funct3");
    #[allow(clippy::identity_op)] // readability
    let immediate = 0
        | (i.value(Part::B12b).expect("invalid B12b") << 11)
        | (i.value(Part::B11b).expect("invalid B11b") << 10)
        | (i.value(Part::Imm105).expect("invalid Imm105") << 4)
        | (i.value(Part::Imm41).expect("invalid Imm41") << 0);
    let op = match f3 {
        0b000 => "beq".to_owned(),
        0b001 => "bne".to_owned(),
        0b100 => "blt".to_owned(),
        0b110 => "bltu".to_owned(),
        0b101 => "bge".to_owned(),
        0b111 => "bgeu".to_owned(),
        _ => unreachable!("invalid branch instruction"),
    };
    format!("{} {}, {}, {}", op, rs1, rs2, immediate)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::jump;

    #[test]
    fn bne() {
        // Compiles to 2 instructions
        //   - If EQUAL
        //   - Unconditional jump to 0

        let raw = 0x00d60463; // bne x12, x13, 0
        let i = Instruction::parse(raw).expect("unable to parse");
        let expected = "beq x12, x13, 4".to_owned();
        assert_eq!(disassemble(i), expected);

        let raw = 0xffdff06f;
        let i = Instruction::parse(raw).expect("unable to parse");
        let expected = "jal x0, -4".to_owned(); // -4 because it'll be added to PC=4
        assert_eq!(jump::unconditional(i), expected);
    }

    #[test]
    fn bne_12() {
        // Compiles to 2 instructions
        //   - If EQUAL
        //   - Unconditional jump to 12

        let raw = 0x00d60463; // bne x12, x13, 0
        let i = Instruction::parse(raw).expect("unable to parse");
        let expected = "beq x12, x13, 4".to_owned();
        assert_eq!(disassemble(i), expected);

        let raw = 0x0080006f;
        let i = Instruction::parse(raw).expect("unable to parse");
        let expected = "jal x0, 8".to_owned(); // 8 because it'll be added to PC=4
        assert_eq!(jump::unconditional(i), expected);
    }

    #[test]
    fn bne_32() {
        // Compiles to 2 instructions
        //   - If EQUAL
        //   - Unconditional jump to 32

        let raw = 0x00d60463; // bne x12, x13, 0
        let i = Instruction::parse(raw).expect("unable to parse");
        let expected = "beq x12, x13, 4".to_owned();
        assert_eq!(disassemble(i), expected);

        let raw = 0x01c0006f;
        let i = Instruction::parse(raw).expect("unable to parse");
        let expected = "jal x0, 28".to_owned(); // 8 because it'll be added to PC=4
        assert_eq!(jump::unconditional(i), expected);
    }

    #[test]
    fn beq() {
        // Compiles to 2 instructions
        //   - If NOT EQUAL
        //   - Unconditional jump to 0

        let raw = 0x00d61463; // beq x12, x13, 0
        let i = Instruction::parse(raw).expect("unable to parse");
        let expected = "bne x12, x13, 4".to_owned();
        assert_eq!(disassemble(i), expected);

        let raw = 0xffdff06f;
        let i = Instruction::parse(raw).expect("unable to parse");
        let expected = "jal x0, -4".to_owned(); // 8 because it'll be added to PC=4
        assert_eq!(jump::unconditional(i), expected);
    }

    #[test]
    fn beq_32() {
        // Compiles to 2 instructions
        //   - If NOT EQUAL
        //   - Unconditional jump to 32

        let raw = 0x00d61463; // beq x12, x13, 0
        let i = Instruction::parse(raw).expect("unable to parse");
        let expected = "bne x12, x13, 4".to_owned();
        assert_eq!(disassemble(i), expected);

        let raw = 0x01c0006f;
        let i = Instruction::parse(raw).expect("unable to parse");
        let expected = "jal x0, 28".to_owned(); // 8 because it'll be added to PC=4
        assert_eq!(jump::unconditional(i), expected);
    }

    #[test]
    fn blt() {
        // Compiles to 2 instructions
        //   - If GREATER/EQUAL
        //   - Unconditional jump to 32

        let raw = 0x00d65463; // blt x12, x13, 0
        let i = Instruction::parse(raw).expect("unable to parse");
        let expected = "bge x12, x13, 4".to_owned();
        assert_eq!(disassemble(i), expected);

        let raw = 0xffdff06f;
        let i = Instruction::parse(raw).expect("unable to parse");
        let expected = "jal x0, -4".to_owned(); // 8 because it'll be added to PC=4
        assert_eq!(jump::unconditional(i), expected);
    }
}
