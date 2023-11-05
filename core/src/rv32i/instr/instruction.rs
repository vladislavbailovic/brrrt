use super::format::Format;
use super::operation::{Operation, OperationError};
use super::part::Part;
use crate::cpu::RegisterError;
use crate::memory::MemoryError;

#[derive(Debug, Clone)]
pub struct Instruction {
    pub opcode: Operation,
    pub raw: u32,
    format: Format,
}

impl Instruction {
    pub fn parse(raw: u32) -> Result<Self, InstructionError> {
        let part = Part::Opcode;
        let opcode = part.get(raw).try_into()?;

        Ok(Self {
            raw,
            opcode,
            format: opcode.format(),
        })
    }

    #[cfg(test)]
    pub(crate) fn get(&self, part: Part) -> Result<u32, InstructionError> {
        for x in self.format.get() {
            if x == part {
                return Ok(x.get(self.raw));
            }
        }
        Err(InstructionError::Get)
    }

    pub fn value(&self, part: Part) -> Result<u32, InstructionError> {
        for x in self.format.get() {
            if x == part {
                return Ok(x.value(self.raw));
            }
        }
        Err(InstructionError::Value)
    }
}

#[derive(Debug)]
pub enum InstructionError {
    #[cfg(test)]
    Get,
    Value,
    Parse,

    UnknownOperation(u32),
    InvalidOperation(Operation),

    InvalidArgument(Part),
    InvalidRegister,
    InvalidMemory,
}

impl From<OperationError> for InstructionError {
    fn from(e: OperationError) -> Self {
        match e {
            OperationError::UnknownOpcode(raw) => Self::UnknownOperation(raw),
        }
    }
}

impl From<RegisterError> for InstructionError {
    fn from(_e: RegisterError) -> Self {
        Self::InvalidRegister
    }
}

impl From<MemoryError> for InstructionError {
    fn from(_e: MemoryError) -> Self {
        Self::InvalidMemory
    }
}

impl From<InstructionError> for String {
    fn from(e: InstructionError) -> Self {
        match e {
            InstructionError::InvalidOperation(op) => format!("Invalid operation: {:?}", op),
            InstructionError::InvalidArgument(part) => format!("Unknown argument: {:?}", part),
            InstructionError::UnknownOperation(raw) => format!("Unknown operation: {}", raw),
            InstructionError::InvalidRegister => "Invalid register".to_owned(), // TODO: wat
            InstructionError::InvalidMemory => "Invalid memory".to_owned(),     // TODO: wat
            InstructionError::Value => "Unable to extract value".to_owned(),
            #[cfg(test)]
            InstructionError::Get => "Unable to get part".to_owned(),
            InstructionError::Parse => "Unable to parse instruction".to_owned(),
        }
    }
}

#[cfg(test)]
use super::builder::Builder;
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn add_immediate() {
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
        assert_eq!(
            reg1, expected,
            "got: {:#034b}, want: {:#034b}",
            reg1, expected
        );
        assert_eq!(inst.value(Part::Reg1).unwrap(), 2);

        assert_eq!(
            inst.get(Part::Imm110).unwrap(),
            0b011111111111_00000_000_00000_0000000
        );
        assert_eq!(inst.value(Part::Imm110).unwrap(), 0b011111111111);
    }

    #[test]
    fn add_reg2reg() {
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
