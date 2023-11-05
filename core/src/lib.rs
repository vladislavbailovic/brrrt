pub mod bitops;
pub mod cpu;
pub mod debug;
pub mod elf32;
pub mod memory;
pub mod program;
pub mod rv32i;

// tests
#[cfg(test)]
mod branches;
#[cfg(test)]
mod immediate;
#[cfg(test)]
mod immediate_math;
#[cfg(test)]
mod jumps;
#[cfg(test)]
mod load;
#[cfg(test)]
mod math;
#[cfg(test)]
mod store;

pub use cpu::{Register, Registers, CPU, REGISTER_INCREMENT};
pub use memory::Memory;
pub use program::Program;
use rv32i::{
    instr::instruction::{Instruction, InstructionError},
    instr::operation::{Operation, OperationError},
    instr::part::Part,
};

#[derive(Default, Debug)]
pub struct VM {
    pub cpu: CPU,
    pub ram: Memory,
    #[cfg(feature = "debug")]
    debug: Vec<String>,
    #[cfg(feature = "debug")]
    last: Option<Instruction>,
}

impl VM {
    #[cfg(feature = "debug")]
    pub fn debug(&self) -> Vec<String> {
        self.debug.clone()
    }
    #[cfg(feature = "debug")]
    pub fn last(&self) -> Option<Instruction> {
        self.last.clone()
    }

    pub fn execute(&mut self, i: Instruction) -> Result<(), InstructionError> {
        self.debug.clear();
        self.last = Some(i.clone());
        let result = match i.opcode {
            Operation::LUI => self.load_upper_immediate(i),
            Operation::AUIPC => self.add_upper_immediate(i),
            Operation::Math => self.register_math(i),
            Operation::ImmediateMath => self.immediate_math(i),
            Operation::JAL => self.unconditional_jump(i),
            Operation::JALR => self.unconditional_register_jump(i),
            Operation::Branch => self.branch(i),
            Operation::Load => self.load(i),
            Operation::Store => self.store(i),
            _ => Err(OperationError::UnknownOpcode(i.raw).into()),
        };
        if result.is_ok() {
            self.cpu.increment_pc();
        }
        result
    }

    fn store(&mut self, i: Instruction) -> Result<(), InstructionError> {
        let rs1: Register = i
            .value(Part::Reg1)
            .or(Err(InstructionError::InvalidArgument(Part::Reg1)))?
            .try_into()?;
        let rs2: Register = i
            .value(Part::Reg2)
            .or(Err(InstructionError::InvalidArgument(Part::Reg1)))?
            .try_into()?;
        let f3 = i
            .value(Part::Funct3)
            .or(Err(InstructionError::InvalidArgument(Part::Funct3)))?;
        let im40 = i
            .value(Part::Imm40)
            .or(Err(InstructionError::InvalidArgument(Part::Imm40)))?;
        let im115 = i
            .value(Part::Imm115)
            .or(Err(InstructionError::InvalidArgument(Part::Imm115)))?;
        let immediate = (im115 << 5) | im40; // https://stackoverflow.com/a/60239441
        let address = self.cpu.register.get(rs1) as i32 + bitops::sign_extend(immediate, 12);

        #[cfg(any(feature = "trace", feature = "debug"))]
        {
            let debug = vec![
                format!("\t\trs1: {:?}", rs1),
                format!("\t\trs2: {:?}", rs2),
                format!("\t\t f3: {}", debug::number(f3, 5)),
                format!("\t\tim1: {}", debug::number(im115, 12)),
                format!("\t\tim4: {}", debug::number(im40, 12)),
                format!("\t\timm: {}", debug::number(immediate, 12)),
                format!(
                    "\t\text: {}",
                    debug::number(bitops::sign_extend(immediate, 12), 12)
                ),
                format!("\t\tadr: {}", debug::number(address, 12)),
            ];
            #[cfg(feature = "trace")]
            eprintln!("{}", debug.join("\n"));
            #[cfg(feature = "debug")]
            self.debug.extend_from_slice(&debug);
        }

        match f3 {
            0b000 => {
                // SB
                self.ram
                    .set_byte_at(address as u32, self.cpu.register.get(rs2) as u8)?;
                Ok(())
            }
            0b001 => {
                // SH
                self.ram
                    .set_hw_at(address as u32, self.cpu.register.get(rs2) as u16)?;
                Ok(())
            }
            0b010 => {
                // SW
                self.ram
                    .set_word_at(address as u32, self.cpu.register.get(rs2))?;
                Ok(())
            }
            _ => Err(InstructionError::InvalidOperation(Operation::Store)),
        }
    }

    fn load(&mut self, i: Instruction) -> Result<(), InstructionError> {
        let rsd: Register = i
            .value(Part::Dest)
            .or(Err(InstructionError::InvalidArgument(Part::Dest)))?
            .try_into()?;
        let rs1: Register = i
            .value(Part::Reg1)
            .or(Err(InstructionError::InvalidArgument(Part::Reg1)))?
            .try_into()?;
        let f3 = i
            .value(Part::Funct3)
            .or(Err(InstructionError::InvalidArgument(Part::Funct3)))?;
        let immediate = i
            .value(Part::Imm110)
            .or(Err(InstructionError::InvalidArgument(Part::Imm110)))?;
        let address = self.cpu.register.get(rs1) as i32 + bitops::sign_extend(immediate, 12);

        #[cfg(any(feature = "trace", feature = "debug"))]
        {
            let debug = vec![
                format!("\t\trsd: {:?}", rsd),
                format!("\t\trs1: {:?}", rs1),
                format!("\t\t f3: {}", debug::number(f3, 3)),
                format!("\t\timm: {}", debug::number(immediate, 12)),
                format!("\t\tadr: {}", debug::number(address, 12)),
            ];
            #[cfg(feature = "trace")]
            eprintln!("{}", debug.join("\n"));
            #[cfg(feature = "debug")]
            self.debug.extend_from_slice(&debug);
        }

        match f3 {
            0b000 => {
                // LB
                let value = self.ram.byte_at(
                    address
                        .try_into()
                        .or(Err(InstructionError::InvalidMemory))?,
                )?;
                self.cpu
                    .register
                    .set(rsd, bitops::sign_extend(value as u32, 8) as u32);
                Ok(())
            }
            0b001 => {
                // LH
                let value = self.ram.hw_at(
                    address
                        .try_into()
                        .or(Err(InstructionError::InvalidMemory))?,
                )?;
                self.cpu
                    .register
                    .set(rsd, bitops::sign_extend(value as u32, 16) as u32);
                Ok(())
            }
            0b010 => {
                // LW
                self.cpu.register.set(
                    rsd,
                    self.ram.word_at(
                        address
                            .try_into()
                            .or(Err(InstructionError::InvalidMemory))?,
                    )?,
                );
                Ok(())
            }
            0b100 => {
                // LBU
                let value = self.ram.byte_at(
                    address
                        .try_into()
                        .or(Err(InstructionError::InvalidMemory))?,
                )?;
                self.cpu.register.set(rsd, value as u32);
                Ok(())
            }
            0b101 => {
                // LHU
                let value = self.ram.hw_at(
                    address
                        .try_into()
                        .or(Err(InstructionError::InvalidMemory))?,
                )?;
                self.cpu.register.set(rsd, value as u32);
                Ok(())
            }
            _ => Err(InstructionError::InvalidOperation(Operation::Load)),
        }
    }

    fn add_upper_immediate(&mut self, i: Instruction) -> Result<(), InstructionError> {
        let rsd: Register = i
            .value(Part::Dest)
            .or(Err(InstructionError::InvalidArgument(Part::Dest)))?
            .try_into()?;
        let immediate = i
            .value(Part::Imm3112)
            .or(Err(InstructionError::InvalidArgument(Part::Imm3112)))?;
        let pc = self.cpu.register.get(Register::PC);

        #[cfg(any(feature = "trace", feature = "debug"))]
        {
            let debug = vec![
                format!("\t\trsd: {:?}", rsd),
                format!("\t\timm: {}", debug::number(immediate, 20)),
                format!("\t\t pc: {}", pc),
            ];
            #[cfg(feature = "trace")]
            eprintln!("{}", debug.join("\n"));
            #[cfg(feature = "debug")]
            self.debug.extend_from_slice(&debug);
        }

        self.cpu.register.set(
            rsd,
            (immediate & 0b0000_0000_0000_1111_1111_1111_1111_1111) + pc + REGISTER_INCREMENT,
        );
        Ok(())
    }

    fn load_upper_immediate(&mut self, i: Instruction) -> Result<(), InstructionError> {
        let rsd: Register = i
            .value(Part::Dest)
            .or(Err(InstructionError::InvalidArgument(Part::Dest)))?
            .try_into()?;
        let immediate = i
            .value(Part::Imm3112)
            .or(Err(InstructionError::InvalidArgument(Part::Imm3112)))?;

        #[cfg(any(feature = "trace", feature = "debug"))]
        {
            let debug = vec![
                format!("\t\trsd: {:?}", rsd),
                format!("\t\timm: {}", debug::number(immediate, 20)),
            ];
            #[cfg(feature = "trace")]
            eprintln!("{}", debug.join("\n"));
            #[cfg(feature = "debug")]
            self.debug.extend_from_slice(&debug);
        }

        self.cpu
            .register
            .set(rsd, immediate & 0b0000_0000_0000_1111_1111_1111_1111_1111);
        Ok(())
    }

    #[allow(clippy::identity_op)] // readability
    fn branch(&mut self, i: Instruction) -> Result<(), InstructionError> {
        let rs1: Register = i
            .value(Part::Reg1)
            .or(Err(InstructionError::InvalidArgument(Part::Reg1)))?
            .try_into()?;
        let rs2: Register = i
            .value(Part::Reg2)
            .or(Err(InstructionError::InvalidArgument(Part::Reg2)))?
            .try_into()?;
        let f3 = i
            .value(Part::Funct3)
            .or(Err(InstructionError::InvalidArgument(Part::Funct3)))?;
        let pc = (self.cpu.register.get(Register::PC) as i32) - REGISTER_INCREMENT as i32;

        #[cfg(any(feature = "trace", feature = "debug"))]
        {
            let debug = vec![
                format!("\t\t- rs1: {:?}", rs1),
                format!("\t\t- rs2: {:?}", rs2),
                format!("\t\t-  pc: {}", pc),
                format!("\t\t-  f3: {}", debug::number(f3, 3)),
            ];
            #[cfg(feature = "trace")]
            eprintln!("{}", debug.join("\n"));
            #[cfg(feature = "debug")]
            self.debug.extend_from_slice(&debug);
        }

        let immediate = 0
            | (i.value(Part::B12b)
                .or(Err(InstructionError::InvalidArgument(Part::B12b)))?
                << 11)
            | (i.value(Part::B11b)
                .or(Err(InstructionError::InvalidArgument(Part::B11b)))?
                << 10)
            | (i.value(Part::Imm105)
                .or(Err(InstructionError::InvalidArgument(Part::Imm105)))?
                << 4)
            | (i.value(Part::Imm41)
                .or(Err(InstructionError::InvalidArgument(Part::Imm41)))?
                << 0);
        #[cfg(any(feature = "trace", feature = "debug"))]
        {
            let debug = vec![
                format!(
                    "\t\t\t- b12b: {}",
                    debug::number(i.value(Part::B12b).unwrap(), 12)
                ),
                format!(
                    "\t\t\t- b11b: {}",
                    debug::number(i.value(Part::B11b).unwrap(), 12)
                ),
                format!(
                    "\t\t\t- imm1: {}",
                    debug::number(i.value(Part::Imm105).unwrap(), 12)
                ),
                format!(
                    "\t\t\t- imm4: {}",
                    debug::number(i.value(Part::Imm41).unwrap(), 12)
                ),
                format!("\t\t- rim: {}", debug::number(immediate, 12)),
            ];
            #[cfg(feature = "trace")]
            eprintln!("{}", debug.join("\n"));
            #[cfg(feature = "debug")]
            self.debug.extend_from_slice(&debug);
        }

        let immediate = (immediate >> 1) << 1;
        let address = bitops::sign_extend(immediate, 12) * 2;
        #[cfg(any(feature = "trace", feature = "debug"))]
        {
            let debug = vec![
                format!("\t\t- imm: {}", debug::number(immediate, 12)),
                format!("\t\t- adr: {}", debug::number(address, 12)),
            ];
            #[cfg(feature = "trace")]
            eprintln!("{}", debug.join("\n"));
            #[cfg(feature = "debug")]
            self.debug.extend_from_slice(&debug);
        }

        match f3 {
            0b000 => {
                // BEQ
                if self.cpu.register.get(rs1) == self.cpu.register.get(rs2) {
                    self.cpu.register.set(Register::PC, (pc + address) as u32);
                }
                Ok(())
            }
            0b001 => {
                // BNE
                if self.cpu.register.get(rs1) != self.cpu.register.get(rs2) {
                    self.cpu.register.set(Register::PC, (pc + address) as u32);
                }
                Ok(())
            }
            0b100 => {
                // BLT
                if (self.cpu.register.get(rs1) as i32) < (self.cpu.register.get(rs2) as i32) {
                    self.cpu.register.set(Register::PC, (pc + address) as u32);
                }
                Ok(())
            }
            0b110 => {
                // BLTU
                if self.cpu.register.get(rs1) < self.cpu.register.get(rs2) {
                    self.cpu.register.set(Register::PC, (pc + address) as u32);
                }
                Ok(())
            }
            0b101 => {
                // BGE
                if (self.cpu.register.get(rs1) as i32) > (self.cpu.register.get(rs2) as i32) {
                    self.cpu.register.set(Register::PC, (pc + address) as u32);
                }
                Ok(())
            }
            0b111 => {
                // BGEU
                if self.cpu.register.get(rs1) > self.cpu.register.get(rs2) {
                    self.cpu.register.set(Register::PC, (pc + address) as u32);
                }
                Ok(())
            }
            _ => Err(InstructionError::InvalidOperation(Operation::Branch)),
        }
    }

    fn unconditional_register_jump(&mut self, i: Instruction) -> Result<(), InstructionError> {
        let rsd: Register = i
            .value(Part::Dest)
            .or(Err(InstructionError::InvalidArgument(Part::Dest)))?
            .try_into()?;
        let rs1: Register = i
            .value(Part::Reg1)
            .or(Err(InstructionError::InvalidArgument(Part::Reg1)))?
            .try_into()?;
        let immediate = i
            .value(Part::Imm110)
            .or(Err(InstructionError::InvalidArgument(Part::Imm110)))?;

        let pc = self.cpu.register.get(Register::PC);
        let address =
            (self.cpu.register.get(rs1) + immediate) & 0b0111_1111_1111_1111_1111_1111_1111_1111;

        #[cfg(any(feature = "trace", feature = "debug"))]
        {
            let debug = vec![
                format!("\t\t- rsd: {:?}", rsd),
                format!("\t\t- rs1: {:?}", rs1),
                format!("\t\t-  pc: {}", debug::number(pc, 12)),
                format!("\t\t- imm: {}", debug::number(immediate, 12)),
                format!("\t\t- adr: {}", debug::number(address, 12)),
            ];
            #[cfg(feature = "trace")]
            eprintln!("{}", debug.join("\n"));
            #[cfg(feature = "debug")]
            self.debug.extend_from_slice(&debug);
        }

        if rsd != Register::X0 {
            self.cpu.register.set(rsd, pc + REGISTER_INCREMENT);
        }
        if address > REGISTER_INCREMENT {
            self.cpu
                .register
                .set(Register::PC, address - REGISTER_INCREMENT); // Because on Ok PC gets incremented
        }
        Ok(())
    }

    #[allow(clippy::identity_op)] // readability
    fn unconditional_jump(&mut self, i: Instruction) -> Result<(), InstructionError> {
        let rsd: Register = i
            .value(Part::Dest)
            .or(Err(InstructionError::InvalidArgument(Part::Dest)))?
            .try_into()?;
        let immediate = 0
            | (i.value(Part::B20j)
                .or(Err(InstructionError::InvalidArgument(Part::B20j)))?
                << 19)
            | (i.value(Part::Imm1912)
                .or(Err(InstructionError::InvalidArgument(Part::Imm1912)))?
                << 11)
            | (i.value(Part::B11j)
                .or(Err(InstructionError::InvalidArgument(Part::B11j)))?
                << 10)
            | (i.value(Part::Imm101)
                .or(Err(InstructionError::InvalidArgument(Part::Imm101)))?
                << 0);
        let pc = self.cpu.register.get(Register::PC);
        if rsd != Register::X0 {
            self.cpu.register.set(rsd, pc + REGISTER_INCREMENT);
        }

        #[cfg(any(feature = "trace", feature = "debug"))]
        {
            let debug = vec![
                format!("\t\t- rsd: {:?}", rsd),
                format!("\t\t- pc: {}", pc),
                format!(
                    "\t\t\t- b20b: {}",
                    debug::number(i.value(Part::B20j).unwrap(), 20)
                ),
                format!(
                    "\t\t\t- im19: {}",
                    debug::number(i.value(Part::Imm1912).unwrap(), 20)
                ),
                format!(
                    "\t\t\t- b11j: {}",
                    debug::number(i.value(Part::B11j).unwrap(), 20)
                ),
                format!(
                    "\t\t\t- im10: {}",
                    debug::number(i.value(Part::Imm101).unwrap(), 20)
                ),
                format!("\t\t- imm: {}", debug::number(immediate, 20)),
            ];
            #[cfg(feature = "trace")]
            eprintln!("{}", debug.join("\n"));
            #[cfg(feature = "debug")]
            self.debug.extend_from_slice(&debug);
        }

        let immediate = bitops::sign_extend(immediate, 20);
        #[cfg(any(feature = "trace", feature = "debug"))]
        {
            let debug = vec![format!("\t\t- sim: {}", debug::number(immediate, 20))];
            #[cfg(feature = "trace")]
            eprintln!("{}", debug.join("\n"));
            #[cfg(feature = "debug")]
            self.debug.extend_from_slice(&debug);
        }

        self.cpu.register.set(
            Register::PC,
            (immediate * 2) as u32 + pc - REGISTER_INCREMENT,
        ); // TODO: Why *2??
        Ok(())
    }

    fn immediate_math(&mut self, i: Instruction) -> Result<(), InstructionError> {
        let f3 = i
            .value(Part::Funct3)
            .or(Err(InstructionError::InvalidArgument(Part::Funct3)))?;

        match f3 {
            0b001 | 0b101 => self.immediate_math_shift(i),
            _ => self.immediate_math_normal(i),
        }
    }

    fn immediate_math_normal(&mut self, i: Instruction) -> Result<(), InstructionError> {
        let f3 = i
            .value(Part::Funct3)
            .or(Err(InstructionError::InvalidArgument(Part::Funct3)))?;
        let immediate = i
            .value(Part::Imm110)
            .or(Err(InstructionError::InvalidArgument(Part::Imm110)))?;
        let immediate = bitops::sign_extend(immediate, 12);
        let rs1: Register = i
            .value(Part::Reg1)
            .or(Err(InstructionError::InvalidArgument(Part::Reg1)))?
            .try_into()?;
        let rsd: Register = i
            .value(Part::Dest)
            .or(Err(InstructionError::InvalidArgument(Part::Dest)))?
            .try_into()?;

        #[cfg(any(feature = "trace", feature = "debug"))]
        {
            let debug = vec![
                format!("\t\t- rs1: {:?}", rs1),
                format!("\t\t- rsd: {:?}", rsd),
                format!("\t\t-  f3: {}", debug::number(f3, 3)),
                format!("\t\t- imm: {}", debug::number(immediate, 12)),
            ];
            #[cfg(feature = "trace")]
            eprintln!("{}", debug.join("\n"));
            #[cfg(feature = "debug")]
            self.debug.extend_from_slice(&debug);
        }

        match f3 {
            0b000 => {
                // ADDI
                self.cpu.register.set(
                    rsd,
                    (self.cpu.register.get(rs1) as i32 + bitops::sign_extend(immediate as u32, 12))
                        as u32,
                );
                Ok(())
            }
            0b010 => {
                // SLTI
                let a = self.cpu.register.get(rs1) as i32;
                let b = immediate;
                let cmp = if a < b { 1 } else { 0 };
                self.cpu.register.set(rsd, cmp);
                Ok(())
            }
            0b011 => {
                // SLTIU
                let a = self.cpu.register.get(rs1);
                let b = immediate as u32;
                let cmp = if immediate == 1 {
                    if a == 0 {
                        1
                    } else {
                        0
                    }
                } else if a < b {
                    1
                } else {
                    0
                };
                self.cpu.register.set(rsd, cmp);
                Ok(())
            }
            0b100 => {
                // XORI
                let reg = self.cpu.register.get(rs1);
                let result = if immediate == -1 {
                    !reg
                } else {
                    reg ^ immediate as u32
                };
                self.cpu.register.set(rsd, result);
                Ok(())
            }
            0b110 => {
                // ORI
                self.cpu.register.set(
                    rsd,
                    (self.cpu.register.get(rs1) as i32 | bitops::sign_extend(immediate as u32, 12))
                        as u32,
                );
                Ok(())
            }
            0b111 => {
                // XORI
                self.cpu.register.set(
                    rsd,
                    (self.cpu.register.get(rs1) as i32 & bitops::sign_extend(immediate as u32, 12))
                        as u32,
                );
                Ok(())
            }
            _ => Err(InstructionError::InvalidOperation(Operation::Branch)),
        }
    }

    fn immediate_math_shift(&mut self, i: Instruction) -> Result<(), InstructionError> {
        let f3 = i.value(Part::Funct3).unwrap();
        let raw_immediate = i
            .value(Part::Imm110)
            .or(Err(InstructionError::InvalidArgument(Part::Imm110)))?;

        let immediate = raw_immediate & 0b000_0000_0000_0000_0000_0000_0000_0001_1111;
        let shift = raw_immediate & 0b000_0000_0000_0000_0000_0000_1111_1110_0000;

        let rs1: Register = i
            .value(Part::Reg1)
            .or(Err(InstructionError::InvalidArgument(Part::Reg1)))?
            .try_into()?;
        let rsd: Register = i
            .value(Part::Dest)
            .or(Err(InstructionError::InvalidArgument(Part::Dest)))?
            .try_into()?;

        #[cfg(any(feature = "trace", feature = "debug"))]
        {
            let debug = vec![
                format!("\t\t- rs1: {:?}", rs1),
                format!("\t\t- rsd: {:?}", rsd),
                format!("\t\t-  f3: {}", debug::number(f3, 3)),
                format!("\t\t- rim: {}", debug::number(raw_immediate, 12)),
                format!("\t\t- imm: {}", debug::number(immediate, 12)),
                format!("\t\t- shf: {}", debug::number(shift, 12)),
            ];
            #[cfg(feature = "trace")]
            eprintln!("{}", debug.join("\n"));
            #[cfg(feature = "debug")]
            self.debug.extend_from_slice(&debug);
        }

        match (f3, shift) {
            (0b001, 0b0000000) => {
                // SLLI
                self.cpu
                    .register
                    .set(rsd, self.cpu.register.get(rs1) << immediate);
                Ok(())
            }
            (0b101, 0b0000000) => {
                // SRLI
                self.cpu
                    .register
                    .set(rsd, self.cpu.register.get(rs1) >> immediate);
                Ok(())
            }
            (0b101, 0b0100000) => {
                // SRAI: TODO: is this right?
                self.cpu
                    .register
                    .set(rsd, self.cpu.register.get(rs1).wrapping_shr(immediate));
                Ok(())
            }
            _ => Err(InstructionError::InvalidOperation(Operation::ImmediateMath)),
        }
    }

    fn register_math(&mut self, i: Instruction) -> Result<(), InstructionError> {
        let f3 = i
            .value(Part::Funct3)
            .or(Err(InstructionError::InvalidArgument(Part::Funct3)))?;
        let f7 = i
            .value(Part::Funct7)
            .or(Err(InstructionError::InvalidArgument(Part::Funct7)))?;
        let rs1: Register = i
            .value(Part::Reg1)
            .or(Err(InstructionError::InvalidArgument(Part::Reg1)))?
            .try_into()?;
        let rs2: Register = i
            .value(Part::Reg2)
            .or(Err(InstructionError::InvalidArgument(Part::Reg2)))?
            .try_into()?;
        let rsd: Register = i
            .value(Part::Dest)
            .or(Err(InstructionError::InvalidArgument(Part::Dest)))?
            .try_into()?;

        #[cfg(any(feature = "trace", feature = "debug"))]
        {
            let debug = vec![
                format!("\t\t- rs1: {:?}", rs1),
                format!("\t\t- rs2: {:?}", rs2),
                format!("\t\t- rsd: {:?}", rsd),
                format!("\t\t-  f3: {}", debug::number(f3, 3)),
                format!("\t\t-  f7: {}", debug::number(f7, 8)),
            ];
            #[cfg(feature = "trace")]
            eprintln!("{}", debug.join("\n"));
            #[cfg(feature = "debug")]
            self.debug.extend_from_slice(&debug);
        }

        match (f3, f7) {
            (0b000, 0b0000000) => {
                let lhs = self.cpu.register.get(rs1);
                let rhs = self.cpu.register.get(rs2);
                let result = lhs.overflowing_add(rhs).0 & 0x0000FFFF;
                self.cpu.register.set(rsd, result);
                Ok(())
            }
            (0b000, 0b0100000) => {
                // TODO: Overflows are ignored and the low XLEN bits of results are written to the destination rd
                self.cpu
                    .register
                    .set(rsd, self.cpu.register.get(rs1) - self.cpu.register.get(rs2));
                Ok(())
            }
            (0b010, 0b0000000) => {
                // SLT
                let a = self.cpu.register.get(rs1) as i32;
                let b = self.cpu.register.get(rs2) as i32;
                let cmp = if a < b { 1 } else { 0 };
                self.cpu.register.set(rsd, cmp);
                Ok(())
            }
            (0b011, 0b0000000) => {
                // SLTU
                let is_zero_register = Register::X0 == rs1;
                let a = self.cpu.register.get(rs1);
                let b = self.cpu.register.get(rs2);
                let cmp = if is_zero_register {
                    if b != 0 {
                        1
                    } else {
                        0
                    }
                } else if a < b {
                    1
                } else {
                    0
                };
                self.cpu.register.set(rsd, cmp);
                Ok(())
            }
            (0b001, 0b0000000) => {
                self.cpu.register.set(
                    rsd,
                    self.cpu.register.get(rs1)
                        << (self.cpu.register.get(rs2)
                            & 0b000_0000_0000_0000_0000_0000_0000_0001_1111),
                );
                Ok(())
            }
            (0b101, 0b0000000) => {
                // SRL - logical right shift
                self.cpu.register.set(
                    rsd,
                    self.cpu.register.get(rs1)
                        >> (self.cpu.register.get(rs2)
                            & 0b000_0000_0000_0000_0000_0000_0000_0001_1111),
                );
                Ok(())
            }
            (0b101, 0b0100000) => {
                // SRA - arithmetic right shift
                let a = self.cpu.register.get(rs1);
                let b = self.cpu.register.get(rs2) & 0b000_0000_0000_0000_0000_0000_0000_0001_1111;
                self.cpu.register.set(rsd, a.wrapping_shr(b)); // TODO: is this right?
                Ok(())
            }
            (0b100, 0b0000000) => {
                self.cpu
                    .register
                    .set(rsd, self.cpu.register.get(rs1) ^ self.cpu.register.get(rs2));
                Ok(())
            }
            (0b110, 0b0000000) => {
                self.cpu
                    .register
                    .set(rsd, self.cpu.register.get(rs1) | self.cpu.register.get(rs2));
                Ok(())
            }
            (0b111, 0b0000000) => {
                self.cpu
                    .register
                    .set(rsd, self.cpu.register.get(rs1) & self.cpu.register.get(rs2));
                Ok(())
            }
            _ => {
                #[cfg(feature = "trace")]
                {
                    eprintln!("doing register math {:#05b}, {:#09b}:", f3, f7);
                }
                Err(InstructionError::InvalidOperation(Operation::Math))
            }
        }
    }
}
