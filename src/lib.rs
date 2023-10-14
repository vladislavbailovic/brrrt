pub mod bitops;
pub mod debug;
pub mod memory;
pub mod risc32i;

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

pub use memory::Memory;
use risc32i::{instr::instruction::Instruction, instr::operation::Operation, instr::part::Part};

#[derive(Default)]
pub struct Program {
    end: usize,
    rom: Memory,
}

impl Program {
    pub fn from_asm(asm: &[u32]) -> Self {
        let mut prg: Self = Default::default();
        for (n, x) in asm.iter().enumerate() {
            eprintln!("{n}: {}", debug::binary(*x, 32));
            prg.rom
                .set_word_at((n * 4) as u32, *x)
                .expect("invalid memory access");
        }
        prg.end = asm.len();
        prg
    }

    pub fn run(&self, vm: &mut VM) -> Result<(), String> {
        for x in 0..100 {
            let pc = vm.cpu.register.get(Register::PC);
            let code = self.rom.word_at(pc).expect("invalid memory access");
            eprintln!("iteration {} :: PC: {}", x, pc);

            let inst = Instruction::parse(code).expect("should parse");
            eprintln!("{x}: {}", debug::binary(code, 32));
            eprintln!("\t{:?}", inst);

            vm.execute(inst)?;
            if (vm.cpu.register.get(Register::PC) / 4) as usize == self.end {
                break;
            }
            eprintln!("");
        }
        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct CPU {
    pub register: Registers,
}

impl CPU {
    fn increment_pc(&mut self) {
        self.register.set(
            Register::PC,
            self.register.get(Register::PC) + REGISTER_INCREMENT,
        );
    }
}

#[derive(Default, Debug)]
pub struct VM {
    pub cpu: CPU,
    pub ram: Memory,
}

impl VM {
    pub fn execute(&mut self, i: Instruction) -> Result<(), &'static str> {
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
            _ => Err("unknown opcode"),
        };
        if result.is_ok() {
            self.cpu.increment_pc();
        }
        result
    }

    fn store(&mut self, i: Instruction) -> Result<(), &'static str> {
        let rs1: Register = i
            .value(Part::Reg1)
            .expect("invalid reg1")
            .try_into()
            .expect("invalid register");
        let rs2: Register = i
            .value(Part::Reg2)
            .expect("invalid reg2")
            .try_into()
            .expect("invalid register");
        let f3 = i.value(Part::Funct3).expect("invalid funct3");
        let im40 = i.value(Part::Imm40).expect("invalid imm40");
        let im115 = i.value(Part::Imm115).expect("invalid imm115");
        let immediate = (im115 << 5) | im40; // https://stackoverflow.com/a/60239441
        let address = self.cpu.register.get(rs1) as i32 + bitops::sign_extend(immediate, 12);

        eprintln!("\t\trs1: {:?}", rs1);
        eprintln!("\t\trs2: {:?}", rs2);
        eprintln!("\t\t f3: {}", debug::number(f3, 5));
        eprintln!("\t\tim1: {}", debug::number(im115, 12));
        eprintln!("\t\tim4: {}", debug::number(im40, 12));
        eprintln!("\t\timm: {}", debug::number(immediate, 12));
        eprintln!(
            "\t\text: {}",
            debug::number(bitops::sign_extend(immediate, 12), 12)
        );
        eprintln!("\t\tadr: {}", debug::number(address, 12));

        match f3 {
            0b000 => {
                // SB
                self.ram
                    .set_byte_at(address as u32, self.cpu.register.get(rs2) as u8)
                    .expect("invalid memory access");
                Ok(())
            }
            0b001 => {
                // SH
                self.ram
                    .set_hw_at(address as u32, self.cpu.register.get(rs2) as u16)
                    .expect("invalid memory access");
                Ok(())
            }
            0b010 => {
                // SW
                self.ram
                    .set_word_at(address as u32, self.cpu.register.get(rs2))
                    .expect("invalid memory access");
                Ok(())
            }
            _ => Err("invalid store instruction"),
        }
    }

    fn load(&mut self, i: Instruction) -> Result<(), &'static str> {
        let rsd: Register = i
            .value(Part::Dest)
            .expect("invalid dest")
            .try_into()
            .expect("invalid register");
        let rs1: Register = i
            .value(Part::Reg1)
            .expect("invalid reg1")
            .try_into()
            .expect("invalid register");
        let f3 = i.value(Part::Funct3).expect("invalid funct3");
        let immediate = i.value(Part::Imm110).expect("invalid imm110");
        let address = self.cpu.register.get(rs1) as i32 + bitops::sign_extend(immediate, 12);

        eprintln!("\t\trsd: {:?}", rsd);
        eprintln!("\t\trs1: {:?}", rs1);
        eprintln!("\t\t f3: {}", debug::number(f3, 3));
        eprintln!("\t\timm: {}", debug::number(immediate, 12));
        eprintln!("\t\tadr: {}", debug::number(address, 12));

        match f3 {
            0b000 => {
                // LB
                let value = self
                    .ram
                    .byte_at(address.try_into().expect("invalid address"))
                    .expect("invalid memory access");
                self.cpu
                    .register
                    .set(rsd, bitops::sign_extend(value as u32, 8) as u32);
                Ok(())
            }
            0b001 => {
                // LH
                let value = self
                    .ram
                    .hw_at(address.try_into().expect("invalid address"))
                    .expect("invalid memory access");
                self.cpu
                    .register
                    .set(rsd, bitops::sign_extend(value as u32, 16) as u32);
                Ok(())
            }
            0b010 => {
                // LW
                self.cpu.register.set(
                    rsd,
                    self.ram
                        .word_at(address.try_into().expect("invalid address"))
                        .expect("invalid memory access"),
                );
                Ok(())
            }
            0b100 => {
                // LBU
                let value = self
                    .ram
                    .byte_at(address.try_into().expect("invalid address"))
                    .expect("invalid memory access");
                self.cpu.register.set(rsd, value as u32);
                Ok(())
            }
            0b101 => {
                // LHU
                let value = self
                    .ram
                    .hw_at(address.try_into().expect("invalid address"))
                    .expect("invalid memory access");
                self.cpu.register.set(rsd, value as u32);
                Ok(())
            }
            _ => Err("unknown load instruction"),
        }
    }

    fn add_upper_immediate(&mut self, i: Instruction) -> Result<(), &'static str> {
        let rsd: Register = i
            .value(Part::Dest)
            .expect("invalid dest")
            .try_into()
            .expect("invalid register");
        let immediate = i.value(Part::Imm3112).expect("invalid immediate 31:12");
        let pc = self.cpu.register.get(Register::PC);

        eprintln!("\t\trsd: {:?}", rsd);
        eprintln!("\t\timm: {}", debug::number(immediate, 20));
        eprintln!("\t\t pc: {}", pc);

        self.cpu.register.set(
            rsd,
            (immediate & 0b0000_0000_0000_1111_1111_1111_1111_1111) + pc,
        );
        Ok(())
    }

    fn load_upper_immediate(&mut self, i: Instruction) -> Result<(), &'static str> {
        let rsd: Register = i
            .value(Part::Dest)
            .expect("invalid dest")
            .try_into()
            .expect("invalid register");
        let immediate = i.value(Part::Imm3112).expect("invalid immediate 31:12");

        eprintln!("\t\trsd: {:?}", rsd);
        eprintln!("\t\timm: {}", debug::number(immediate, 20));

        self.cpu
            .register
            .set(rsd, immediate & 0b0000_0000_0000_1111_1111_1111_1111_1111);
        Ok(())
    }

    fn branch(&mut self, i: Instruction) -> Result<(), &'static str> {
        let rs1: Register = i
            .value(Part::Reg1)
            .expect("invalid reg1")
            .try_into()
            .expect("invalid register");
        let rs2: Register = i
            .value(Part::Reg2)
            .expect("invalid reg2")
            .try_into()
            .expect("invalid register");
        let f3 = i.value(Part::Funct3).expect("invalid funct3");
        let pc = (self.cpu.register.get(Register::PC) as i32) - REGISTER_INCREMENT as i32;

        eprintln!("\t\t- rs1: {:?}", rs1);
        eprintln!("\t\t- rs2: {:?}", rs2);
        eprintln!("\t\t-  pc: {}", pc);
        eprintln!("\t\t-  f3: {}", debug::number(f3, 3));

        let immediate = 0
            | (i.value(Part::B12b).expect("invalid B12b") << 11)
            | (i.value(Part::B11b).expect("invalid B11b") << 10)
            | (i.value(Part::Imm105).expect("invalid Imm105") << 4)
            | (i.value(Part::Imm41).expect("invalid Imm41") << 0);
        eprintln!(
            "\t\t\t- b12b: {}",
            debug::number(i.value(Part::B12b).unwrap(), 12)
        );
        eprintln!(
            "\t\t\t- b11b: {}",
            debug::number(i.value(Part::B11b).unwrap(), 12)
        );
        eprintln!(
            "\t\t\t- imm1: {}",
            debug::number(i.value(Part::Imm105).unwrap(), 12)
        );
        eprintln!(
            "\t\t\t- imm4: {}",
            debug::number(i.value(Part::Imm41).unwrap(), 12)
        );
        eprintln!("\t\t- rim: {}", debug::number(immediate, 12));

        let immediate = (immediate >> 1) << 1;
        let address = bitops::sign_extend(immediate, 12) * 2;
        eprintln!("\t\t- imm: {}", debug::number(immediate, 12));
        eprintln!("\t\t- adr: {}", debug::number(address, 12));

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
            _ => Err("invalid branch"),
        }
    }

    fn unconditional_register_jump(&mut self, i: Instruction) -> Result<(), &'static str> {
        let rsd: Register = i
            .value(Part::Dest)
            .expect("invalid destination")
            .try_into()
            .expect("invalid register");
        let rs1: Register = i
            .value(Part::Reg1)
            .expect("invalid reg1")
            .try_into()
            .expect("invalid register");
        let immediate = i.value(Part::Imm110).expect("invalid immediate value 11:0");

        let pc = self.cpu.register.get(Register::PC);
        let address =
            (self.cpu.register.get(rs1) + immediate) & 0b0111_1111_1111_1111_1111_1111_1111_1111;

        eprintln!("\t\t- rsd: {:?}", rsd);
        eprintln!("\t\t- rs1: {:?}", rs1);
        eprintln!("\t\t-  pc: {}", debug::number(pc, 12));
        eprintln!("\t\t- imm: {}", debug::number(immediate, 12));
        eprintln!("\t\t- adr: {}", debug::number(address, 12));

        self.cpu.register.set(rsd, pc + REGISTER_INCREMENT);
        self.cpu
            .register
            .set(Register::PC, address - REGISTER_INCREMENT); // Because on Ok PC gets incremented
        Ok(())
    }

    fn unconditional_jump(&mut self, i: Instruction) -> Result<(), &'static str> {
        let rsd: Register = i
            .value(Part::Dest)
            .expect("invalid destination")
            .try_into()
            .expect("invalid register");
        let immediate = 0
            | (i.value(Part::B20j).expect("invalid b20j") << 19)
            | (i.value(Part::Imm1912).expect("invalid immediate 1912") << 11)
            | (i.value(Part::B11j).expect("invalid b11j") << 10)
            | (i.value(Part::Imm101).expect("invalid immediate 10:1") << 0);
        let pc = self.cpu.register.get(Register::PC);
        if rsd != Register::X0 {
            self.cpu.register.set(rsd, pc + REGISTER_INCREMENT);
        }

        eprintln!("\t\t- rsd: {:?}", rsd);
        eprintln!("\t\t- pc: {}", pc);
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

        let immediate = bitops::sign_extend(immediate, 20);
        eprintln!("\t\t- sim: {}", debug::number(immediate, 20));

        self.cpu
            .register
            .set(Register::PC, (pc as i32 + immediate) as u32);
        Ok(())
    }

    fn immediate_math(&mut self, i: Instruction) -> Result<(), &'static str> {
        let f3 = i.value(Part::Funct3).expect("invalid f3");

        match f3 {
            0b001 | 0b101 => self.immediate_math_shift(i),
            _ => self.immediate_math_normal(i),
        }
    }

    fn immediate_math_normal(&mut self, i: Instruction) -> Result<(), &'static str> {
        let f3 = i.value(Part::Funct3).expect("invalid f3");
        let immediate = i.value(Part::Imm110).expect("invalid imm110");
        let rs1: Register = i
            .value(Part::Reg1)
            .expect("invalid reg1")
            .try_into()
            .expect("invalid register");
        let rsd: Register = i
            .value(Part::Dest)
            .expect("invalid dest")
            .try_into()
            .expect("invalid register");

        eprintln!("\t\t- rs1: {:?}", rs1);
        eprintln!("\t\t- rsd: {:?}", rsd);
        eprintln!("\t\t-  f3: {}", debug::number(f3, 3));
        eprintln!("\t\t- imm: {}", debug::number(immediate, 12));

        match f3 {
            0b000 => {
                // ADDI
                self.cpu.register.set(
                    rsd,
                    (self.cpu.register.get(rs1) as i32 + bitops::sign_extend(immediate, 12)) as u32,
                );
                Ok(())
            }
            0b010 => {
                // SLTI
                let a = self.cpu.register.get(rs1) as i32;
                let b = bitops::sign_extend(immediate, 12);
                let cmp = if a < b { 1 } else { 0 };
                self.cpu.register.set(rsd, cmp);
                Ok(())
            }
            0b011 => {
                // SLTIU
                let a = self.cpu.register.get(rs1);
                let b = immediate;
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
                let simm = bitops::sign_extend(immediate, 12);
                let result = if simm == -1 { !reg } else { reg ^ immediate };
                self.cpu.register.set(rsd, result);
                Ok(())
            }
            0b110 => {
                // ORI
                self.cpu.register.set(
                    rsd,
                    (self.cpu.register.get(rs1) as i32 | bitops::sign_extend(immediate, 12)) as u32,
                );
                Ok(())
            }
            0b111 => {
                // XORI
                self.cpu.register.set(
                    rsd,
                    (self.cpu.register.get(rs1) as i32 & bitops::sign_extend(immediate, 12)) as u32,
                );
                Ok(())
            }
            _ => Err("invalid immediate math operation"),
        }
    }

    fn immediate_math_shift(&mut self, i: Instruction) -> Result<(), &'static str> {
        let f3 = i.value(Part::Funct3).unwrap();
        let raw_immediate = i.value(Part::Imm110).expect("invalid imm110");

        let immediate = raw_immediate & 0b000_0000_0000_0000_0000_0000_0000_0001_1111;
        let shift = raw_immediate & 0b000_0000_0000_0000_0000_0000_1111_1110_0000;

        let rs1: Register = i
            .value(Part::Reg1)
            .expect("invalid reg1")
            .try_into()
            .expect("invalid register");
        let rsd: Register = i
            .value(Part::Dest)
            .expect("invalid dest")
            .try_into()
            .expect("invalid register");

        eprintln!("\t\t- rs1: {:?}", rs1);
        eprintln!("\t\t- rsd: {:?}", rsd);
        eprintln!("\t\t-  f3: {}", debug::number(f3, 3));
        eprintln!("\t\t- rim: {}", debug::number(raw_immediate, 12));
        eprintln!("\t\t- imm: {}", debug::number(immediate, 12));
        eprintln!("\t\t- shf: {}", debug::number(shift, 12));

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
            _ => Err("invalid immediate math shift operation"),
        }
    }

    fn register_math(&mut self, i: Instruction) -> Result<(), &'static str> {
        let f3 = i.value(Part::Funct3).expect("invalid funct3");
        let f7 = i.value(Part::Funct7).expect("invalid funct7");
        let rs1: Register = i
            .value(Part::Reg1)
            .expect("invalid reg1")
            .try_into()
            .expect("invalid register");
        let rs2: Register = i
            .value(Part::Reg2)
            .expect("invalid reg2")
            .try_into()
            .expect("invalid register");
        let rsd: Register = i
            .value(Part::Dest)
            .expect("invalid dest")
            .try_into()
            .expect("invalid register");

        eprintln!("\t\t- rs1: {:?}", rs1);
        eprintln!("\t\t- rs2: {:?}", rs2);
        eprintln!("\t\t- rsd: {:?}", rsd);
        eprintln!("\t\t-  f3: {}", debug::number(f3, 3));
        eprintln!("\t\t-  f7: {}", debug::number(f7, 8));

        match (f3, f7) {
            (0b000, 0b0000000) => {
                self.cpu
                    .register
                    .set(rsd, self.cpu.register.get(rs1) + self.cpu.register.get(rs2));
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
                eprintln!("doing register math {:#05b}, {:#09b}:", f3, f7);
                Err("unknown r2r operation")
            }
        }
    }
}

const REGISTER_INCREMENT: u32 = 4;

#[derive(Debug)]
pub struct Registers {
    data: [u32; 33],
}

impl Default for Registers {
    fn default() -> Self {
        Self {
            data: [(); 33].map(|_| 0),
        }
    }
}

impl Registers {
    pub fn set(&mut self, key: Register, value: u32) {
        self.data[key as usize] = value;
    }

    pub fn get(&self, key: Register) -> u32 {
        self.data[key as usize]
    }
}

#[repr(u32)]
#[derive(Debug, PartialEq, Copy, Clone)]
#[allow(dead_code)]
pub enum Register {
    X0,
    X1, // return address of a call
    X2, // stack pointer
    X3,
    X4,
    X5, // available as an alternate link register
    X6,
    X7,
    X8,
    X9,
    X10,
    X11,
    X12,
    X13,
    X14,
    X15,
    X16,
    X17,
    X18,
    X19,
    X20,
    X21,
    X22,
    X23,
    X24,
    X25,
    X26,
    X27,
    X28,
    X29,
    X30,
    X31,

    PC,
}

impl TryFrom<u32> for Register {
    type Error = &'static str;

    fn try_from(raw: u32) -> Result<Self, Self::Error> {
        match raw {
            x if x == Self::X0 as u32 => Ok(Self::X0),
            x if x == Self::X1 as u32 => Ok(Self::X1),
            x if x == Self::X2 as u32 => Ok(Self::X2),
            x if x == Self::X3 as u32 => Ok(Self::X3),
            x if x == Self::X4 as u32 => Ok(Self::X4),
            x if x == Self::X5 as u32 => Ok(Self::X5),
            x if x == Self::X6 as u32 => Ok(Self::X6),
            x if x == Self::X7 as u32 => Ok(Self::X7),
            x if x == Self::X8 as u32 => Ok(Self::X8),
            x if x == Self::X9 as u32 => Ok(Self::X9),
            x if x == Self::X10 as u32 => Ok(Self::X10),
            x if x == Self::X11 as u32 => Ok(Self::X11),
            x if x == Self::X12 as u32 => Ok(Self::X12),
            x if x == Self::X13 as u32 => Ok(Self::X13),
            x if x == Self::X14 as u32 => Ok(Self::X14),
            x if x == Self::X15 as u32 => Ok(Self::X15),
            x if x == Self::X16 as u32 => Ok(Self::X16),
            x if x == Self::X17 as u32 => Ok(Self::X17),
            x if x == Self::X18 as u32 => Ok(Self::X18),
            x if x == Self::X19 as u32 => Ok(Self::X19),
            x if x == Self::X20 as u32 => Ok(Self::X20),
            x if x == Self::X21 as u32 => Ok(Self::X21),
            x if x == Self::X22 as u32 => Ok(Self::X22),
            x if x == Self::X23 as u32 => Ok(Self::X23),
            x if x == Self::X24 as u32 => Ok(Self::X24),
            x if x == Self::X25 as u32 => Ok(Self::X25),
            x if x == Self::X26 as u32 => Ok(Self::X26),
            x if x == Self::X27 as u32 => Ok(Self::X27),
            x if x == Self::X28 as u32 => Ok(Self::X28),
            x if x == Self::X29 as u32 => Ok(Self::X29),
            x if x == Self::X30 as u32 => Ok(Self::X30),

            x if x == Self::PC as u32 => Ok(Self::PC),

            _ => Err("unknown register"),
        }
    }
}
