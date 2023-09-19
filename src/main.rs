mod risc32i;
use risc32i::{
    instr::builder::Builder, instr::format::Format, instr::operation::*, instr::part::Part, *,
};
mod memory;
use memory::Memory;

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

// https://riscvasm.lucasteske.dev
fn from_asm() -> Vec<u32> {
    vec![
        0x00d00093, // addi x1, x0, 13
        0x00c08113, // addi x2, x1, 12
        0x00282023, // sw x2, 0(x16)
    ]
}

fn from_builder() -> Vec<u32> {
    vec![
        // X1 = 13
        Builder::opcode(Operation::LUI)
            .pack(Part::Dest, Register::X1 as u32)
            .pack(Part::Imm3112, 13)
            .build(),
        // X2 = X1 + 12
        Builder::opcode(Operation::ImmediateMath)
            .pack(Part::Funct3, 0b000)
            .pack(Part::Imm110, 12)
            .pack(Part::Reg1, Register::X1 as u32)
            .pack(Part::Dest, Register::X2 as u32)
            .build(),
        // X2 => m@[X16]
        Builder::opcode(Operation::Store)
            .pack(Part::Imm110, 0)
            .pack(Part::Reg1, Register::X16 as u32)
            .pack(Part::Reg2, Register::X2 as u32)
            .build(),
    ]
}

fn main() -> Result<(), String> {
    let instructions = from_asm();

    let mut cpu: Cpu = Default::default();
    for (n, x) in instructions.iter().enumerate() {
        eprintln!("{n}: {:#034b}", x);
        cpu.rom
            .set_word_at((n * 4) as u32, *x)
            .expect("invalid memory access");
    }

    eprintln!("-------------------------------------");

    for x in 0..10 {
        let code = cpu
            .rom
            .word_at(cpu.register.get(Register::PC) * 4)
            .expect("invalid memory access");

        let inst = Instruction::parse(code).expect("should parse");
        eprintln!("{x}: {:#034b}", code);
        eprintln!("\t{:?}", inst);

        if cpu.execute(inst).is_err() {
            eprintln!("Error!");
            break;
        }
        if cpu.register.get(Register::PC) as usize == instructions.len() {
            break;
        }
    }

    eprintln!("-------------------------------------");
    eprintln!("X01: {} (expected 13)", cpu.register.get(Register::X1));
    eprintln!("X02: {} (expected 25)", cpu.register.get(Register::X2));
    eprintln!("X16: {} (expected 0)", cpu.register.get(Register::X6));
    eprintln!("M@0: {} (expected 25)", cpu.ram.byte_at(0).unwrap());

    Ok(())
}

#[derive(Default, Debug)]
struct Cpu {
    register: Registers,
    ram: Memory,
    rom: Memory,
}

impl Cpu {
    fn execute(&mut self, i: Instruction) -> Result<(), &'static str> {
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
            self.register.increment(Register::PC);
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
        let address = self.register.get(rs1) as i32 + sign_extend(immediate, 12);

        // eprintln!("rs1: {:?}", rs1);
        // eprintln!("rs2: {:?}", rs2);
        // eprintln!(" f3: {:#05b} ({})", f3, f3);
        // eprintln!("im1: {:#014b} ({})", im115, im115);
        // eprintln!("im4: {:#014b} ({})", im40, im40);
        // eprintln!("imm: {:#014b} ({})", immediate, immediate);
        // eprintln!("ext: {:#014b} ({})", sign_extend(immediate, 12), sign_extend(immediate, 12));
        // eprintln!("adr: {:#014b} ({})", address, address);

        match f3 {
            0b000 => {
                // SB
                self.ram
                    .set_byte_at(address as u32, self.register.get(rs2) as u8)
                    .expect("invalid memory access");
                Ok(())
            }
            0b001 => {
                // SH
                self.ram
                    .set_hw_at(address as u32, self.register.get(rs2) as u16)
                    .expect("invalid memory access");
                Ok(())
            }
            0b010 => {
                // SW
                self.ram
                    .set_word_at(address as u32, self.register.get(rs2))
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
        let address = self.register.get(rs1) as i32 + sign_extend(immediate, 12);

        // eprintln!("rsd: {:?}", rsd);
        // eprintln!("rs1: {:?}", rs1);
        // eprintln!(" f3: {:#05b} ({})", f3, f3);
        // eprintln!("imm: {:#014b} ({})", immediate, immediate);
        // eprintln!("adr: {:#014b} ({})", address, address);

        match f3 {
            0b000 => {
                // LB
                let value = self
                    .ram
                    .byte_at(address.try_into().expect("invalid address"))
                    .expect("invalid memory access");
                self.register.set(rsd, sign_extend(value as u32, 8) as u32);
                Ok(())
            }
            0b001 => {
                // LH
                let value = self
                    .ram
                    .hw_at(address.try_into().expect("invalid address"))
                    .expect("invalid memory access");
                self.register.set(rsd, sign_extend(value as u32, 16) as u32);
                Ok(())
            }
            0b010 => {
                // LW
                self.register.set(
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
                self.register.set(rsd, value as u32);
                Ok(())
            }
            0b101 => {
                // LHU
                let value = self
                    .ram
                    .hw_at(address.try_into().expect("invalid address"))
                    .expect("invalid memory access");
                self.register.set(rsd, value as u32);
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
        let pc = self.register.get(Register::PC);
        self.register.set(
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
        self.register
            .set(rsd, immediate & 0b0000_0000_0000_1111_1111_1111_1111_1111);
        Ok(())
    }

    fn branch(&mut self, i: Instruction) -> Result<(), &'static str> {
        let im41 = i.value(Part::Imm41).expect("invalid immediate 4:1")
            | i.value(Part::B11b).expect("invalid b11b");
        let im12 = i.value(Part::B12b).expect("invalid b12b")
            | i.value(Part::Imm105).expect("invalid immediate 10:5");
        let address = (im12 << 5) | im41; // TODO: is this right?
                                          // eprintln!("immediate 1: {:#034b} ({})", im41, im41);
                                          // eprintln!("immediate 2: {:#034b} ({})", im12, im12);
                                          // eprintln!("immediate R: {:#034b} ({})", address, address);
        if address % 2 != 0 {
            // TODO: is this right? The 12-bit B-immediate encodes
            // signed offsets in multiples of 2
            return Err("address not a multiple of 2");
        }
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
        let pc = self.register.get(Register::PC);

        match f3 {
            0b000 => {
                // BEQ
                if self.register.get(rs1) == self.register.get(rs2) {
                    self.register.set(Register::PC, pc + address);
                }
                Ok(())
            }
            0b001 => {
                // BNE
                if self.register.get(rs1) != self.register.get(rs2) {
                    self.register.set(Register::PC, pc + address);
                }
                Ok(())
            }
            0b100 => {
                // BLT
                if (self.register.get(rs1) as i32) < (self.register.get(rs2) as i32) {
                    self.register.set(Register::PC, pc + address);
                }
                Ok(())
            }
            0b110 => {
                // BLTU
                if self.register.get(rs1) < self.register.get(rs2) {
                    self.register.set(Register::PC, pc + address);
                }
                Ok(())
            }
            0b101 => {
                // BGE
                if (self.register.get(rs1) as i32) > (self.register.get(rs2) as i32) {
                    self.register.set(Register::PC, pc + address);
                }
                Ok(())
            }
            0b111 => {
                // BGEU
                if self.register.get(rs1) > self.register.get(rs2) {
                    self.register.set(Register::PC, pc + address);
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

        let pc = self.register.get(Register::PC);
        let address =
            (self.register.get(rs1) + immediate) & 0b0111_1111_1111_1111_1111_1111_1111_1111;
        self.register.set(rsd, pc + REGISTER_INCREMENT);
        self.register
            .set(Register::PC, address - REGISTER_INCREMENT); // Because on Ok PC gets incremented
        Ok(())
    }

    fn unconditional_jump(&mut self, i: Instruction) -> Result<(), &'static str> {
        let rsd: Register = i
            .value(Part::Dest)
            .expect("invalid destination")
            .try_into()
            .expect("invalid register");
        let immediate = i.value(Part::B20j).expect("invalid b20j")
            | i.value(Part::Imm101).expect("invalid immediate 10:1")
            | i.value(Part::B11j).expect("invalid b11j")
            | i.value(Part::Imm1912).expect("invalid immediate 10:1");
        // TODO: sign-extended?
        let pc = self.register.get(Register::PC);
        if rsd != Register::X0 {
            self.register.set(rsd, pc + REGISTER_INCREMENT);
        }
        self.register
            .set(Register::PC, pc + immediate - REGISTER_INCREMENT); // because Ok will increment PC
        Ok(())
    }

    fn immediate_math(&mut self, i: Instruction) -> Result<(), &'static str> {
        let f3 = i.value(Part::Funct3).unwrap();

        match f3 {
            0b001 | 0b101 => self.immediate_math_shift(i),
            _ => self.immediate_math_normal(i),
        }
    }

    fn immediate_math_normal(&mut self, i: Instruction) -> Result<(), &'static str> {
        let f3 = i.value(Part::Funct3).unwrap();
        let immediate = i.value(Part::Imm110).unwrap();
        let rs1: Register = i.value(Part::Reg1).unwrap().try_into().unwrap();
        let rsd: Register = i.value(Part::Dest).unwrap().try_into().unwrap();

        match f3 {
            0b000 => {
                // ADDI
                self.register.set(
                    rsd,
                    (self.register.get(rs1) as i32 + sign_extend(immediate, 12)) as u32,
                );
                Ok(())
            }
            0b010 => {
                // SLTI
                let a = self.register.get(rs1) as i32;
                let b = sign_extend(immediate, 12);
                let cmp = if a < b { 1 } else { 0 };
                self.register.set(rsd, cmp);
                Ok(())
            }
            0b011 => {
                // SLTIU
                let a = self.register.get(rs1);
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
                self.register.set(rsd, cmp);
                Ok(())
            }
            0b100 => {
                // XORI
                let reg = self.register.get(rs1);
                let simm = sign_extend(immediate, 12);
                let result = if simm == -1 { !reg } else { reg ^ immediate };
                self.register.set(rsd, result);
                Ok(())
            }
            0b110 => {
                // ORI
                self.register.set(
                    rsd,
                    (self.register.get(rs1) as i32 | sign_extend(immediate, 12)) as u32,
                );
                Ok(())
            }
            0b111 => {
                // XORI
                self.register.set(
                    rsd,
                    (self.register.get(rs1) as i32 & sign_extend(immediate, 12)) as u32,
                );
                Ok(())
            }
            _ => Err("invalid immediate math operation"),
        }
    }

    fn immediate_math_shift(&mut self, i: Instruction) -> Result<(), &'static str> {
        let f3 = i.value(Part::Funct3).unwrap();
        let raw_immediate = i.value(Part::Imm110).unwrap();

        let immediate = raw_immediate & 0b000_0000_0000_0000_0000_0000_0000_0001_1111;
        let shift = raw_immediate & 0b000_0000_0000_0000_0000_0000_1111_1110_0000;

        let rs1: Register = i.value(Part::Reg1).unwrap().try_into().unwrap();
        let rsd: Register = i.value(Part::Dest).unwrap().try_into().unwrap();

        match (f3, shift) {
            (0b001, 0b0000000) => {
                // SLLI
                self.register.set(rsd, self.register.get(rs1) << immediate);
                Ok(())
            }
            (0b101, 0b0000000) => {
                // SRLI
                self.register.set(rsd, self.register.get(rs1) >> immediate);
                Ok(())
            }
            (0b101, 0b0100000) => {
                // SRAI: TODO: is this right?
                self.register
                    .set(rsd, self.register.get(rs1).wrapping_shr(immediate));
                Ok(())
            }
            _ => Err("invalid immediate math shift operation"),
        }
    }

    fn register_math(&mut self, i: Instruction) -> Result<(), &'static str> {
        let f3 = i.value(Part::Funct3).unwrap();
        let f7 = i.value(Part::Funct7).unwrap();
        let rs1: Register = i.value(Part::Reg1).unwrap().try_into().unwrap();
        let rs2: Register = i.value(Part::Reg2).unwrap().try_into().unwrap();
        let rsd: Register = i.value(Part::Dest).unwrap().try_into().unwrap();
        eprintln!("values from {:?} <op> {:?} go to {:?}", rs1, rs2, rsd);

        match (f3, f7) {
            (0b000, 0b0000000) => {
                self.register
                    .set(rsd, self.register.get(rs1) + self.register.get(rs2));
                Ok(())
            }
            (0b000, 0b0100000) => {
                // TODO: Overflows are ignored and the low XLEN bits of results are written to the destination rd
                self.register
                    .set(rsd, self.register.get(rs1) - self.register.get(rs2));
                Ok(())
            }
            (0b010, 0b0000000) => {
                // SLT
                let a = self.register.get(rs1) as i32;
                let b = self.register.get(rs2) as i32;
                let cmp = if a < b { 1 } else { 0 };
                self.register.set(rsd, cmp);
                Ok(())
            }
            (0b011, 0b0000000) => {
                // SLTU
                let is_zero_register = Register::X0 == rs1;
                let a = self.register.get(rs1);
                let b = self.register.get(rs2);
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
                self.register.set(rsd, cmp);
                Ok(())
            }
            (0b001, 0b0000000) => {
                self.register.set(
                    rsd,
                    self.register.get(rs1)
                        << (self.register.get(rs2) & 0b000_0000_0000_0000_0000_0000_0000_0001_1111),
                );
                Ok(())
            }
            (0b101, 0b0000000) => {
                // SRL - logical right shift
                self.register.set(
                    rsd,
                    self.register.get(rs1)
                        >> (self.register.get(rs2) & 0b000_0000_0000_0000_0000_0000_0000_0001_1111),
                );
                Ok(())
            }
            (0b101, 0b0100000) => {
                // SRA - arithmetic right shift
                let a = self.register.get(rs1);
                let b = self.register.get(rs2) & 0b000_0000_0000_0000_0000_0000_0000_0001_1111;
                self.register.set(rsd, a.wrapping_shr(b)); // TODO: is this right?
                Ok(())
            }
            (0b100, 0b0000000) => {
                self.register
                    .set(rsd, self.register.get(rs1) ^ self.register.get(rs2));
                Ok(())
            }
            (0b110, 0b0000000) => {
                self.register
                    .set(rsd, self.register.get(rs1) | self.register.get(rs2));
                Ok(())
            }
            (0b111, 0b0000000) => {
                self.register
                    .set(rsd, self.register.get(rs1) & self.register.get(rs2));
                Ok(())
            }
            _ => {
                eprintln!("doing register math {:#05b}, {:#09b}:", f3, f7);
                Err("unknown r2r operation")
            }
        }
    }
}

const REGISTER_INCREMENT: u32 = 1; // TODO: or 4?

#[derive(Debug)]
struct Registers {
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
    fn set(&mut self, key: Register, value: u32) {
        self.data[key as usize] = value;
    }

    fn get(&self, key: Register) -> u32 {
        self.data[key as usize]
    }

    fn increment(&mut self, key: Register) {
        self.data[key as usize] += REGISTER_INCREMENT; // TODO: 4 vs 1? For PC only?
    }
}

#[repr(u32)]
#[derive(Debug, PartialEq, Copy, Clone)]
#[allow(dead_code)]
enum Register {
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

fn sign_extend(v: u32, width: u32) -> i32 {
    assert!(width < 32);
    let base: i32 = 2;
    let mut res = v as i32;
    if v as i32 > base.pow(width - 1) {
        res = v as i32 - base.pow(width);
    }
    res
}

#[cfg(test)]
mod test_sign_extend {
    use super::*;

    #[test]
    fn extend_neg_one() {
        let neg = -1;
        assert_eq!(sign_extend(neg as u32, 8), neg);
        assert_eq!(sign_extend(neg as u32, 12), neg);
        assert_eq!(sign_extend(neg as u32, 16), neg);

        assert_eq!(sign_extend(neg as u32, 16) as i8, neg as i8);

        let neg = -13;
        assert_eq!(sign_extend(neg as u32, 8), neg);
        assert_eq!(sign_extend(neg as u32, 12), neg);
        assert_eq!(sign_extend(neg as u32, 16), neg);

        assert_eq!(sign_extend(neg as u32, 16) as i8, neg as i8);

        let neg = -161;
        assert_eq!(sign_extend(neg as u32, 8), neg);
        assert_eq!(sign_extend(neg as u32, 12), neg);
        assert_eq!(sign_extend(neg as u32, 16), neg);

        assert_eq!(sign_extend(neg as u32, 16) as i16, neg as i16);

        let neg = -1312;
        assert_eq!(sign_extend(neg as u32, 8), neg);
        assert_eq!(sign_extend(neg as u32, 12), neg);
        assert_eq!(sign_extend(neg as u32, 16), neg);

        assert_eq!(sign_extend(neg as u32, 16) as i16, neg as i16);
    }
}

fn first_lsb_set(source: u32) -> i32 {
    for x in 0..32 {
        if source & (1 << x) > 0 {
            return x;
        }
    }

    return -1;
}

#[cfg(test)]
mod test_first_lsb_set {
    use super::*;

    #[test]
    fn lsb() {
        let test = 0b0000_0000_0000_0000_0000_0000_0000_0000;
        let result = first_lsb_set(test);
        assert_eq!(result, -1);

        let test = 0b0000_0000_0000_0000_0000_0000_0000_0001;
        let result = first_lsb_set(test);
        assert_eq!(result, 0);

        let test = 0b0000_0000_0000_0000_0000_0000_0000_0010;
        let result = first_lsb_set(test);
        assert_eq!(result, 1);

        let test = 0b0000_0000_0000_0000_0000_0000_0000_0100;
        let result = first_lsb_set(test);
        assert_eq!(result, 2);

        let test = 0b0000_0000_0000_0000_0000_0000_0000_1000;
        let result = first_lsb_set(test);
        assert_eq!(result, 3);

        let test = 0b1000_0000_0000_0000_0000_0000_0000_1000;
        let result = first_lsb_set(test);
        assert_eq!(result, 3);
    }
}
