use super::format::Format;
use super::part::Part;

#[repr(u32)]
#[derive(Debug, PartialEq, Copy, Clone)]
#[allow(clippy::upper_case_acronyms)]
pub(crate) enum Operation {
    LUI = 0b0110111,
    AUIPC = 0b0010111,
    JAL = 0b1101111,
    JALR = 0b1100111,
    FENCE = 0b0001111,

    Branch = 0b1100011,        // BEQ, BNE, BLT, BGE, BLTU, BGEU
    Load = 0b0000011,          // LB, LH, LW, LBU, LHU
    Store = 0b1000011,         // SB, SH, SW
    ImmediateMath = 0b0010011, // ADDI, SLTI, SLTIU, XORI, ORI, ANDI, SLLI, SLLI, SRLI, SRAI
    Math = 0b0110011,          // ADD, SUB, SLL, SLT, SLTU, XOR, SRL, SRA, OR, AND
    Call = 0b1110011,          // ECALL, EBREAK
}

impl Operation {
    pub(crate) fn format(&self) -> Format {
        match self {
            Self::LUI => Format::UpperImmediate,
            Self::AUIPC => Format::UpperImmediate,
            Self::JAL => Format::Jump,
            Self::JALR => Format::Immediate,
            Self::FENCE => Format::Immediate, // Nope!

            Self::Branch => Format::Branch,
            Self::Load => Format::Immediate,
            Self::Store => Format::Store,
            Self::ImmediateMath => Format::Immediate,
            Self::Math => Format::Register2register,
            Self::Call => Format::Immediate,
        }
    }
}

impl TryFrom<u32> for Operation {
    type Error = &'static str;

    fn try_from(raw: u32) -> Result<Self, Self::Error> {
        let part = Part::Opcode;
        match part.get(raw) {
            x if x == Self::LUI as u32 => Ok(Self::LUI),
            x if x == Self::AUIPC as u32 => Ok(Self::AUIPC),
            x if x == Self::JAL as u32 => Ok(Self::JAL),
            x if x == Self::JALR as u32 => Ok(Self::JALR),
            x if x == Self::FENCE as u32 => Ok(Self::FENCE),

            x if x == Self::Branch as u32 => Ok(Self::Branch),
            x if x == Self::Load as u32 => Ok(Self::Load),
            x if x == Self::Store as u32 => Ok(Self::Store),
            x if x == Self::ImmediateMath as u32 => Ok(Self::ImmediateMath),
            x if x == Self::Math as u32 => Ok(Self::Math),
            x if x == Self::Call as u32 => Ok(Self::Call),

            _ => Err("unknown opcode"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn lui_extract() {
        let part = Part::Opcode;
        let op = Operation::LUI as u32;

        assert_eq!(op, part.get(op));
    }
}
