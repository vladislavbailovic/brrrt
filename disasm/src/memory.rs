use brrrt_core::{
    risc32i::{instr::instruction::Instruction, instr::part::Part},
    Register,
};

pub fn load(i: Instruction) -> String {
    let rsd: Register = i
        .value(Part::Dest)
        .expect("invalid dest")
        .try_into()
        .expect("invalid register");
    let rsd: String = rsd.try_into().unwrap();
    let rs1: Register = i
        .value(Part::Reg1)
        .expect("invalid reg1")
        .try_into()
        .expect("invalid register");
    let rs1: String = rs1.try_into().unwrap();
    let f3 = i.value(Part::Funct3).expect("invalid funct3");
    let immediate = i.value(Part::Imm110).expect("invalid imm110");
    let op = match f3 {
        0b000 => "lb".to_owned(),
        0b001 => "lh".to_owned(),
        0b010 => "lw".to_owned(),
        0b100 => "lbu".to_owned(),
        0b101 => "lhu".to_owned(),
        0b011 => "ld".to_owned(), // wat? Not in spec o.0! "Arbitrary load" apparently
        _ => unreachable!("invalid load operation"),
    };
    format!("{} {}, {}({})", op, rsd, immediate, rs1)
}

pub fn store(i: Instruction) -> String {
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
    let im40 = i.value(Part::Imm40).expect("invalid imm40");
    let im115 = i.value(Part::Imm115).expect("invalid imm115");
    let immediate = (im115 << 5) | im40; // https://stackoverflow.com/a/60239441
    let op = match f3 {
        0b000 => "sb".to_owned(),
        0b001 => "sh".to_owned(),
        0b010 => "sw".to_owned(),
        0b011 => "sd".to_owned(), // wat? Not in spec o.0! "Arbitrary store" apparently
        _ => unreachable!("invalid store instruction"),
    };
    format!("{} {}, {}({})", op, rs2, immediate, rs1)
}

#[cfg(test)]
mod load_test {
    use super::*;

    #[test]
    fn load_ld_0() {
        let raw = 0x0006b603;
        let i = Instruction::parse(raw).expect("unable to parse");
        let expected = "ld x12, 0(x13)".to_owned();
        assert_eq!(load(i), expected);
    }

    #[test]
    fn load_ld_13() {
        let raw = 0x00c6b603;
        let i = Instruction::parse(raw).expect("unable to parse");
        let expected = "ld x12, 12(x13)".to_owned();
        assert_eq!(load(i), expected);
    }

    #[test]
    fn load_lb_0() {
        let raw = 0x00068603;
        let i = Instruction::parse(raw).expect("unable to parse");
        let expected = "lb x12, 0(x13)".to_owned();
        assert_eq!(load(i), expected);
    }

    #[test]
    fn load_lb_13() {
        let raw = 0x00c68603;
        let i = Instruction::parse(raw).expect("unable to parse");
        let expected = "lb x12, 12(x13)".to_owned();
        assert_eq!(load(i), expected);
    }
}

#[cfg(test)]
mod store_test {
    use super::*;

    #[test]
    fn test_sd_0() {
        let raw = 0x0100b023;
        let i = Instruction::parse(raw).expect("unable to parse");
        let expected = "sd x16, 0(x1)".to_owned();
        assert_eq!(store(i), expected);
    }

    #[test]
    fn test_sd_32() {
        let raw = 0x0300b023;
        let i = Instruction::parse(raw).expect("unable to parse");
        let expected = "sd x16, 32(x1)".to_owned();
        assert_eq!(store(i), expected);
    }
}
