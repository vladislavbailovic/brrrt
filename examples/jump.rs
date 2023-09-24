use brrrt::{debug, risc32i::instr::instruction::Instruction, Cpu, Register};

// https://riscvasm.lucasteske.dev
fn from_asm() -> Vec<u32> {
    /*
    addi x1, x0, 13
    addi x2, x0, 12
    j end
    addi x2, x0, 161
    end: sw x2, 0(x16)
        */
    vec![0x00d00093, 0x00c00113, 0x0080006f, 0x0a100113, 0x00282023]
}

fn main() -> Result<(), String> {
    let instructions = from_asm();

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
    eprintln!("X02: {} (expected 12)", cpu.register.get(Register::X2));
    eprintln!("X16: {} (expected 0)", cpu.register.get(Register::X16));
    eprintln!("M@0: {} (expected 12)", cpu.ram.byte_at(0).unwrap());

    Ok(())
}
