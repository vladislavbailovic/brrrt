use brrrt_core::{
    rv32i::{instr::instruction::Instruction, instr::part::Part},
    Register,
};

pub fn load(i: Instruction) -> String {
    let rsd: Register = i
        .value(Part::Dest)
        .expect("invalid dest")
        .try_into()
        .expect("invalid register");
    let rsd: String = rsd.try_into().unwrap();
    let immediate = i.value(Part::Imm3112).expect("invalid immediate 31:12");
    format!("lui {}, {}", rsd, immediate)
}

pub fn add(i: Instruction) -> String {
    let rsd: Register = i
        .value(Part::Dest)
        .expect("invalid dest")
        .try_into()
        .expect("invalid register");
    let rsd: String = rsd.try_into().unwrap();
    let immediate = i.value(Part::Imm3112).expect("invalid immediate 31:12");
    format!("auipc {}, {}", rsd, immediate)
}

#[cfg(test)]
mod load_test {
    use super::*;

    #[test]
    fn upper_immediate_0() {
        let raw = 0x000000b7;
        let i = Instruction::parse(raw).expect("should parse");
        let expected = "lui x1, 0".to_owned();
        assert_eq!(load(i), expected);
    }

    #[test]
    fn upper_immediate_1312() {
        let raw = 0x005200b7;
        let i = Instruction::parse(raw).expect("should parse");
        let expected = "lui x1, 1312".to_owned();
        assert_eq!(load(i), expected);
    }

    #[test]
    fn upper_immediate_13_12() {
        let raw = 0x0000c6b7;
        let i = Instruction::parse(raw).expect("should parse");
        let expected = "lui x13, 12".to_owned();
        assert_eq!(load(i), expected);
    }
}

#[cfg(test)]
mod add_test {
    use super::*;

    #[test]
    fn upper_immediate_0() {
        let raw = 0x00000097;
        let i = Instruction::parse(raw).expect("should parse");
        let expected = "auipc x1, 0".to_owned();
        assert_eq!(add(i), expected);
    }

    #[test]
    fn upper_immediate_1312() {
        let raw = 0x00520097;
        let i = Instruction::parse(raw).expect("should parse");
        let expected = "auipc x1, 1312".to_owned();
        assert_eq!(add(i), expected);
    }

    #[test]
    fn upper_immediate_13_12() {
        let raw = 0x0000c697;
        let i = Instruction::parse(raw).expect("should parse");
        let expected = "auipc x13, 12".to_owned();
        assert_eq!(add(i), expected);
    }
}
