use super::operation::Operation;
use super::part::Part;

pub(crate) struct Builder {
    raw: u32,
}

impl Builder {
    pub(crate) fn new(raw: u32) -> Self {
        Self { raw }
    }

    pub(crate) fn opcode(code: Operation) -> Self {
        Self::new(code as u32 | Part::Opcode as u32)
    }

    pub(crate) fn pack(&self, part: Part, data: u32) -> Self {
        let packed = data << part.shift();
        let masked = packed & part.mask();
        // eprintln!("data  : {:#034b}", data);
        // eprintln!("packed: {:#034b}", packed);
        // eprintln!("mask  : {:#034b}", part.mask());
        // eprintln!("masked: {:#034b}", masked);
        Builder::new(self.raw | masked)
    }

    pub fn build(&self) -> u32 {
        self.raw
    }
}

#[cfg(test)]
use super::instruction::Instruction;
#[cfg(test)]
mod test_reg1 {
    use super::*;

    #[test]
    fn test_pack_zero_reg1() {
        let i = Builder::opcode(Operation::Math).pack(Part::Reg1, 0).build();
        let p = Instruction::parse(i).unwrap();
        assert_eq!(p.value(Part::Reg1).unwrap(), 0);
    }

    #[test]
    fn test_pack_x1_reg1() {
        let i = Builder::opcode(Operation::Math).pack(Part::Reg1, 1).build();
        let p = Instruction::parse(i).unwrap();
        assert_eq!(p.value(Part::Reg1).unwrap(), 1);
    }

    #[test]
    fn test_pack_x16_reg1() {
        let i = Builder::opcode(Operation::Math)
            .pack(Part::Reg1, 16)
            .build();
        eprintln!("instruction: {:#034b}", i);
        let p = Instruction::parse(i).unwrap();
        assert_eq!(p.value(Part::Reg1).unwrap(), 16);
    }

    #[test]
    fn test_pack_x31_reg1() {
        let i = Builder::opcode(Operation::Math)
            .pack(Part::Reg1, 31)
            .build();
        eprintln!("instruction: {:#034b}", i);
        let p = Instruction::parse(i).unwrap();
        assert_eq!(p.value(Part::Reg1).unwrap(), 31);
    }
}

#[cfg(test)]
mod test_reg2 {
    use super::*;

    #[test]
    fn test_pack_zero_reg2() {
        let i = Builder::opcode(Operation::Math).pack(Part::Reg2, 0).build();
        let p = Instruction::parse(i).unwrap();
        assert_eq!(p.value(Part::Reg2).unwrap(), 0);
    }

    #[test]
    fn test_pack_x1_reg2() {
        let i = Builder::opcode(Operation::Math).pack(Part::Reg2, 1).build();
        let p = Instruction::parse(i).unwrap();
        assert_eq!(p.value(Part::Reg2).unwrap(), 1);
    }

    #[test]
    fn test_pack_x16_reg2() {
        let i = Builder::opcode(Operation::Math)
            .pack(Part::Reg2, 16)
            .build();
        eprintln!("instruction: {:#034b}", i);
        let p = Instruction::parse(i).unwrap();
        assert_eq!(p.value(Part::Reg2).unwrap(), 16);
    }

    #[test]
    fn test_pack_x31_reg2() {
        let i = Builder::opcode(Operation::Math)
            .pack(Part::Reg2, 31)
            .build();
        eprintln!("instruction: {:#034b}", i);
        let p = Instruction::parse(i).unwrap();
        assert_eq!(p.value(Part::Reg2).unwrap(), 31);
    }
}

#[cfg(test)]
mod test_reg_dest {
    use super::*;

    #[test]
    fn test_pack_zero_reg_dest() {
        let i = Builder::opcode(Operation::Math).pack(Part::Dest, 0).build();
        let p = Instruction::parse(i).unwrap();
        assert_eq!(p.value(Part::Dest).unwrap(), 0);
    }

    #[test]
    fn test_pack_x1_reg_dest() {
        let i = Builder::opcode(Operation::Math).pack(Part::Dest, 1).build();
        let p = Instruction::parse(i).unwrap();
        assert_eq!(p.value(Part::Dest).unwrap(), 1);
    }

    #[test]
    fn test_pack_x16_reg_dest() {
        let i = Builder::opcode(Operation::Math)
            .pack(Part::Dest, 16)
            .build();
        eprintln!("instruction: {:#034b}", i);
        let p = Instruction::parse(i).unwrap();
        assert_eq!(p.value(Part::Dest).unwrap(), 16);
    }

    #[test]
    fn test_pack_x31_reg_dest() {
        let i = Builder::opcode(Operation::Math)
            .pack(Part::Dest, 31)
            .build();
        eprintln!("instruction: {:#034b}", i);
        let p = Instruction::parse(i).unwrap();
        assert_eq!(p.value(Part::Dest).unwrap(), 31);
    }
}

#[cfg(test)]
mod test_imm110 {
    use super::*;

    #[test]
    fn test_pack_imm110_zero_unsigned() {
        let i = Builder::opcode(Operation::ImmediateMath)
            .pack(Part::Imm110, 0)
            .build();
        eprintln!("instruction: {:#034b}", i);
        let p = Instruction::parse(i).unwrap();
        assert_eq!(p.value(Part::Imm110).unwrap(), 0);
    }

    #[test]
    fn test_pack_imm110_one_unsigned() {
        let i = Builder::opcode(Operation::ImmediateMath)
            .pack(Part::Imm110, 1)
            .build();
        eprintln!("instruction: {:#034b}", i);
        let p = Instruction::parse(i).unwrap();
        assert_eq!(p.value(Part::Imm110).unwrap(), 1);
    }

    #[test]
    fn test_pack_imm110_minus_one_signed() {
        let neg = -1;
        let pos = neg as u32;
        let i = Builder::opcode(Operation::ImmediateMath)
            .pack(Part::Imm110, pos)
            .build();
        eprintln!("instruction: {:#034b}", i);
        let p = Instruction::parse(i).unwrap();

        let pres = p.value(Part::Imm110).unwrap();
        let nres = 4094 - pres as i32;
        assert_eq!(pres, 4095);
        assert_eq!(nres, -1);
    }
}

#[cfg(test)]
mod imm40 {
    use super::*;

    fn pack_imm40_zero() {
        let i = Builder::opcode(Operation::ImmediateMath)
            .pack(Part::Imm40, 0)
            .build();
        eprintln!("instruction: {:#034b}", i);
        let p = Instruction::parse(i).unwrap();
        assert_eq!(p.value(Part::Imm40).unwrap(), 0);
    }

    fn pack_imm40_one() {
        let i = Builder::opcode(Operation::ImmediateMath)
            .pack(Part::Imm40, 1)
            .build();
        eprintln!("instruction: {:#034b}", i);
        let p = Instruction::parse(i).unwrap();
        assert_eq!(p.value(Part::Imm40).unwrap(), 1);
    }

    fn pack_imm40_five() {
        let i = Builder::opcode(Operation::ImmediateMath)
            .pack(Part::Imm40, 5)
            .build();
        eprintln!("instruction: {:#034b}", i);
        let p = Instruction::parse(i).unwrap();
        assert_eq!(p.value(Part::Imm40).unwrap(), 5);
    }

    fn pack_imm40_one_imm115_one() {
        let i = Builder::opcode(Operation::ImmediateMath)
            .pack(Part::Imm40, 1)
            .pack(Part::Imm115, 1)
            .build();
        eprintln!("instruction: {:#034b}", i);
        let p = Instruction::parse(i).unwrap();
        assert_eq!(p.value(Part::Imm40).unwrap(), 1);
        assert_eq!(p.value(Part::Imm115).unwrap(), 1);
    }

    fn pack_imm40_five_imm115_one() {
        let i = Builder::opcode(Operation::ImmediateMath)
            .pack(Part::Imm40, 5)
            .pack(Part::Imm115, 1)
            .build();
        eprintln!("instruction: {:#034b}", i);
        let p = Instruction::parse(i).unwrap();
        assert_eq!(p.value(Part::Imm40).unwrap(), 5);
        assert_eq!(p.value(Part::Imm115).unwrap(), 1);
    }
}
