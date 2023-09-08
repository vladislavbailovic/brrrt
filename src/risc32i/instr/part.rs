#[derive(PartialEq, Copy, Clone)]
pub(crate) enum Part {
    Null,

    Opcode,

    Dest,
    Reg1,
    Reg2,

    Funct3,
    Funct7,

    Imm110,
    Imm3112,
    Imm40,
    Imm41,
    Imm105,
    Imm115,
    Imm1912,
    Imm101,

    B11b,
    B12b,
    B11j,
    B20j,
}


impl Part {
    pub(crate) fn mask(&self) -> u32 {
        match self {
            Self::Null => 0b0000_0000_0000_0000_0000_0000_0000_0000,

            Self::Opcode => 0b0000_0000_0000_0000_0000_0000_0111_1111,
            Self::Dest   => 0b0000_0000_0000_0000_0000_1111_1000_0000,

            // Register/register: Operation -> Dest ->
            Self::Funct3 => 0b0000_0000_0000_0000_0111_0000_0000_0000,
            Self::Reg1   => 0b0000_0000_0000_1111_1000_0000_0000_0000,
            Self::Reg2   => 0b0000_0001_1111_0000_0000_0000_0000_0000,
            Self::Funct7 => 0b1111_1110_0000_0000_0000_0000_0000_0000,

            // Immediate: Operation -> Dest -> Funct3 -> Reg1 ->
            Self::Imm110  => 0b1111_1111_1111_0000_0000_0000_0000_0000,

            // Upper immediate: Operation -> Dest -> 
            Self::Imm3112 => 0b1111_1111_1111_1111_1111_0000_0000_0000,

            // Store: Operation ->
            Self::Imm40  => 0b0000_0000_0000_0000_0000_1111_1000_0000,
            // -> Funct3 -> Reg1 -> Reg2 ->
            Self::Imm115 => 0b1111_1110_0000_0000_0000_0000_0000_0000,

            // Branch: Operation -> 
            Self::B11b   => 0b0000_0000_0000_0000_0000_0000_1000_0000,
            Self::Imm41  => 0b0000_0000_0000_0000_0000_1111_0000_0000,
            // -> Reg1 -> Reg2 ->
            Self::Imm105 => 0b0111_1110_0000_0000_0000_0000_0000_0000,
            Self::B12b   => 0b1000_0000_0000_0000_0000_0000_0000_0000,

            // Jump: Operation -> Dest ->
            Self::Imm1912 => 0b0000_0000_0000_1111_1100_0000_0000_0000,
            Self::B11j    => 0b0000_0000_0001_0000_0000_0000_0000_0000,
            Self::Imm101  => 0b0111_1111_1110_0000_0000_0000_0000_0000,
            Self::B20j    => 0b1000_0000_0000_0000_0000_0000_0000_0000,
        }
    }

    pub(crate) fn get(&self, from: u32) -> u32 {
        let mask = Self::mask(self);
        from & mask
    }

    pub(crate) fn value(&self, from: u32) -> u32 {
        let raw = self.get(from);
        match self {
            Self::Null => 0,
            Self::Opcode => raw,
            Self::Dest | Self::Imm40 | Self::B11b => raw >> 7,
            Self::Imm41 => raw >> 8,
            Self::Funct3 | Self::Imm3112 | Self::Imm1912 => raw >> 13,
            Self::Reg1   => raw >> 16,
            Self::Reg2 | Self::Imm110 | Self::B11j => raw >> 20,
            Self::Imm101 => raw >> 21,
            Self::Funct7 | Self::Imm115 | Self::Imm105 => raw >> 25,
            Self::B12b | Self::B20j => raw >> 31,
        }
    }

}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn null_is_zero() {
        let part = Part::Null;
        assert_eq!(part as u32, 0);
    }

    #[test]
    fn part_opcode_extracts_7bit_opcode() {
        let part = Part::Opcode;

        let actual = part.get(1);
        assert_eq!(actual, 1, "actual: {}", actual);

        let actual = part.get(4);
        assert_eq!(actual, 4, "actual: {}", actual);

        let actual = part.get(8);
        assert_eq!(actual, 8, "actual: {}", actual);

        let actual = part.get(16);
        assert_eq!(actual, 16, "actual: {}", actual);

        let actual = part.get(32);
        assert_eq!(actual, 32, "actual: {}", actual);

        let actual = part.get(64);
        assert_eq!(actual, 64, "actual: {}", actual);

        let actual = part.get(127);
        assert_eq!(actual, 127, "actual: {}", actual);
    }

    #[test]
    fn part_opcode_extracts_7bit_opcode_expected_wrap() {
        let part = Part::Opcode;

        // Expect wrap
        let actual = part.get(128);
        assert_eq!(actual, 0, "actual: {}", actual);

        let actual = part.get(129);
        assert_eq!(actual, 1, "actual: {}", actual);
    }
}
