use brrrt::{risc32i::instr::instruction::Instruction, Cpu, Register};

// https://riscvasm.lucasteske.dev
fn from_asm() -> Vec<u32> {
    vec![
        0x00d00093, // addi x1, x0, 13
        0x00c08113, // addi x2, x1, 12
        0x00282023, // sw x2, 0(x16)
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

    for x in 0..100 {
        let pc = cpu.register.get(Register::PC);
        let code = cpu.rom.word_at(pc).expect("invalid memory access");
        eprintln!("iteration {} :: PC: {}", x, pc);

        let inst = Instruction::parse(code).expect("should parse");
        eprintln!("{x}: {:#034b}", code);
        eprintln!("\t{:?}", inst);

        if cpu.execute(inst).is_err() {
            eprintln!("Error!");
            break;
        }
        if (cpu.register.get(Register::PC) / 4) as usize == instructions.len() {
            break;
        }
    }

    eprintln!("-------------------------------------");
    eprintln!("X01: {} (expected 13)", cpu.register.get(Register::X1));
    eprintln!("X02: {} (expected 25)", cpu.register.get(Register::X2));
    eprintln!("X16: {} (expected 0)", cpu.register.get(Register::X16));
    eprintln!("M@0: {} (expected 25)", cpu.ram.byte_at(0).unwrap());

    Ok(())
}
