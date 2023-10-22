use brrrt_core::{
    rv32i::{instr::instruction::Instruction, instr::part::Part},
    Register,
};

pub fn register(i: Instruction) -> String {
    let f3 = i.value(Part::Funct3).expect("invalid funct3");
    let f7 = i.value(Part::Funct7).expect("invalid funct7");
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
    let rsd: Register = i
        .value(Part::Dest)
        .expect("invalid dest")
        .try_into()
        .expect("invalid register");
    let rsd: String = rsd.try_into().unwrap();
    let op = match (f3, f7) {
        (0b000, 0b0000000) => "add".to_owned(),
        (0b000, 0b0100000) => "sub".to_owned(),
        (0b010, 0b0000000) => "slt".to_owned(),
        (0b011, 0b0000000) => "sltu".to_owned(),
        (0b001, 0b0000000) => "sll".to_owned(),
        (0b101, 0b0000000) => "srl".to_owned(),
        (0b101, 0b0100000) => "sra".to_owned(),
        (0b100, 0b0000000) => "xor".to_owned(),
        (0b110, 0b0000000) => "or".to_owned(),
        (0b111, 0b0000000) => "and".to_owned(),
        _ => unreachable!("invalid register math operation"),
    };
    format!("{} {}, {}, {}", op, rsd, rs1, rs2)
}

pub fn immediate(i: Instruction) -> String {
    let rs1: Register = i
        .value(Part::Reg1)
        .expect("invalid reg1")
        .try_into()
        .expect("invalid register");
    let rs1: String = rs1.try_into().unwrap();
    let rsd: Register = i
        .value(Part::Dest)
        .expect("invalid dest")
        .try_into()
        .expect("invalid register");
    let rsd: String = rsd.try_into().unwrap();
    let f3 = i.value(Part::Funct3).expect("invalid f3");
    match f3 {
        0b001 | 0b101 => {
            // immediate_math_shift
            let raw_immediate = i.value(Part::Imm110).expect("invalid imm110");
            let immediate = raw_immediate & 0b000_0000_0000_0000_0000_0000_0000_0001_1111;
            let shift = raw_immediate & 0b000_0000_0000_0000_0000_0000_1111_1110_0000;
            let op = match (f3, shift) {
                (0b001, 0b0000000) => "slli",
                (0b101, 0b0000000) => "srli",
                (0b101, 0b0100000) => "srai",
                _ => unreachable!("invalid immediate math operation"),
            };
            format!("{} {}, {}, {}", op, rsd, rs1, immediate)
        }
        _ => {
            // immediate_math_normal
            let immediate = i.value(Part::Imm110).expect("invalid imm110");
            let op = match f3 {
                0b000 => "addi".to_owned(),
                0b010 => "slti".to_owned(),
                0b011 => "sltiu".to_owned(),
                0b100 => "xori".to_owned(),
                0b110 => "ori".to_owned(),
                0b111 => "xori".to_owned(),
                _ => unreachable!("invalid immediate math operation"),
            };
            format!("{} {}, {}, {}", op, rsd, rs1, immediate)
        }
    }
}

#[cfg(test)]
mod immediate_math {
    use super::*;

    #[test]
    fn normal() {
        let raw = 0x00d00093; // addi x1, x0, 13
        let i = Instruction::parse(raw).expect("unable to parse");
        let expected = "addi x1, x0, 13".to_owned();
        assert_eq!(immediate(i), expected);
    }

    #[test]
    fn shift() {
        let raw = 0x00c09a93; // slli x21, x1, 12
        let i = Instruction::parse(raw).expect("unable to parse");
        let expected = "slli x21, x1, 12".to_owned();
        assert_eq!(immediate(i), expected);
    }
}

#[cfg(test)]
mod register_math {
    use super::*;

    #[test]
    fn add() {
        let raw = 0x003100b3; // add x1, x2, x3
        let i = Instruction::parse(raw).expect("unable to parse");
        let expected = "add x1, x2, x3".to_owned();
        assert_eq!(register(i), expected);
    }
}
