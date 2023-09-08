use super::part::Part;
use super::operation::Operation;

pub(crate) struct Builder {
    raw: u32
}

impl Builder {
    pub fn new(raw: u32) -> Self {
        Self{raw}
    }

    pub fn opcode(code: Operation) -> Self {
        Self::new(code as u32 | Part::Opcode as u32)
    }

    pub fn pack(&self, part: Part, data: u32) -> Self {
        let packed = match part {
            Part::Null => 0,
            Part::Opcode => data,
            Part::Dest | Part::Imm40 | Part::B11b => data << 7,
            Part::Imm41 => data << 8,
            Part::Funct3 | Part::Imm3112 | Part::Imm1912 => data << 13,
            Part::Reg1   => data << 16,
            Part::Reg2 | Part::Imm110 | Part::B11j => data << 20,
            Part::Imm101 => data << 21,
            Part::Funct7 | Part::Imm115 | Part::Imm105 => data << 25,
            Part::B12b | Part::B20j => data << 31,
        };
        Builder::new(
            self.raw | (packed & part.mask())
        )
    }

    pub fn build(&self) -> u32 {
        self.raw
    }
}

