#[cfg(feature = "trace")]
use brrrt_core::debug;
use brrrt_core::{
    bitops,
    risc32i::{instr::instruction::Instruction, instr::part::Part},
    Register,
};

pub fn unconditional(i: Instruction) -> String {
    let rsd: Register = i
        .value(Part::Dest)
        .expect("invalid destination")
        .try_into()
        .expect("invalid register");
    #[allow(clippy::identity_op)] // readability
    let immediate = 0
        | (i.value(Part::B20j).expect("invalid b20j") << 19)
        | (i.value(Part::Imm1912).expect("invalid immediate 1912") << 11)
        | (i.value(Part::B11j).expect("invalid b11j") << 10)
        | (i.value(Part::Imm101).expect("invalid immediate 10:1") << 0);
    #[cfg(feature = "trace")]
    {
        eprintln!("\t\t- ins: {:?}", i);
        eprintln!("\t\t- rsd: {:?}", rsd);
        eprintln!(
            "\t\t\t- b20b: {}",
            debug::number(i.value(Part::B20j).unwrap(), 20)
        );
        eprintln!(
            "\t\t\t- im19: {}",
            debug::number(i.value(Part::Imm1912).unwrap(), 20)
        );
        eprintln!(
            "\t\t\t- b11j: {}",
            debug::number(i.value(Part::B11j).unwrap(), 20)
        );
        eprintln!(
            "\t\t\t- im10: {}",
            debug::number(i.value(Part::Imm101).unwrap(), 20)
        );
        eprintln!("\t\t- imm: {}", debug::number(immediate, 20));
    }

    let immediate = bitops::sign_extend(immediate, 20);
    #[cfg(feature = "trace")]
    {
        eprintln!("\t\t- sim: {}", debug::number(immediate, 20));
    }
    let rsd: String = rsd.try_into().unwrap();
    format!("jal {}, {}", rsd, immediate * 2) // TODO: why *2???
}

pub fn register(i: Instruction) -> String {
    let rsd: Register = i
        .value(Part::Dest)
        .expect("invalid destination")
        .try_into()
        .expect("invalid register");
    let rsd: String = rsd.try_into().unwrap();
    let rs1: Register = i
        .value(Part::Reg1)
        .expect("invalid reg1")
        .try_into()
        .expect("invalid register");
    let rs1: String = rs1.try_into().unwrap();
    let immediate = i.value(Part::Imm110).expect("invalid immediate value 11:0");
    format!("jalr {}, {}, {}", rsd, rs1, immediate)
}

#[cfg(test)]
mod unconditional_jump {
    use super::*;

    #[test]
    fn to_zero() {
        let raw = 0x0000006f;
        let i = Instruction::parse(raw).expect("unable to parse");
        let expected = "jal x0, 0".to_owned();
        assert_eq!(unconditional(i), expected);
    }

    #[test]
    fn to_immediate_8() {
        let raw = 0x0080066f;
        let i = Instruction::parse(raw).expect("unable to parse");
        let expected = "jal x12, 8".to_owned();
        assert_eq!(unconditional(i), expected);
    }

    #[test]
    fn to_immediate_16() {
        let raw = 0x0100006f;
        let i = Instruction::parse(raw).expect("unable to parse");
        let expected = "jal x0, 16".to_owned();
        assert_eq!(unconditional(i), expected);
    }

    #[test]
    fn to_immediate_24() {
        let raw = 0x018006ef;
        let i = Instruction::parse(raw).expect("unable to parse");
        let expected = "jal x13, 24".to_owned();
        assert_eq!(unconditional(i), expected);
    }

    #[test]
    fn to_immediate_32() {
        let raw = 0x020006ef;
        let i = Instruction::parse(raw).expect("unable to parse");
        let expected = "jal x13, 32".to_owned();
        assert_eq!(unconditional(i), expected);
    }

    #[test]
    fn to_immediate_48() {
        let raw = 0x030006ef;
        let i = Instruction::parse(raw).expect("unable to parse");
        let expected = "jal x13, 48".to_owned();
        assert_eq!(unconditional(i), expected);
    }

    #[test]
    fn to_immediate_64() {
        let raw = 0x040006ef;
        let i = Instruction::parse(raw).expect("unable to parse");
        let expected = "jal x13, 64".to_owned();
        assert_eq!(unconditional(i), expected);
    }

    #[test]
    fn to_immediate_128() {
        let raw = 0x080006ef;
        let i = Instruction::parse(raw).expect("unable to parse");
        let expected = "jal x13, 128".to_owned();
        assert_eq!(unconditional(i), expected);
    }
}

#[cfg(test)]
mod register_jump {
    use super::*;

    #[test]
    fn to_zero() {
        let raw = 0x000680e7;
        let i = Instruction::parse(raw).expect("unable to parse");
        let expected = "jalr x1, x13, 0".to_owned();
        assert_eq!(register(i), expected);
    }

    #[test]
    fn to_immediate_4() {
        let raw = 0x004680e7;
        let i = Instruction::parse(raw).expect("unable to parse");
        let expected = "jalr x1, x13, 4".to_owned();
        assert_eq!(register(i), expected);
    }

    #[test]
    fn to_immediate_8() {
        let raw = 0x00868667;
        let i = Instruction::parse(raw).expect("unable to parse");
        let expected = "jalr x12, x13, 8".to_owned();
        assert_eq!(register(i), expected);
    }

    #[test]
    fn to_immediate_16() {
        let raw = 0x01068667;
        let i = Instruction::parse(raw).expect("unable to parse");
        let expected = "jalr x12, x13, 16".to_owned();
        assert_eq!(register(i), expected);
    }

    #[test]
    fn to_immediate_162() {
        let raw = 0x0a268667;
        let i = Instruction::parse(raw).expect("unable to parse");
        let expected = "jalr x12, x13, 162".to_owned();
        assert_eq!(register(i), expected);
    }
}
