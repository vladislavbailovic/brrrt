use super::part::Part;

#[derive(Debug, Clone)]
pub(crate) enum Format {
    Register2register,
    Immediate,
    UpperImmediate,
    Store,
    Branch,
    Jump,
}

impl Format {
    pub(crate) fn get(&self) -> Vec<Part> {
        match self {
            Self::Register2register => vec![
                Part::Opcode,
                Part::Dest,
                Part::Funct3,
                Part::Reg1,
                Part::Reg2,
                Part::Funct7,
            ],
            Self::Immediate => vec![
                Part::Opcode,
                Part::Dest,
                Part::Funct3,
                Part::Reg1,
                Part::Imm110,
            ],
            Self::UpperImmediate => vec![Part::Opcode, Part::Dest, Part::Imm3112],
            Self::Store => vec![
                Part::Opcode,
                Part::Imm40,
                Part::Funct3,
                Part::Reg1,
                Part::Reg2,
                Part::Imm115,
            ],
            Self::Branch => vec![
                Part::Opcode,
                Part::B11b,
                Part::Imm41,
                Part::Funct3,
                Part::Reg1,
                Part::Reg2,
                Part::Imm105,
                Part::B12b,
            ],
            Self::Jump => vec![
                Part::Opcode,
                Part::Dest,
                Part::Imm1912,
                Part::B11j,
                Part::Imm101,
                Part::B20j,
            ],
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn upper_immediate_parts() {
        let fmt = Format::UpperImmediate;
        let parts = fmt.get();
        assert_eq!(parts.len(), 3, "got: {}", parts.len());
    }

    #[test]
    fn upper_immediate_parts_extraction() {
        let fmt = Format::UpperImmediate;
        let instr = 224; // 00000000000000000000000011100000;
        for part in fmt.get() {
            match part {
                Part::Opcode => assert_eq!(0b00000000000000000000000001100000, part.get(instr)),
                Part::Dest => assert_eq!(0b00000000000000000000000010000000, part.get(instr)),
                Part::Imm3112 => assert_eq!(0b00000000000000000000000000000000, part.get(instr)),
                _ => assert!(false, "should not happen"),
            }
        }
    }

    #[test]
    fn jump_parts() {
        let fmt = Format::Jump;
        assert_eq!(fmt.get().len(), 6);
    }

    fn printp(wat: &str, num: u32) {
        eprintln!("{:>6}: {:#034b} ({})", wat, num, num);
    }

    #[test]
    fn branch() {
        use super::super::{builder::Builder, instruction::Instruction, operation::Operation};
        let i = Instruction::parse(
            Builder::opcode(Operation::Branch)
                .pack(Part::B11b, 0)
                .pack(Part::Imm41, 0b1000)
                .pack(Part::Imm105, 0b000000)
                .pack(Part::B12b, 0)
                .build(),
        )
        .expect("should parse");
        printp("B11b", i.value(Part::B11b).expect("invalid B11b"));
        printp("Imm41", i.value(Part::Imm41).expect("invalid Imm41"));
        printp("Imm105", i.value(Part::Imm105).expect("invalid Imm105"));
        printp("B12b", i.value(Part::B12b).expect("invalid B12b"));

        // B12b -> MSB
        let value = 0
            | (i.value(Part::B12b).expect("invalid B12b") << 11)
            | (i.value(Part::B11b).expect("invalid B11b") << 10)
            | (i.value(Part::Imm105).expect("invalid Imm105") << 4)
            | (i.value(Part::Imm41).expect("invalid Imm41") << 0);
        printp("value", value >> 1);

        assert_eq!(value, 0b000000001000, "CS61C Su18 - Lecture 7, page 45");
    }
}
