use super::builder::Builder;
use super::part::Part;
use super::format::Format;
use super::operation::Operation;

pub struct Instruction {
    raw: u32,
    opcode: Operation,
    format: Format,
}

impl Instruction {
    pub fn parse(raw: u32) -> Result<Self, &'static str> {
        let part = Part::Opcode;
        let opcode = part.get(raw).try_into()?;

        Ok(Self{
            raw,
            opcode,
            format: opcode.format(),
        })
    }

    pub(crate) fn get(&self, part: Part) -> Result<u32, ()> {
        for x in self.format.get() {
            if x == part {
                return Ok(x.get(self.raw));
            }
        }
        Err(())
    }

    pub(crate) fn value(&self, part: Part) -> Result<u32, ()> {
        for x in self.format.get() {
            if x == part {
                return Ok(x.value(self.raw));
            }
        }
        Err(())
    }
}

mod test_instruction {
    use super::*;

    #[test]
    fn test_add_immediate() {
        let raw = 0b011111111111_00010_000_00001_0010011; // ADDI rd=1 rs=2 imm=whatever
        let inst = Instruction::parse(raw).expect("valid instruction");

        assert_eq!(inst.get(Part::Opcode).unwrap(), 0b0010011);
        assert_eq!(inst.value(Part::Opcode).unwrap(), 0b0010011);

        assert_eq!(inst.get(Part::Dest).unwrap(), 0b000001_0000000);
        assert_eq!(inst.value(Part::Dest).unwrap(), 1);

        assert_eq!(inst.get(Part::Funct3).unwrap(), 0b000_000000_0000000);
        assert_eq!(inst.value(Part::Funct3).unwrap(), 0);

        let reg1 = inst.get(Part::Reg1).unwrap();
        let expected = 0b00010_000_00000_0000000;
        assert_eq!(reg1, expected, "got: {:#034b}, want: {:#034b}", reg1, expected);
        assert_eq!(inst.value(Part::Reg1).unwrap(), 1);

        assert_eq!(inst.get(Part::Imm110).unwrap(), 0b011111111111_00000_000_00000_0000000);
        assert_eq!(inst.value(Part::Imm110).unwrap(), 0b011111111111);
    }

    #[test]
    fn test_add_reg2reg() {
        let slt = Builder::opcode(Operation::Math)
            .pack(Part::Dest, 2)
            .pack(Part::Funct3, 0b010)
            .pack(Part::Reg1, 13)
            .pack(Part::Reg2, 12)
            .pack(Part::Funct7, 0)
        .build();
        let inst = Instruction::parse(slt).expect("valid instruction");

        assert_eq!(inst.opcode, Operation::Math);

        assert_eq!(inst.value(Part::Dest).unwrap(), 2);
        assert_eq!(inst.value(Part::Funct3).unwrap(), 0b010);
        assert_eq!(inst.value(Part::Reg1).unwrap(), 13);
        assert_eq!(inst.value(Part::Reg2).unwrap(), 12);
        assert_eq!(inst.value(Part::Funct7).unwrap(), 0);
    }
}


