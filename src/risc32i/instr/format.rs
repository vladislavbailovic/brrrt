use super::part::Part;

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
            Self::UpperImmediate => vec![
                Part::Opcode,
                Part::Dest,
                Part::Imm3112,
            ],
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
mod test{
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
}
