mod risc32i;
use risc32i::{
    instr::builder::Builder, instr::format::Format, instr::operation::*, instr::part::Part, *,
};

// tests
mod immediate_math;
mod math;

fn main() -> Result<(), String> {
    let i = Instruction::parse(
        Builder::opcode(Operation::ImmediateMath)
            .pack(Part::Funct3, 0b101)
            .pack(Part::Reg1, Register::X12 as u32)
            .pack(Part::Imm110, 2)
            .pack(Part::Dest, Register::X16 as u32)
            .build(),
    )?;
    let num = i.get(Part::Reg1).unwrap();
    let value = i.value(Part::Reg1).unwrap();
    eprintln!("{:?}, reg1 value: {}, reg1 num: {}", i, value, num);
    eprintln!("{:?}", Part::Null);
    eprintln!("{:?}", Format::Jump);

    eprintln!("Register X21:{:?}", Register::X21 as u32);

    let mut cpu: Cpu = Default::default();
    cpu.register.set(Register::X12, 4);
    cpu.execute(i)?;
    eprintln!("Result: {:?}", cpu.register.get(Register::X16));
    eprintln!("{:?}", cpu);

    Ok(())
}

#[derive(Default, Debug)]
struct Cpu {
    register: Registers,
}

impl Cpu {
    fn execute(&mut self, i: Instruction) -> Result<(), &'static str> {
        let result = match i.opcode {
            Operation::Math => self.register_math(i),
            Operation::ImmediateMath => self.immediate_math(i),
            _ => Err("unknown opcode"),
        };
        if result.is_ok() {
            self.register
                .set(Register::PC, self.register.get(Register::PC) + 1);
        }
        result
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
                self.register.set(rsd, self.register.get(rs1) + immediate);
                Ok(())
            }
            0b010 => {
                // SLTI
                let a = self.register.get(rs1) as i32;
                let b = immediate as i32;
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
                let reg = self.register.get(rs1);
                let simm = 0xFFF - immediate as i32 - 1; // TODO: wat!
                let result = if simm == -1 { !reg } else { reg ^ immediate };
                self.register.set(rsd, result);
                Ok(())
            }
            0b110 => {
                self.register.set(rsd, self.register.get(rs1) | immediate);
                Ok(())
            }
            0b111 => {
                self.register.set(rsd, self.register.get(rs1) & immediate);
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
}

#[repr(u32)]
#[derive(Debug, PartialEq)]
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
