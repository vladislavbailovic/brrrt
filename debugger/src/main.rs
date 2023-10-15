use brrrt_vm::{debug, Program, Register, VM};
use std::{fs, io};

fn load_program(path: &str) -> Program {
    let mut prg: Program = Default::default();
    let src = fs::read(path)
        .expect("Unable to read file")
        .into_iter()
        .enumerate();
    for (i, x) in src {
        prg.write(i as u32, x);
    }
    prg
}

fn main() -> Result<(), String> {
    let mut vm: VM = Default::default();
    let program = load_program("asm/jump.bin");

    let registers = &[Register::X0, Register::X1, Register::X2, Register::X3];

    let mut render_memory = true;
    loop {
        let instr = program.peek(&vm)?;
        eprintln!(
            "PC: {}",
            debug::number(vm.cpu.register.get(Register::PC), 32)
        );
        for reg in registers {
            eprintln!(
                "{:?}: {}",
                reg,
                debug::number(vm.cpu.register.get(*reg), 32)
            );
        }
        eprintln!("Next: {:?}", instr);
        eprintln!("Raw: {}", debug::number(instr.raw, 32));
        program.step(&mut vm, 0)?;

        if program.is_done(&vm) {
            break;
        }

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            continue;
        }
        match input.chars().next() {
            Some('s') | Some('n') => continue,
            Some('q') => {
                render_memory = false;
                break;
            }
            _ => continue,
        }
    }

    if render_memory {
        eprintln!();
        for pos in 0..24 {
            if pos > 0 && pos % 4 == 0 {
                eprintln!();
            }
            eprint!(
                "{:02}: {: <18}",
                pos,
                debug::number(vm.ram.byte_at(pos)? as u32, 8)
            );
        }
        eprintln!();
    }

    Ok(())
}
