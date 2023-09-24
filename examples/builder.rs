use brrrt::{
    debug,
    risc32i::{
        instr::builder::Builder, instr::instruction::Instruction, instr::operation::*,
        instr::part::Part,
    },
    Cpu, Register,
};

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
    let instructions = from_builder();

    let mut cpu: Cpu = Default::default();
    for (n, x) in instructions.iter().enumerate() {
        eprintln!("{n}: {}", debug::binary(*x, 32));
        cpu.rom
            .set_word_at((n * 4) as u32, *x)
            .expect("invalid memory access");
    }

    eprintln!("-------------------------------------");

    for x in 0..100 {
        let pc = cpu.register.get(Register::PC);
        let code = cpu.rom.word_at(pc).expect("invalid memory access");
        eprintln!("iteration {} :: PC: {}", x, pc);

        let inst = Instruction::parse(code).expect("should parse");
        eprintln!("{x}: {}", debug::binary(code, 32));
        eprintln!("\t{:?}", inst);

        if cpu.execute(inst).is_err() {
            eprintln!("Error!");
            break;
        }
        if (cpu.register.get(Register::PC) / 4) as usize == instructions.len() {
            break;
        }
        eprintln!("");
    }

    eprintln!("-------------------------------------");
    eprintln!("X01: {} (expected 13)", cpu.register.get(Register::X1));
    eprintln!("X02: {} (expected 25)", cpu.register.get(Register::X2));
    eprintln!("X16: {} (expected 0)", cpu.register.get(Register::X16));
    eprintln!("M@0: {} (expected 25)", cpu.ram.byte_at(0).unwrap());

    Ok(())
}
